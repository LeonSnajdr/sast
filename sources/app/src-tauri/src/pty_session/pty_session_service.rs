use std::io::{Read, Write};
use once_cell::sync::Lazy;
use portable_pty::{native_pty_system, Child, CommandBuilder, PtyPair, PtySize};
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use uuid::Uuid;
use tauri::AppHandle;
use tauri_specta::Event;
use crate::prelude::*;
use crate::pty_session::pty_session_contracts::{PtySessionInfoContract, PtySessionResizeContract, PtySessionSpawnContract};
use crate::pty_session::pty_session_events::{PtySessionEvent, PtySessionEventData};

static PTY_STATE: Lazy<PtyState> = Lazy::new(|| PtyState {
	sessions: RwLock::new(Vec::new()),
});

struct PtyState {
	sessions: RwLock<Vec<Arc<PtySession>>>,
}

struct PtySession {
	session_id: Uuid,
	project_id: Uuid,
	name: String,
	pair: Mutex<PtyPair>,
	child: Mutex<Box<dyn Child + Send + Sync>>,
	writer: Mutex<Box<dyn Write + Send>>,
	reader: Mutex<Box<dyn Read + Send>>,
	read_history: Mutex<String>,
}

pub async fn pty_session_spawn(app_handle: AppHandle, spawn_contract: &PtySessionSpawnContract) -> Result<Uuid> {
	let pty_system = native_pty_system();
	let pair = pty_system
		.openpty(PtySize::default())
		.map_err(|_| Error::Failed)?;

	let writer = pair.master.take_writer().map_err(|_| Error::Failed)?;
	let reader = pair.master.try_clone_reader().map_err(|_| Error::Failed)?;

	let cmd = CommandBuilder::new("pwsh.exe");
	let child = pair.slave.spawn_command(cmd).map_err(|_| Error::Failed)?;

	let session_id = Uuid::new_v4();

	let session = Arc::new(PtySession {
		session_id,
		project_id: spawn_contract.project_id,
		name: spawn_contract.name.clone(),
		pair: Mutex::new(pair),
		child: Mutex::new(child),
		writer: Mutex::new(writer),
		reader: Mutex::new(reader),
		read_history: Mutex::new(String::new()),
	});

	let mut sessions = PTY_STATE.sessions.write().await;
	sessions.push(session);

	start_pty_session_read_thread(app_handle, session_id);

	Ok(session_id)
}

fn start_pty_session_read_thread(app_handle: AppHandle, session_id: Uuid) {
	tauri::async_runtime::spawn(async move {
		let session = pty_session_get_one(&session_id).await.unwrap();

		println!("Starting pty session thread {}", session_id);

		loop {
			let mut buf = [0u8; 1024];

			let read_bytes = match session.reader.lock().await.read(&mut buf) {
				Ok(0) => {
					println!("Reached EOF for session {}", session_id);
					break;
				}
				Ok(read_bytes) => read_bytes,
				Err(e) => {
					eprintln!("Reading failed for session {session_id}: {e}");
					break;
				}
			};

			let read_data = String::from_utf8_lossy(&buf[..read_bytes]).to_string();

			session.read_history.lock().await.push_str(&read_data);

			let event_data = PtySessionEventData {
				session_id,
				data: read_data,
			};

			PtySessionEvent(event_data).emit(&app_handle).map_err(|_| Error::Failed).unwrap();
		}

		println!("Finished pty session thread {}", session_id);
	});
}

pub async fn pty_session_write(session_id: &Uuid, data: &String) -> Result<()> {
	let session = pty_session_get_one(session_id).await?;

	session
		.writer
		.lock()
		.await
		.write_all(data.as_bytes())
		.map_err(|_| Error::Failed)?;
	Ok(())
}

pub async fn pty_session_get_read_history(session_id: &Uuid) -> Result<String> {
	let session = pty_session_get_one(session_id).await?;
	let history = session.read_history.lock().await.clone();
	Ok(history)
}

pub async fn pty_session_resize(session_id: &Uuid, resize_contract: &PtySessionResizeContract) -> Result<()> {
	let session = pty_session_get_one(session_id).await?;

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

pub async fn pty_session_kill(session_id: &Uuid) -> Result<()> {
	let session = pty_session_get_one(session_id).await?;

	let mut child = session.child.lock().await;
	let mut writer = session.writer.lock().await;

	// Send ctrl+c to child to terminate the currently running process
	let ctrl_c: u8 = 3;
	writer.write_all(&[ctrl_c]).map_err(|_| Error::Failed)?;

	child.kill().map_err(|_| Error::Failed)?;

	let mut sessions = PTY_STATE.sessions.write().await;
	if let Some(pos) = sessions.iter().position(|s| s.session_id == *session_id) {
		sessions.remove(pos);
	}

	Ok(())
}

pub async fn pty_session_get_exitstatus(session_id: &Uuid) -> Result<u32> {
	let session = pty_session_get_one(session_id).await?;

	let exitstatus = session
		.child
		.lock()
		.await
		.wait()
		.map_err(|_| Error::Failed)?
		.exit_code();

	Ok(exitstatus)
}

async fn pty_session_get_one(session_id: &Uuid) -> Result<Arc<PtySession>> {
	let sessions = PTY_STATE.sessions.read().await;

	let session = sessions
		.iter()
		.find(|&s| s.session_id == *session_id)
		.ok_or(Error::NotExists)?
		.clone();

	drop(sessions);

	Ok(session)
}

pub async fn pty_session_info_get_all(project_id: &Uuid) -> Result<Vec<PtySessionInfoContract>> {
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