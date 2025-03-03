use uuid::Uuid;

use crate::prelude::*;
use crate::task_set::task::task_set_task_contracts::TaskSetTaskInfoContract;
use crate::task_set::task::task_set_task_models::TaskSetTaskModel;
use crate::task_set::task::task_set_task_repository;

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
