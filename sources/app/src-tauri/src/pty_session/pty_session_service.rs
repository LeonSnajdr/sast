use crate::prelude::*;
use crate::pty_session::pty_session_contracts::{PtySessionFilterContract, PtySessionInfoContract, PtySessionResizeContract, PtySessionSpawnContract};
use crate::pty_session::pty_session_enums::{PtySessionHistoryPersistence, PtySessionShellStatus};
use crate::pty_session::pty_session_events::{PtySessionKilledEvent, PtySessionReadEvent, PtySessionReadEventData, PtySessionSpawnedEvent};
use crate::pty_session::pty_session_models::{PtySessionFilterModel, PtySessionModel};
use crate::pty_session::pty_session_repository;
use portable_pty::{native_pty_system, CommandBuilder, PtySize};
use std::io::{Read, Write};
use tauri::AppHandle;
use tauri_specta::Event;
use tokio::sync::{oneshot, Mutex};
use tokio::time::{sleep, Duration};
use uuid::Uuid;

pub async fn spawn_blocking(app_handle: &AppHandle, spawn_contract: PtySessionSpawnContract) -> Result<()> {
	let session_id = build_and_spawn(app_handle, spawn_contract).await?;

	start_handle_threads(app_handle, &session_id).await;

	println!("threads finished");

	Ok(())
}

pub async fn spawn(app_handle: &AppHandle, spawn_contract: PtySessionSpawnContract) -> Result<Uuid> {
	let session_id = build_and_spawn(app_handle, spawn_contract).await?;
	let app_handle_clone = app_handle.clone();

	tauri::async_runtime::spawn(async move { start_handle_threads(&app_handle_clone, &session_id).await });

	Ok(session_id)
}

async fn build_and_spawn(app_handle: &AppHandle, spawn_contract: PtySessionSpawnContract) -> Result<Uuid> {
	let pty_system = native_pty_system();
	let pair = pty_system.openpty(PtySize::default()).map_err(|_| Error::Failed)?;

	let writer = pair.master.take_writer().map_err(|_| Error::Failed)?;
	let reader = pair.master.try_clone_reader().map_err(|_| Error::Failed)?;

	let cmd = build_command(&spawn_contract);

	let child = pair.slave.spawn_command(cmd).map_err(|_| Error::Failed)?;
	let child_killer = child.clone_killer();

	let id = Uuid::new_v4();
	let session = PtySessionModel {
		id,
		project_id: spawn_contract.project_id,
		task_id: spawn_contract.task_id,
		task_set_id: spawn_contract.task_set_id,
		name: build_name(spawn_contract.name),
		force_kill: spawn_contract.force_kill,
		history_persistence: spawn_contract.history_persistence,
		shell_status: Mutex::new(PtySessionShellStatus::Running),
		pair: Mutex::new(pair),
		child: Mutex::new(child),
		child_killer: Mutex::new(child_killer),
		writer: Mutex::new(writer),
		reader: Mutex::new(reader),
		read_history: Mutex::new(String::new()),
	};

	pty_session_repository::create_one(session).await?;

	PtySessionSpawnedEvent(id).emit(app_handle).map_err(|_| Error::Failed)?;

	Ok(id)
}

fn build_command(spawn_contract: &PtySessionSpawnContract) -> CommandBuilder {
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

fn build_name(name: Option<String>) -> String {
	match name {
		Some(ref name) => name.clone(),
		None => "PowerShell".to_string(),
	}
}

async fn start_handle_threads(app_handle: &AppHandle, session_id: &Uuid) {
	let (session_kill_sender, mut session_kill_receiver) = oneshot::channel();
	let (session_finished_sender, session_finished_receiver) = oneshot::channel();

	let kill_app_handle = app_handle.clone();
	let kill_session_id = session_id.clone();

	tauri::async_runtime::spawn(async move {
		let session = pty_session_repository::get_one(&kill_session_id).await.unwrap();

		loop {
			let exit_code = session.child.lock().await.wait().unwrap().exit_code();

			if *session.shell_status.lock().await == PtySessionShellStatus::Restarting {
				continue;
			}

			println!("Session exited with code {}", exit_code);

			session_kill_sender.send(()).unwrap();

			let mut read_unlock_cmd = CommandBuilder::new("pwsh.exe");
			read_unlock_cmd.args(["-Command", "echo 'Session killed'"]);
			session.pair.lock().await.slave.spawn_command(read_unlock_cmd).unwrap();

			drop(session.writer.lock().await);
			drop(session.pair.lock().await);

			*session.shell_status.lock().await = PtySessionShellStatus::Killed;

			let delete_session = match session.history_persistence {
				PtySessionHistoryPersistence::Never => true,
				PtySessionHistoryPersistence::OnError if exit_code != 0 => true,
				PtySessionHistoryPersistence::OnSuccess if exit_code == 0 => true,
				_ => false,
			};

			if delete_session {
				pty_session_repository::delete_one(kill_session_id).await.unwrap();
			}

			PtySessionKilledEvent(kill_session_id).emit(&kill_app_handle).unwrap();

			break;
		}
	});

	let read_app_handle = app_handle.clone();
	let read_session_id = session_id.clone();
	tauri::async_runtime::spawn(async move {
		let session = pty_session_repository::get_one(&read_session_id).await.unwrap();

		println!("Starting pty session thread {}", read_session_id);

		let mut buf = [0u8; 1024];

		loop {
			if let Ok(()) = session_kill_receiver.try_recv() {
				println!("Received stop signal for session {}", read_session_id);
				break;
			}

			let read_bytes = match session.reader.lock().await.read(&mut buf) {
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

			session.read_history.lock().await.push_str(&read_data);

			let event_data = PtySessionReadEventData {
				id: read_session_id.clone(),
				data: read_data,
			};

			if let Err(e) = PtySessionReadEvent(event_data).emit(&read_app_handle) {
				println!("Emit failed for session {}: {}", read_session_id, e);
				break;
			}
		}

		drop(read_app_handle);

		println!("Finished pty session thread {}", read_session_id);

		session_finished_sender.send(()).unwrap();
	});

	session_finished_receiver.await.unwrap();
}

pub async fn write(id: &Uuid, data: &String) -> Result<()> {
	let session = pty_session_repository::get_one(id).await?;

	session.writer.lock().await.write_all(data.as_bytes()).map_err(|_| Error::Failed)?;
	Ok(())
}

pub async fn get_read_history(id: &Uuid) -> Result<String> {
	let session = pty_session_repository::get_one(id).await?;

	let history = session.read_history.lock().await.clone();
	Ok(history)
}

pub async fn resize(id: &Uuid, resize_contract: &PtySessionResizeContract) -> Result<()> {
	let session = pty_session_repository::get_one(id).await?;

	session
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

pub async fn kill(id: &Uuid) -> Result<()> {
	let session = pty_session_repository::get_one(id).await?;

	let mut child_killer = session.child_killer.lock().await;

	if session.force_kill {
		let mut writer = session.writer.lock().await;

		// Send ctrl+c to child to terminate the currently running process and ensure read thread is finished
		let ctrl_c: u8 = 3;
		writer.write_all(&[ctrl_c]).map_err(|_| Error::Failed)?;

		sleep(Duration::from_millis(500)).await;
	}

	child_killer.kill().ok();

	Ok(())
}

pub async fn kill_many(filter: PtySessionFilterContract) -> Result<()> {
	let sessions = get_many_info(filter).await?;

	for session in sessions {
		kill(&session.id).await?;
	}

	Ok(())
}

pub async fn restart(id: Uuid) -> Result<()> {
	let filter = PtySessionFilterContract {
		id: Some(id),
		..PtySessionFilterContract::default()
	};

	restart_first(filter).await?;

	Ok(())
}

pub async fn restart_first(filter: PtySessionFilterContract) -> Result<()> {
	let filter_model = PtySessionFilterModel::from(filter);

	let session_option = pty_session_repository::get_first(filter_model).await;

	if let Some(session) = session_option {
		let mut cmd = CommandBuilder::new("pwsh.exe");
		cmd.args(["-WorkingDirectory", "C:/Repos/metis/sources/ControlCenter.Api/ControlCenter.Api"]);
		cmd.args(["-NoExit"]);
		cmd.args(["-Command", "dotnet build"]);

		// TODO: hold the lock somehow that the other method in thread blocks and checks afterwards
		*session.shell_status.lock().await = PtySessionShellStatus::Restarting;

		kill(&session.id).await?;

		let child = session.pair.lock().await.slave.spawn_command(cmd).map_err(|_| Error::Failed)?;
		let child_killer = child.clone_killer();

		*session.child.lock().await = child;
		*session.child_killer.lock().await = child_killer;
		*session.shell_status.lock().await = PtySessionShellStatus::Running;
	}

	Ok(())
}

pub async fn get_many_info(filter: PtySessionFilterContract) -> Result<Vec<PtySessionInfoContract>> {
	let filter_model = PtySessionFilterModel::from(filter);

	let session_models = pty_session_repository::get_many_info(filter_model).await?;

	let session_info_contracts = session_models.into_iter().map(PtySessionInfoContract::from).collect();

	Ok(session_info_contracts)
}
