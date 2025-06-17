use uuid::Uuid;

use crate::prelude::*;
use crate::task::task_service;
use crate::task_set::task::task_set_task_contracts::TaskSetTaskInfoContract;
use crate::task_set::task::task_set_task_models::{TaskSetTaskInfoModel, TaskSetTaskModel};
use crate::task_set::task::task_set_task_repository;
use crate::terminal::shell::shell_contracts::ShellSpawnContract;
use crate::terminal::terminal_contracts::{TerminalCreateContract, TerminalRestartContract};

pub async fn create_or_replace(task_set_id: Uuid, contracts: Vec<TaskSetTaskInfoContract>) -> Result<()> {
	let models = contracts
		.into_iter()
		.enumerate()
		.map(|(position, contract)| {
			let id = Uuid::new_v4();
			TaskSetTaskModel::from(id, task_set_id, position as i64, contract)
		})
		.collect();

	task_set_task_repository::create_or_replace(task_set_id, models).await?;

	Ok(())
}

pub async fn get_all_info(task_set_id: Uuid) -> Result<Vec<TaskSetTaskInfoContract>> {
	let models = task_set_task_repository::get_all_info(task_set_id).await?;

	let contracts = models.into_iter().map(TaskSetTaskInfoContract::from).collect();

	Ok(contracts)
}

pub async fn build_terminal_create_contract(project_id: Uuid, task_set_task: &TaskSetTaskInfoModel) -> Result<TerminalCreateContract> {
	let task_terminal_create_contract = task_service::build_terminal_create_contract(project_id, task_set_task.task_id).await?;

	let terminal_create_contract = TerminalCreateContract {
		task_set_id: Some(task_set_task.task_set_id),
		..task_terminal_create_contract
	};

	Ok(terminal_create_contract)
}

pub async fn build_terminal_restart_contract(task_set_task: &TaskSetTaskInfoModel) -> Result<TerminalRestartContract> {
	let task_terminal_restart_contract = task_service::build_terminal_restart_contract(task_set_task.task_id).await?;
	
	let terminal_restart_contract = TerminalRestartContract {
		task_set_id: Some(task_set_task.task_set_id),
		..task_terminal_restart_contract
	};
	
	Ok(terminal_restart_contract)
}

pub async fn build_shell_spawn_contract(task_set_task: &TaskSetTaskInfoModel) -> Result<ShellSpawnContract> {
	let task_spawn_contract = task_service::build_shell_spawn_contract(task_set_task.task_id).await?;

	let no_exit = match task_set_task.blocking {
		true => false,
		false => task_spawn_contract.no_exit,
	};

	let spawn_contract = ShellSpawnContract {
		no_exit,
		..task_spawn_contract
	};

	Ok(spawn_contract)
}
