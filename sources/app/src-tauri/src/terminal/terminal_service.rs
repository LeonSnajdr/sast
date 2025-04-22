use crate::prelude::*;
use crate::task::task_service;
use crate::terminal::shell::shell_contracts::{ShellSizeContract, ShellSpawnContract};
use crate::terminal::shell::shell_enums::ShellKillReason;
use crate::terminal::terminal_contracts::{TerminalCreateContract, TerminalInfoContract, TerminalOpenContract};
use crate::terminal::terminal_enums::TerminalShellStatus;
use crate::terminal::terminal_filters::TerminalFilter;
use crate::terminal::{terminal_repository, Terminal};
use tauri::AppHandle;
use uuid::Uuid;

pub async fn create(app_handle: AppHandle, create_contract: TerminalCreateContract, spawn_contract: Option<ShellSpawnContract>) -> Result<Uuid> {
	let terminal = Terminal::create(app_handle, create_contract).await?;

	if let Some(spawn_contract) = spawn_contract {
		terminal.shell_spawn(spawn_contract).await;
	}

	Ok(terminal.id)
}

pub async fn create_blocking(app_handle: AppHandle, create_contract: TerminalCreateContract, spawn_contract: ShellSpawnContract) -> Result<bool> {
	let terminal = Terminal::create(app_handle, create_contract).await?;

	terminal.shell_spawn_blocking(spawn_contract).await
}

pub async fn get_one_open(id: Uuid) -> Result<TerminalOpenContract> {
	let open_model = terminal_repository::get_one_open(&id).await?;

	let open_contract = TerminalOpenContract::from(open_model);

	Ok(open_contract)
}

pub async fn close(id: Uuid) -> Result<()> {
	let terminal = terminal_repository::get_one(&id).await?;

	terminal.close().await?;

	Ok(())
}

pub async fn shell_write(id: Uuid, data: String) -> Result<()> {
	let terminal = terminal_repository::get_one(&id).await?;

	terminal.shell_write(data).await;

	Ok(())
}

pub async fn shell_resize(id: Uuid, resize_contract: ShellSizeContract) -> Result<()> {
	let terminal = terminal_repository::get_one(&id).await?;

	terminal.shell_resize(resize_contract).await;

	Ok(())
}

pub async fn close_first(filter: &TerminalFilter) -> Result<()> {
	let terminal = terminal_repository::get_first(filter).await;

	if let Some(terminal) = terminal {
		terminal.close().await?;
	}

	Ok(())
}

pub async fn close_many(filter: &TerminalFilter) -> Result<()> {
	let terminals = terminal_repository::get_many(filter).await?;

	for terminal in terminals {
		terminal.close().await?;
	}

	Ok(())
}

pub async fn restart_schedule(filter: &TerminalFilter) -> Result<()> {
	let terminals = terminal_repository::get_many(filter).await?;

	for terminal in terminals {
		terminal.shell_kill(ShellKillReason::Restart).await;

		// This is needed if the shell already crashed before the restart. This should be reworked but is fine for now.
		*terminal.shell_status.write().await = TerminalShellStatus::Restarting;
	}

	Ok(())
}

pub async fn shell_restart_first_blocking(filter: &TerminalFilter, spawn_contract: ShellSpawnContract) -> Result<bool> {
	let terminal = terminal_repository::get_first(filter).await;

	if let Some(terminal) = terminal {
		return terminal.shell_restart_blocking(spawn_contract).await;
	}

	Ok(true)
}

pub async fn shell_restart_first(filter: &TerminalFilter, spawn_contract: ShellSpawnContract) -> Result<()> {
	let terminal = terminal_repository::get_first(filter).await;

	if let Some(terminal) = terminal {
		terminal.shell_restart(spawn_contract).await;
	}

	Ok(())
}

pub async fn get_is_existing(filter: &TerminalFilter) -> bool {
	terminal_repository::get_is_existing(filter).await
}

pub async fn get_many_info(filter: &TerminalFilter) -> Result<Vec<TerminalInfoContract>> {
	let terminals = terminal_repository::get_many_info(filter).await?;
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
