use crate::terminal::shell::shell_contracts::ShellSizeContract;
use crate::terminal::shell::shell_enums::ShellKillReason;

#[derive(Debug)]
pub enum ShellOutputEvent {
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
