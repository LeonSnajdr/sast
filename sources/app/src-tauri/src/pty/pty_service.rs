use once_cell::sync::Lazy;
use portable_pty::{native_pty_system, Child, ChildKiller, CommandBuilder, PtyPair, PtySize};
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use uuid::Uuid;

use crate::prelude::*;
use crate::pty::pty_contracts::{PtyInfoContract, PtyResizeContract, PtySpawnContract};

static PTY_STATE: Lazy<PtyState> = Lazy::new(|| PtyState {
	sessions: RwLock::new(Vec::new()),
});

struct PtyState {
	// Now using a Vec of sessions instead of a BTreeMap
	sessions: RwLock<Vec<Arc<PtySession>>>,
}

struct PtySession {
	session_id: Uuid,
	project_id: Uuid,
	name: String,
	pair: Mutex<PtyPair>,
	child: Mutex<Box<dyn Child + Send + Sync>>,
	child_killer: Mutex<Box<dyn ChildKiller + Send + Sync>>,
	writer: Mutex<Box<dyn std::io::Write + Send>>,
	reader: Mutex<Box<dyn std::io::Read + Send>>,
	read_history: Mutex<Vec<String>>,
}

/// Spawn (create) a new PTY session, storing it along with its associated project ID and name.
pub async fn pty_spawn(spawn_contract: &PtySpawnContract) -> Result<Uuid> {
	let pty_system = native_pty_system();

	let pair = pty_system
		.openpty(PtySize {
			rows: 24,
			cols: 80,
			pixel_width: 0,
			pixel_height: 0,
		})
		.map_err(|_| Error::Failed)?;

	let writer = pair.master.take_writer().map_err(|_| Error::Failed)?;
	let reader = pair.master.try_clone_reader().map_err(|_| Error::Failed)?;

	let cmd = CommandBuilder::new("powershell.exe");
	let child = pair.slave.spawn_command(cmd).map_err(|_| Error::Failed)?;
	let child_killer = child.clone_killer();

	let session_id = Uuid::new_v4();
	let session = Arc::new(PtySession {
		session_id,
		project_id: spawn_contract.project_id,
		name: spawn_contract.name.clone(),
		pair: Mutex::new(pair),
		child: Mutex::new(child),
		child_killer: Mutex::new(child_killer),
		writer: Mutex::new(writer),
		reader: Mutex::new(reader),
		read_history: Mutex::new(Vec::new()),
	});

	let mut sessions = PTY_STATE.sessions.write().await;
	sessions.push(session);

	println!("{}", session_id);

	Ok(session_id)
}

/// Write data to the specified session ID.
pub async fn pty_write(session_id: &Uuid, data: &String) -> Result<()> {
	let session = pty_get_session(session_id).await?;
	session.writer.lock().await.write_all(data.as_bytes()).map_err(|_| Error::Failed)?;

	Ok(())
}

/// Read data from the specified session ID.
pub async fn pty_read(session_id: &Uuid) -> Result<String> {
	let session = pty_get_session(session_id).await?;

	let mut buf = [0u8; 1024];
	let read_bytes = session.reader.lock().await.read(&mut buf).map_err(|_| Error::Failed)?;

	let read_data = String::from_utf8_lossy(&buf[..read_bytes]).to_string();

	session.read_history.lock().await.push(read_data.clone());

	Ok(read_data)
}

pub async fn get_read_history(session_id: &Uuid) -> Result<Vec<String>> {
	let session = pty_get_session(session_id).await?;

	let history = session.read_history.lock().await.clone();

	Ok(history)
}

/// Resize the PTY for the specified session ID.
pub async fn pty_resize(session_id: &Uuid, resize_contract: &PtyResizeContract) -> Result<()> {
	let session = pty_get_session(session_id).await?;
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

/// Kill the child process for the specified session ID.
pub async fn pty_kill(session_id: &Uuid) -> Result<()> {
	let session = pty_get_session(session_id).await?;
	session.child_killer.lock().await.kill().map_err(|_| Error::Failed)?;

	Ok(())
}

/// Get the exit status (return code) for the specified session ID.
pub async fn pty_exitstatus(session_id: &Uuid) -> Result<u32> {
	let session = pty_get_session(session_id).await?;

	let exitstatus = session.child.lock().await.wait().map_err(|_| Error::Failed)?.exit_code();

	Ok(exitstatus)
}

/// Find a single session (Arc<PtySession>) by its Session ID.
async fn pty_get_session(session_id: &Uuid) -> Result<Arc<PtySession>> {
	let sessions = PTY_STATE.sessions.read().await;
	let session = sessions.iter().find(|&s| s.session_id == *session_id).ok_or(Error::NotExists)?.clone(); // clone the Arc

	Ok(session)
}

pub async fn pty_get_sessions(project_id: &Uuid) -> Result<Vec<PtyInfoContract>> {
	let sessions = PTY_STATE.sessions.read().await;

	// Filter by matching project_id, then map each PtySession to a PtyInfoContract.
	let info_list = sessions
		.iter()
		.filter(|s| s.project_id == *project_id)
		.map(|session| PtyInfoContract {
			session_id: session.session_id,
			project_id: session.project_id,
			name: session.name.clone(),
		})
		.collect::<Vec<PtyInfoContract>>();

	Ok(info_list)
}
