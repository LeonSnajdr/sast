use prisma_client_rust::QueryError;

use crate::contracts::task_contracts::{CreateTaskContract, UpdateTaskContract};
use crate::prisma::{task, task_set};
use crate::utils::db_utils::DbState;

pub async fn create_task(
    db: DbState<'_>, create_contract: CreateTaskContract,
) -> Result<task::Data, QueryError> {
    return db
        .task()
        .create(
            create_contract.order,
            create_contract.command,
            create_contract.working_directory,
            create_contract.delay,
            task_set::id::equals(create_contract.task_set_id),
            vec![],
        )
        .exec()
        .await;
}

pub async fn update_task(
    db: DbState<'_>, update_contract: UpdateTaskContract,
) -> Result<task::Data, QueryError> {
    return db
        .task()
        .update(
            task::id::equals(update_contract.id),
            vec![
                task::command::set(update_contract.command),
                task::working_directory::set(update_contract.working_directory),
                task::delay::set(update_contract.delay),
            ],
        )
        .exec()
        .await;
}

pub async fn update_tasks(
    db: DbState<'_>, update_contracts: Vec<UpdateTaskContract>,
) -> Result<Vec<task::Data>, QueryError> {
    let task_updates = update_contracts.into_iter().map(|update_contract| {
        db.task().update(
            task::id::equals(update_contract.id),
            vec![
                task::order::set(update_contract.order),
                task::command::set(update_contract.command),
                task::working_directory::set(update_contract.working_directory),
                task::delay::set(update_contract.delay),
            ],
        )
    });

    return db._batch(task_updates).await;
}

pub async fn delete_task(db: DbState<'_>, task_id: String) -> Result<task::Data, QueryError> {
    return db.task().delete(task::id::equals(task_id)).exec().await;
}
