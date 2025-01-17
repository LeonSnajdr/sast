use std::collections::BTreeMap;
use std::sync::Arc;

use once_cell::sync::Lazy;
use portable_pty::{native_pty_system, Child, ChildKiller, CommandBuilder, PtyPair, PtySize};
use tokio::sync::{Mutex, RwLock};
use uuid::Uuid;

use crate::prelude::*;
use crate::pty::pty_contracts::{ResizePtyContract, SpawnPtyContract};

static PTY_STATE: Lazy<PtyState> = Lazy::new(|| PtyState {
	sessions: RwLock::new(BTreeMap::new()),
});

struct PtyState {
	sessions: RwLock<BTreeMap<Uuid, Arc<PtySession>>>,
}

struct PtySession {
	pair: Mutex<PtyPair>,
	child: Mutex<Box<dyn Child + Send + Sync>>,
	child_killer: Mutex<Box<dyn ChildKiller + Send + Sync>>,
	writer: Mutex<Box<dyn std::io::Write + Send>>,
	reader: Mutex<Box<dyn std::io::Read + Send>>,
}

pub async fn spawn(spawn_contract: &SpawnPtyContract) -> Result<Uuid> {
	let pty_system = native_pty_system();

	let pair = pty_system
		.openpty(PtySize {
			rows: spawn_contract.rows,
			cols: spawn_contract.cols,
			pixel_width: 0,
			pixel_height: 0,
		})
		.map_err(|_| Error::Failed)?;

	let writer = pair.master.take_writer().map_err(|_| Error::Failed)?;
	let reader = pair.master.try_clone_reader().map_err(|_| Error::Failed)?;

	let cmd = CommandBuilder::new("powershell.exe");

	let child = pair.slave.spawn_command(cmd).map_err(|_| Error::Failed)?;
	let child_killer = child.clone_killer();

	let pair = Arc::new(PtySession {
		pair: Mutex::new(pair),
		child: Mutex::new(child),
		child_killer: Mutex::new(child_killer),
		writer: Mutex::new(writer),
		reader: Mutex::new(reader),
	});

	let mut sessions = PTY_STATE.sessions.write().await;
	let session_id = Uuid::new_v4();

	sessions.insert(session_id, pair);

	println!("{}", session_id);

	Ok(session_id)
}

pub async fn write(session_id: &Uuid, data: &String) -> Result<()> {
	let session = get_session(session_id).await?;

	session.writer.lock().await.write_all(data.as_bytes()).map_err(|_| Error::Failed)?;

	Ok(())
}

pub async fn read(session_id: &Uuid) -> Result<String> {
	let session = get_session(session_id).await?;

	let mut buf = [0u8; 1024];
	let read_bytes = session.reader.lock().await.read(&mut buf).map_err(|_| Error::Failed)?;

	Ok(String::from_utf8_lossy(&buf[..read_bytes]).to_string())
}

pub async fn resize(session_id: &Uuid, resize_contract: &ResizePtyContract) -> Result<()> {
	let session = get_session(session_id).await?;

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
	let session = get_session(session_id).await?;

	session.child_killer.lock().await.kill().map_err(|_| Error::Failed)?;

	Ok(())
}

pub async fn exitstatus(session_id: &Uuid) -> Result<u32> {
	let session = get_session(session_id).await?;

	let exitstatus = session.child.lock().await.wait().map_err(|_| Error::Failed)?.exit_code();

	Ok(exitstatus)
}

async fn get_session(session_id: &Uuid) -> Result<Arc<PtySession>> {
	let sessions = PTY_STATE.sessions.read().await;
	let session = sessions.get(session_id).ok_or(Error::NotExists)?.clone();

	// Ensure that the lock on the sessions is dropped, to prevent deadlocks
	drop(sessions);

	Ok(session)
}
