pub mod shell_contracts;
pub mod shell_enums;
pub mod shell_events;

use crate::terminal::shell::shell_contracts::{ShellResizeContract, ShellSpawnContract};
use crate::terminal::shell::shell_enums::ShellKillReason;
use crate::terminal::shell::shell_events::{ShellInputEvent, ShellOutputEvent};
use portable_pty::{CommandBuilder, NativePtySystem, PtySize, PtySystem};
use std::sync::Arc;
use tokio::sync::mpsc::{channel, Sender};
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};
use uuid::Uuid;

pub struct Shell {
	id: Uuid,
	terminal_sender: Arc<Sender<ShellOutputEvent>>,
	guard: Arc<Mutex<()>>,
	shell_sender: Arc<Mutex<Option<Sender<ShellInputEvent>>>>,
	kill_reason: Arc<Mutex<Option<ShellKillReason>>>,
}

impl Shell {
	pub async fn new(terminal_sender: Arc<Sender<ShellOutputEvent>>) -> Self {
		Self {
			id: Uuid::new_v4(),
			terminal_sender,
			guard: Arc::new(Mutex::new(())),
			shell_sender: Arc::new(Mutex::new(None)),
			kill_reason: Arc::new(Mutex::new(None)),
		}
	}

	pub async fn run(&self, spawn_contract: ShellSpawnContract) {
		let pty_system = NativePtySystem::default();

		let pair = pty_system.openpty(PtySize::default()).unwrap();

		let cmd = Self::build_command(&spawn_contract);

		let mut child = pair.slave.spawn_command(cmd).unwrap();
		let mut killer = child.clone_killer();

		drop(pair.slave);

		let reader = Arc::new(std::sync::Mutex::new(pair.master.try_clone_reader().unwrap()));
		let mut writer = pair.master.take_writer().unwrap();

		let id_clone = self.id.clone();
		let terminal_sender = Arc::clone(&self.terminal_sender);
		tokio::spawn(async move {
			let _ = terminal_sender.send(ShellOutputEvent::Spawned).await;

			loop {
				let reader_clone = Arc::clone(&reader);
				let result = tokio::task::spawn_blocking(move || {
					let mut buffer = [0u8; 1024];
					match reader_clone.lock().unwrap().read(&mut buffer) {
						Ok(0) => None,
						Ok(n) => Some(String::from_utf8_lossy(&buffer[..n]).to_string()),
						Err(_error) => None,
					}
				})
				.await
				.unwrap();

				match result {
					Some(output) => {
						let _ = terminal_sender.send(ShellOutputEvent::Data(output)).await;
					}
					None => break,
				}
			}

			log::info!("Shell {} read thread closed", id_clone);
		});

		let (shell_sender, mut shell_receiver) = channel::<ShellInputEvent>(32);
		*self.shell_sender.lock().await = Some(shell_sender);

		let id_clone = self.id.clone();
		let force_kill = spawn_contract.force_kill;
		tokio::spawn(async move {
			while let Some(event) = shell_receiver.recv().await {
				match event {
					ShellInputEvent::Write(text) => {
						if let Err(e) = writer.write_all(text.as_bytes()) {
							log::error!("Shell {} failed writing to pty {:?}", id_clone, e);
							break;
						}
					}
					ShellInputEvent::Resize(resize_contract) => {
						pair.master
							.resize(PtySize {
								rows: resize_contract.rows,
								cols: resize_contract.cols,
								..PtySize::default()
							})
							.unwrap();
					}
					ShellInputEvent::Kill => {
						if force_kill {
							// Send ctrl+c to child to terminate the currently running process
							let ctrl_c: u8 = 3;
							if let Err(e) = writer.write_all(&[ctrl_c]) {
								log::error!("Shell {} force kill failed with error {:?}", id_clone, e);
							}

							sleep(Duration::from_millis(500)).await;
						}

						killer.kill().ok();
						break;
					}
				}
			}

			log::info!("Shell {} writing thread closed", id_clone);
		});

		let id_clone = self.id.clone();
		let terminal_sender = Arc::clone(&self.terminal_sender);
		let shell_sender = Arc::clone(&self.shell_sender);
		let guard = Arc::clone(&self.guard);
		let kill_reason = Arc::clone(&self.kill_reason);
		tokio::task::spawn(async move {
			let _guard = guard.lock().await;

			let exit_status = child.wait().unwrap();

			let mut shell_sender_lock = shell_sender.lock().await;
			if shell_sender_lock.is_some() {
				*shell_sender_lock = None;
			}

			let mut kill_reason_lock = kill_reason.lock().await;

			if kill_reason_lock.is_none() {
				if exit_status.success() {
					*kill_reason_lock = Some(ShellKillReason::Success);
				} else {
					*kill_reason_lock = Some(ShellKillReason::Error {
						code: exit_status.exit_code(),
						message: exit_status.to_string(),
					});
				}
			}

			let reason = kill_reason_lock.take().unwrap();
			let _ = terminal_sender.send(ShellOutputEvent::Killed(reason)).await;

			// workaround, that the lock is hold until the killed event is consumed by the terminal
			sleep(Duration::from_millis(100)).await;

			log::info!("Shell {} waiting thread closed", id_clone);
		});

		// workaround, that the lock of the guard is acquired in the thread before continue
		sleep(Duration::from_millis(100)).await;
	}

	fn build_command(spawn_contract: &ShellSpawnContract) -> CommandBuilder {
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

	pub async fn write(&self, data: String) {
		let _ = self.shell_sender.lock().await.as_mut().unwrap().send(ShellInputEvent::Write(data)).await;
	}

	pub async fn resize(&self, resize_contract: ShellResizeContract) {
		let _ = self
			.shell_sender
			.lock()
			.await
			.as_mut()
			.unwrap()
			.send(ShellInputEvent::Resize(resize_contract))
			.await;
	}

	pub async fn kill(&self, reason: ShellKillReason) {
		*self.kill_reason.lock().await = Some(reason);

		if let Some(shell_sender) = self.shell_sender.lock().await.as_mut() {
			let _ = shell_sender.send(ShellInputEvent::Kill).await;
		}

		self.wait().await;
	}

	pub async fn wait(&self) {
		let _ = self.guard.lock().await;
	}
}
