use crate::prelude::*;
use crate::task::task_service;
use crate::terminal::terminal_contracts::{TerminalFilterContract, TerminalInfoContract, TerminalResizeContract, TerminalSpawnContract};
use crate::terminal::terminal_enums::{TerminalHistoryPersistence, TerminalShellStatus};
use crate::terminal::terminal_events::{
	TerminalCreatedEvent, TerminalDeletedEvent, TerminalShellKilledEvent, TerminalShellReadEvent, TerminalShellReadEventData, TerminalShellSpawnedEvent,
};
use crate::terminal::terminal_models::{TerminalBehaviorModel, TerminalFilterModel, TerminalMetaModel, TerminalModel, TerminalShellModel};
use crate::terminal::terminal_repository;
use portable_pty::{native_pty_system, CommandBuilder, PtySize};
use std::io::{Read, Write};
use tauri::AppHandle;
use tauri_specta::Event;
use tokio::sync::{oneshot, Mutex, RwLock};
use tokio::time::{sleep, Duration};
use uuid::Uuid;

pub async fn spawn_blocking(app_handle: &AppHandle, spawn_contract: TerminalSpawnContract) -> Result<()> {
	let session_id = build_and_spawn(app_handle, spawn_contract).await?;

	let _ = start_handle_threads(app_handle, &session_id).await;

	Ok(())
}

pub async fn spawn(app_handle: &AppHandle, spawn_contract: TerminalSpawnContract) -> Result<Uuid> {
	let session_id = build_and_spawn(app_handle, spawn_contract).await?;
	let app_handle_clone = app_handle.clone();

	tauri::async_runtime::spawn(async move { start_handle_threads(&app_handle_clone, &session_id).await });

	Ok(session_id)
}

async fn build_and_spawn(app_handle: &AppHandle, spawn_contract: TerminalSpawnContract) -> Result<Uuid> {
	let id = Uuid::new_v4();
	let meta = build_meta_model(&spawn_contract);
	let behavior = build_behavior_model(&spawn_contract);
	let shell = build_shell_model(&spawn_contract)?;

	let session = TerminalModel {
		id,
		concurrency_guard: Mutex::new(()),
		history: RwLock::new(String::new()),
		behavior: RwLock::new(behavior),
		meta: RwLock::new(meta),
		shell: RwLock::new(shell),
	};

	terminal_repository::create_one(session).await?;

	TerminalCreatedEvent(id).emit(app_handle).map_err(|_| Error::Failed)?;

	Ok(id)
}

fn build_command(spawn_contract: &TerminalSpawnContract) -> CommandBuilder {
	let mut cmd = CommandBuilder::new("pwsh.exe");

	if let Some(working_dir) = &spawn_contract.working_dir {
		cmd.args(["-WorkingDirectory", working_dir.as_str()]);
	}

	if spawn_contract.no_exit {
		cmd.args(["-NoExit"]);
	}

	if let Some(command) = &spawn_contract.command {
		cmd.args(["-Command", command.as_str()]);
	}

	cmd
}

fn build_shell_model(spawn_contract: &TerminalSpawnContract) -> Result<TerminalShellModel> {
	let pty_system = native_pty_system();
	let pair = pty_system.openpty(PtySize::default()).map_err(|_| Error::Failed)?;

	let writer = pair.master.take_writer().map_err(|_| Error::Failed)?;
	let reader = pair.master.try_clone_reader().map_err(|_| Error::Failed)?;

	let cmd = build_command(&spawn_contract);

	let child = pair.slave.spawn_command(cmd).map_err(|_| Error::Failed)?;
	let child_killer = child.clone_killer();

	let shell_model = TerminalShellModel {
		status: Mutex::new(TerminalShellStatus::Creating),
		pair: Mutex::new(pair),
		child: Mutex::new(child),
		child_killer: Mutex::new(child_killer),
		writer: Mutex::new(writer),
		reader: Mutex::new(reader),
	};

	Ok(shell_model)
}

fn build_meta_model(spawn_contract: &TerminalSpawnContract) -> TerminalMetaModel {
	TerminalMetaModel {
		project_id: spawn_contract.project_id,
		task_id: spawn_contract.task_id,
		name: build_name(spawn_contract.name.clone()),
	}
}

fn build_behavior_model(spawn_contract: &TerminalSpawnContract) -> TerminalBehaviorModel {
	TerminalBehaviorModel {
		force_kill: spawn_contract.force_kill,
		history_persistence: spawn_contract.history_persistence.clone(),
	}
}

fn build_name(name: Option<String>) -> String {
	match name {
		Some(ref name) => name.clone(),
		None => "PowerShell".to_string(),
	}
}

async fn start_handle_threads(app_handle: &AppHandle, session_id: &Uuid) -> Result<()> {
	let (session_kill_sender, mut session_kill_receiver) = oneshot::channel();
	let (session_finished_sender, session_finished_receiver) = oneshot::channel();

	let kill_app_handle = app_handle.clone();
	let kill_session_id = session_id.clone();

	tauri::async_runtime::spawn(async move {
		let session = terminal_repository::get_one(&kill_session_id).await.unwrap();

		*session.shell.read().await.status.lock().await = TerminalShellStatus::Running;

		TerminalShellSpawnedEvent(kill_session_id).emit(&kill_app_handle).unwrap();

		let exit_code = session.shell.read().await.child.lock().await.wait().unwrap().exit_code();

		session_kill_sender.send(()).unwrap();

		println!(
			"Session exited with code {} in  {:?}",
			exit_code,
			*session.shell.read().await.status.lock().await
		);

		let mut read_unlock_cmd = CommandBuilder::new("pwsh.exe");
		read_unlock_cmd.args(["-Command", "echo 'Shell killed'"]);
		session.shell.read().await.pair.lock().await.slave.spawn_command(read_unlock_cmd).unwrap();

		drop(session.shell.read().await.writer.lock().await);
		drop(session.shell.read().await.pair.lock().await);

		if *session.shell.read().await.status.lock().await == TerminalShellStatus::Restarting {
			println!("Child of session {} finished due to restart", session.id);
			return;
		}

		let successful_exit_code = exit_code == 0;

		let mut keep_session = match session.behavior.read().await.history_persistence {
			TerminalHistoryPersistence::Always => true,
			TerminalHistoryPersistence::Never => false,
			TerminalHistoryPersistence::OnError if !successful_exit_code => true,
			TerminalHistoryPersistence::OnSuccess if successful_exit_code => true,
			_ => false,
		};

		if *session.shell.read().await.status.lock().await == TerminalShellStatus::Killing {
			keep_session = false;
		}

		*session.shell.read().await.status.lock().await = if successful_exit_code {
			TerminalShellStatus::Killed
		} else {
			TerminalShellStatus::Failed
		};

		if !keep_session {
			delete(&kill_app_handle, kill_session_id).await.unwrap();
		}

		TerminalShellKilledEvent(kill_session_id).emit(&kill_app_handle).unwrap();
	});

	let read_app_handle = app_handle.clone();
	let read_session_id = session_id.clone();
	tauri::async_runtime::spawn(async move {
		let session = terminal_repository::get_one(&read_session_id).await.unwrap();

		let _guard = session.concurrency_guard.lock().await;

		println!("Starting pty session thread {}", read_session_id);

		let mut buf = [0u8; 1024];

		loop {
			if let Ok(()) = session_kill_receiver.try_recv() {
				println!("Received stop signal for session {}", read_session_id);
				break;
			}

			let read_bytes = match session.shell.read().await.reader.lock().await.read(&mut buf) {
				Ok(read_bytes) => read_bytes,
				Err(e) => {
					println!("Reading failed for session {}: {}", read_session_id, e);
					break;
				}
			};

			if let Ok(()) = session_kill_receiver.try_recv() {
				println!("Received stop signal for session {}", read_session_id);
				break;
			}

			if read_bytes == 0 {
				println!("Exited");
				break;
			};

			let read_data = String::from_utf8_lossy(&buf[..read_bytes]).to_string();

			session.history.write().await.push_str(&read_data);

			let event_data = TerminalShellReadEventData {
				id: read_session_id.clone(),
				data: read_data,
			};

			if let Err(e) = TerminalShellReadEvent(event_data).emit(&read_app_handle) {
				println!("Emit failed for session {}: {}", read_session_id, e);
				break;
			}
		}

		drop(read_app_handle);

		println!("Finished pty session thread {}", read_session_id);

		session_finished_sender.send(()).unwrap();
	});

	session_finished_receiver.await.unwrap();

	Ok(())
}

pub async fn write(id: Uuid, data: String) -> Result<()> {
	let session = terminal_repository::get_one(&id).await?;

	session
		.shell
		.read()
		.await
		.writer
		.lock()
		.await
		.write_all(data.as_bytes())
		.map_err(|_| Error::Failed)?;
	Ok(())
}

pub async fn get_read_history(id: Uuid) -> Result<String> {
	let session = terminal_repository::get_one(&id).await?;

	let history = session.history.read().await.clone();
	Ok(history)
}

pub async fn resize(id: Uuid, resize_contract: TerminalResizeContract) -> Result<()> {
	let session = terminal_repository::get_one(&id).await?;

	session
		.shell
		.read()
		.await
		.pair
		.lock()
		.await
		.master
		.resize(PtySize {
			rows: resize_contract.rows,
			cols: resize_contract.cols,
			pixel_width: 0,
			pixel_height: 0,
		})
		.map_err(|_| Error::Failed)?;
	Ok(())
}

pub async fn kill(id: Uuid) -> Result<()> {
	let session = terminal_repository::get_one(&id).await?;

	let shell = session.shell.read().await;

	{
		let mut current_status = shell.status.lock().await;

		println!("Killing pty session {} with status {:?}", id, current_status);

		if *current_status != TerminalShellStatus::Running && *current_status != TerminalShellStatus::Restarting {
			return Err(Error::InvalidStatus);
		}

		if *current_status == TerminalShellStatus::Running {
			*current_status = TerminalShellStatus::Killing;
		}
	}

	let mut child_killer = shell.child_killer.lock().await;

	if session.behavior.read().await.force_kill {
		let mut writer = shell.writer.lock().await;

		// Send ctrl+c to child to terminate the currently running process and ensure read thread is finished
		let ctrl_c: u8 = 3;
		writer.write_all(&[ctrl_c]).map_err(|_| Error::Failed)?;

		sleep(Duration::from_millis(500)).await;
	}

	child_killer.kill().ok();

	Ok(())
}

pub async fn delete(app_handle: &AppHandle, id: Uuid) -> Result<()> {
	let session = terminal_repository::get_one(&id).await?;

	match *session.shell.read().await.status.lock().await {
		TerminalShellStatus::Killed | TerminalShellStatus::Failed => {}
		_ => return Err(Error::InvalidStatus),
	}

	println!("Deleting session {}", id);

	terminal_repository::delete_one(&id).await?;

	TerminalDeletedEvent(id).emit(app_handle).unwrap();

	Ok(())
}

pub async fn kill_or_delete_first(app_handle: &AppHandle, filter: TerminalFilterContract) -> Result<()> {
	let filter_model = TerminalFilterModel::from(filter);
	let session_option = terminal_repository::get_first(filter_model).await;

	if let Some(session) = session_option {
		let status = session.shell.read().await.status.lock().await.clone();

		match status {
			TerminalShellStatus::Killed | TerminalShellStatus::Failed => {
				delete(app_handle, session.id).await?;
			}
			TerminalShellStatus::Running => {
				kill(session.id).await?;
			}
			_ => {
				return Err(Error::InvalidStatus);
			}
		}
	}

	Ok(())
}

pub async fn restart_first_blocking(app_handle: &AppHandle, filter: TerminalFilterContract, spawn_contract: TerminalSpawnContract) -> Result<()> {
	let filter_model = TerminalFilterModel::from(filter);
	let session_option = terminal_repository::get_first(filter_model).await;

	if let Some(session) = session_option {
		*session.shell.read().await.status.lock().await = TerminalShellStatus::Restarting;

		kill(session.id).await?;

		let _ = session.concurrency_guard.lock().await;

		*session.shell.write().await = build_shell_model(&spawn_contract)?;
		*session.behavior.write().await = build_behavior_model(&spawn_contract);
		*session.meta.write().await = build_meta_model(&spawn_contract);

		let id_clone = session.id.clone();
		let app_handle_clone = app_handle.clone();

		let _ = start_handle_threads(&app_handle_clone, &id_clone).await?;
	}

	Ok(())
}

pub async fn restart_first(app_handle: &AppHandle, filter: TerminalFilterContract, spawn_contract: TerminalSpawnContract) -> Result<()> {
	let app_handle_clone = app_handle.clone();

	tauri::async_runtime::spawn(async move { restart_first_blocking(&app_handle_clone, filter, spawn_contract).await });

	Ok(())
}

pub async fn get_is_existing(filter: TerminalFilterContract) -> bool {
	let filter_model = TerminalFilterModel::from(filter);

	terminal_repository::get_is_existing(filter_model).await
}

pub async fn get_many_info(filter: TerminalFilterContract) -> Result<Vec<TerminalInfoContract>> {
	let filter_model = TerminalFilterModel::from(filter);

	let terminals = terminal_repository::get_many_info(filter_model).await?;

	let mut terminal_infos = Vec::new();

	for terminal in terminals {
		let task = match terminal.task_id {
			Some(task_id) => Some(task_service::get_one_info(task_id).await?),
			None => None,
		};

		terminal_infos.push(TerminalInfoContract::from(terminal, task));
	}

	Ok(terminal_infos)
}
