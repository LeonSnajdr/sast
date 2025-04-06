pub mod shell;
pub mod terminal_commands;
pub mod terminal_contracts;
pub mod terminal_enums;
pub mod terminal_events;
pub mod terminal_filters;
pub mod terminal_models;
pub mod terminal_repository;
pub mod terminal_service;

pub mod terminal_mapper;

use crate::prelude::*;
use crate::terminal::shell::shell_contracts::{ShellResizeContract, ShellSpawnContract};
use crate::terminal::shell::shell_enums::ShellKillReason;
use crate::terminal::shell::shell_events::ShellOutputEvent;
use crate::terminal::shell::Shell;
use crate::terminal::terminal_contracts::TerminalCreateContract;
use crate::terminal::terminal_events::{
	TerminalCreatedEvent, TerminalDeletedEvent, TerminalShellReadEvent, TerminalShellReadEventData, TerminalStatusChangedEvent, TerminalStatusChangedEventData,
};

use crate::terminal::terminal_enums::{TerminalHistoryPersistence, TerminalShellStatus};
use std::sync::Arc;
use tauri::AppHandle;
use tauri_specta::Event;
use tokio::sync::mpsc::Sender;
use tokio::sync::{mpsc, Mutex, RwLock};
use tokio::task::JoinHandle;
use uuid::Uuid;

pub struct Terminal {
	pub id: Uuid,
	pub meta: Arc<RwLock<TerminalMeta>>,
	pub behavior: Arc<RwLock<TerminalBehavior>>,
	pub history: Arc<RwLock<String>>,
	pub shell_status: Arc<RwLock<TerminalShellStatus>>,
	app_handle: Arc<AppHandle>,
	handle: Arc<Mutex<Option<JoinHandle<()>>>>,
	sender: Arc<Sender<ShellOutputEvent>>,
	shell: Arc<RwLock<Option<Shell>>>,
}

pub struct TerminalMeta {
	pub name: String,
	pub project_id: Uuid,
	pub task_id: Option<Uuid>,
}

pub struct TerminalBehavior {
	pub history_persistence: TerminalHistoryPersistence,
}

impl Terminal {
	pub async fn create(app_handle: AppHandle, spawn_contract: TerminalCreateContract) -> Result<Arc<Terminal>> {
		let (sender, mut receiver) = mpsc::channel(100);
		let id = Uuid::new_v4();
		let app_handle = Arc::new(app_handle);
		let behavior = Arc::new(RwLock::new(TerminalBehavior {
			history_persistence: spawn_contract.history_persistence,
		}));
		let shell = Arc::new(RwLock::new(None));
		let history = Arc::new(RwLock::new(String::new()));
		let shell_status = Arc::new(RwLock::new(TerminalShellStatus::None));

		let id_clone = id.clone();
		let behavior_clone = Arc::clone(&behavior);
		let app_handle_clone = Arc::clone(&app_handle);
		let shell_clone = Arc::clone(&shell);
		let history_clone = Arc::clone(&history);
		let shell_status_clone = Arc::clone(&shell_status);
		let handle = tokio::spawn(async move {
			while let Some(event) = receiver.recv().await {
				match event {
					ShellOutputEvent::Spawned => {
						println!("Terminal: Shell spawned.");
						*shell_status_clone.write().await = TerminalShellStatus::Running;
						let _ = TerminalStatusChangedEvent(TerminalStatusChangedEventData {
							id: id_clone,
							status: shell_status_clone.read().await.clone(),
						})
						.emit(app_handle_clone.as_ref());
					}
					ShellOutputEvent::Data(text) => {
						history_clone.write().await.push_str(&text);

						let _ = TerminalShellReadEvent(TerminalShellReadEventData { id: id_clone, data: text }).emit(app_handle_clone.as_ref());
					}
					ShellOutputEvent::Killed(reason) => {
						println!("Terminal: Shell closed {:?}", reason);
						*shell_clone.write().await = None;

						let persist_terminal = match (behavior_clone.read().await.history_persistence.clone(), reason.clone()) {
							(_, ShellKillReason::Restart) => true,
							(TerminalHistoryPersistence::Always, _) => true,
							(TerminalHistoryPersistence::OnSuccess, ShellKillReason::Success) => true,
							(TerminalHistoryPersistence::OnError, ShellKillReason::Error { code: _, message: _ }) => true,
							_ => false,
						};

						if persist_terminal {
							*shell_status_clone.write().await = match reason {
								ShellKillReason::Manually => TerminalShellStatus::Killed,
								ShellKillReason::Success => TerminalShellStatus::Killed,
								ShellKillReason::Restart => TerminalShellStatus::Restarting,
								ShellKillReason::Error { code, message } => TerminalShellStatus::Crashed { code, message },
							};

							let _ = TerminalStatusChangedEvent(TerminalStatusChangedEventData {
								id: id_clone,
								status: shell_status_clone.read().await.clone(),
							})
							.emit(app_handle_clone.as_ref());
						} else {
							let _ = terminal_repository::delete_one(&id_clone).await;
							let _ = TerminalDeletedEvent(id_clone).emit(app_handle_clone.as_ref());
						}
					}
				}
			}

			println!("Terminal: Event channel closed. Listener exiting.");
		});

		let meta = TerminalMeta {
			name: spawn_contract.name.unwrap_or("PowerShell".to_string()),
			project_id: spawn_contract.project_id,
			task_id: spawn_contract.task_id,
		};

		let app_handle_clone = Arc::clone(&app_handle);
		let terminal = Self {
			id,
			meta: Arc::new(RwLock::new(meta)),
			behavior,
			app_handle: app_handle_clone,
			handle: Arc::new(Mutex::new(Some(handle))),
			sender: Arc::new(sender),
			shell,
			history,
			shell_status,
		};

		let created_terminal = terminal_repository::create_one(terminal).await?;
		TerminalCreatedEvent(id).emit(app_handle.as_ref()).map_err(|_| Error::EventEmit)?;

		Ok(created_terminal)
	}

	pub async fn get_history(&self) -> String {
		let history_guard = self.history.read().await;
		history_guard.clone()
	}

	pub async fn close(&self) -> Result<()> {
		self.shell_kill(ShellKillReason::Manually).await;

		let mut handle_lock = self.handle.lock().await;
		if let Some(handle) = handle_lock.as_mut() {
			handle.abort();
			let _ = handle.await;
			*handle_lock = None;
		}

		terminal_repository::delete_one(&self.id).await?;

		// TODO: Rename event to closed
		TerminalDeletedEvent(self.id).emit(self.app_handle.as_ref()).map_err(|_| Error::EventEmit)?;

		Ok(())
	}

	pub async fn shell_spawn(&self, spawn_contract: ShellSpawnContract) {
		let mut shell = Shell::new(self.sender.clone()).await;
		shell.run(spawn_contract).await;
		*self.shell.write().await = Some(shell);
	}

	pub async fn shell_spawn_blocking(&self, spawn_contract: ShellSpawnContract) {
		self.shell_spawn(spawn_contract).await;

		if let Some(shell) = self.shell.read().await.as_ref() {
			shell.wait().await;
		}
	}

	pub async fn shell_restart(&self, spawn_contract: ShellSpawnContract) {
		self.shell_kill(ShellKillReason::Restart).await;
		self.shell_spawn(spawn_contract).await;
	}

	pub async fn shell_restart_blocking(&self, spawn_contract: ShellSpawnContract) {
		self.shell_restart(spawn_contract).await;

		if let Some(shell) = self.shell.read().await.as_ref() {
			shell.wait().await;
		}
	}

	pub async fn shell_write(&self, data: String) {
		if let Some(shell) = self.shell.read().await.as_ref() {
			shell.write(data).await;
		}
	}

	pub async fn shell_resize(&self, resize_contract: ShellResizeContract) {
		if let Some(shell) = self.shell.read().await.as_ref() {
			shell.resize(resize_contract).await;
		}
	}

	pub async fn shell_kill(&self, reason: ShellKillReason) {
		if let Some(shell) = self.shell.read().await.as_ref() {
			shell.kill(reason).await;
		}
	}
}
