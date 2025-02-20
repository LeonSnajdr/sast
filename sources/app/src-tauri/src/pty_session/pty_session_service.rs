use crate::prelude::*;
use crate::pty_session::pty_session_contracts::{PtySessionInfoContract, PtySessionResizeContract, PtySessionSpawnContract};
use crate::pty_session::pty_session_events::{PtySessionKilledEvent, PtySessionReadEvent, PtySessionReadEventData, PtySessionSpawnedEvent};
use once_cell::sync::Lazy;
use portable_pty::{native_pty_system, Child, ChildKiller, CommandBuilder, PtyPair, PtySize};
use std::io::{Read, Write};
use std::sync::Arc;
use tauri::AppHandle;
use tauri_specta::Event;
use tokio::sync::{oneshot, Mutex, RwLock};
use uuid::Uuid;

static PTY_STATE: Lazy<PtyState> = Lazy::new(|| PtyState {
	sessions: RwLock::new(Vec::new()),
});

struct PtyState {
	sessions: RwLock<Vec<Arc<PtySession>>>,
}

struct PtySession {
	session_id: Uuid,
	project_id: Uuid,
	task_set_id: Option<Uuid>,
	name: String,
	pair: Mutex<PtyPair>,
	child: Mutex<Box<dyn Child + Send + Sync>>,
	child_killer: Mutex<Box<dyn ChildKiller + Send + Sync>>,
	writer: Mutex<Box<dyn Write + Send>>,
	reader: Mutex<Box<dyn Read + Send>>,
	read_history: Mutex<String>,
}

pub async fn spawn_blocking(app_handle: AppHandle, spawn_contract: PtySessionSpawnContract) -> Result<()> {
	let session_id = build_and_spawn(&app_handle, spawn_contract).await?;

	start_handle_threads(&app_handle, &session_id).await;

	println!("threads finished");

	Ok(())
}

pub async fn spawn(app_handle: AppHandle, spawn_contract: PtySessionSpawnContract) -> Result<Uuid> {
	let session_id = build_and_spawn(&app_handle, spawn_contract).await?;

	tauri::async_runtime::spawn(async move { start_handle_threads(&app_handle, &session_id).await });

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

	let session_id = Uuid::new_v4();
	let session = Arc::new(PtySession {
		session_id,
		project_id: spawn_contract.project_id,
		task_set_id: spawn_contract.task_set_id,
		name: build_name(spawn_contract.name),
		pair: Mutex::new(pair),
		child: Mutex::new(child),
		child_killer: Mutex::new(child_killer),
		writer: Mutex::new(writer),
		reader: Mutex::new(reader),
		read_history: Mutex::new(String::new()),
	});

	PTY_STATE.sessions.write().await.push(session);

	PtySessionSpawnedEvent(session_id).emit(app_handle).map_err(|_| Error::Failed)?;

	Ok(session_id)
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
	let (session_finished_sender, mut session_finished_receiver) = oneshot::channel();

	let kill_app_handle = app_handle.clone();
	let kill_session_id = session_id.clone();
	tauri::async_runtime::spawn(async move {
		let session = get_one(&kill_session_id).await.unwrap();

		session.child.lock().await.wait().unwrap();

		session_kill_sender.send(()).unwrap();

		session.pair.lock().await.slave.spawn_command(CommandBuilder::new("whoami")).unwrap();

		drop(session.writer.lock().await);
		drop(session.pair.lock().await);

		{
			let mut sessions = PTY_STATE.sessions.write().await;
			if let Some(pos) = sessions.iter().position(|s| s.session_id == kill_session_id) {
				sessions.remove(pos);
			}
		}

		PtySessionKilledEvent(kill_session_id).emit(&kill_app_handle).unwrap();

		drop(kill_app_handle);
	});

	let read_app_handle = app_handle.clone();
	let read_session_id = session_id.clone();
	tauri::async_runtime::spawn(async move {
		let session = get_one(&read_session_id).await.unwrap();

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
				session_id: read_session_id.clone(),
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

pub async fn write(session_id: &Uuid, data: &String) -> Result<()> {
	let session = get_one(session_id).await?;

	session.writer.lock().await.write_all(data.as_bytes()).map_err(|_| Error::Failed)?;
	Ok(())
}

pub async fn get_read_history(session_id: &Uuid) -> Result<String> {
	let session = get_one(session_id).await?;
	let history = session.read_history.lock().await.clone();
	Ok(history)
}

pub async fn resize(session_id: &Uuid, resize_contract: &PtySessionResizeContract) -> Result<()> {
	let session = get_one(session_id).await?;

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

pub async fn kill(session_id: &Uuid) -> Result<()> {
	let session = get_one(session_id).await?;

	let mut writer = session.writer.lock().await;
	let mut child_killer = session.child_killer.lock().await;

	// Send ctrl+c to child to terminate the currently running process and ensure read thread is finished
	let ctrl_c: u8 = 3;
	writer.write_all(&[ctrl_c]).map_err(|_| Error::Failed)?;

	child_killer.kill().ok();

	Ok(())
}

async fn get_one(session_id: &Uuid) -> Result<Arc<PtySession>> {
	let sessions = PTY_STATE.sessions.read().await;

	let session = sessions.iter().find(|&s| s.session_id == *session_id).ok_or(Error::NotExists)?.clone();

	drop(sessions);

	Ok(session)
}

pub async fn get_many_info(project_id: &Uuid) -> Result<Vec<PtySessionInfoContract>> {
	let sessions = PTY_STATE.sessions.read().await;

	let info_list = sessions
		.iter()
		.filter(|s| s.project_id == *project_id)
		.map(|session| PtySessionInfoContract {
			session_id: session.session_id,
			project_id: session.project_id,
			name: session.name.clone(),
		})
		.collect::<Vec<PtySessionInfoContract>>();

	Ok(info_list)
}
