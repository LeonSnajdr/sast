use crate::terminal::shell::shell_contracts::ShellSizeContract;
use crate::terminal::shell::shell_enums::ShellKillReason;
use tokio::sync::oneshot;

pub type ShellOutputEvent = (ShellOutputEventData, oneshot::Sender<()>);

#[derive(Debug)]
pub enum ShellOutputEventData {
	Spawned,
	Data(String),
	Killed(ShellKillReason),
}

#[derive(Debug)]
pub enum ShellInputEvent {
	Write(String),
	Resize(ShellSizeContract),
	Kill,
}
