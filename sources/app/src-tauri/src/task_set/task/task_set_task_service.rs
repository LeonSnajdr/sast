use uuid::Uuid;

use crate::prelude::*;
use crate::task::task_service;
use crate::task_set::task::task_set_task_contracts::TaskSetTaskInfoContract;
use crate::task_set::task::task_set_task_models::TaskSetTaskModel;
use crate::task_set::task::task_set_task_repository;
use crate::terminal::terminal_contracts::TerminalSpawnContract;

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

pub async fn get_all_task_ids(task_set_id: Uuid) -> Result<Vec<Uuid>> {
	let task_ids = task_set_task_repository::get_all_task_ids(task_set_id).await?;

	Ok(task_ids)
}

pub async fn build_spawn_contract(project_id: Uuid, task_set_task: &TaskSetTaskModel) -> Result<TerminalSpawnContract> {
	let task_spawn_contract = task_service::build_spawn_contract(project_id, task_set_task.task_id).await?;

	let no_exit = match task_set_task.blocking {
		true => false,
		false => task_spawn_contract.no_exit,
	};

	let spawn_contract = TerminalSpawnContract {
		no_exit,
		..task_spawn_contract
	};

	Ok(spawn_contract)
}
