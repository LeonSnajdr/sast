use prisma_client_rust::{Direction, QueryError};

use crate::contracts::task_set_contracts::{
    full_task_set_contract, project_task_set_contract, CreateTaskSetContract, UpdateTaskSetContract,
};
use crate::prisma::task_set::Update;
use crate::prisma::{project, task_set};
use crate::utils::db_utils::DbState;

pub async fn get_task_sets(
    db: DbState<'_>, project_id: String,
) -> Result<Vec<task_set::Data>, QueryError> {
    let task_sets = db
        .task_set()
        .find_many(vec![task_set::project_id::equals(project_id)])
        .order_by(task_set::order::order(Direction::Asc))
        .exec()
        .await;

    return task_sets;
}

pub async fn create_task_set(
    db: DbState<'_>, create_contract: CreateTaskSetContract,
) -> Result<full_task_set_contract::Data, QueryError> {
    return db
        .task_set()
        .create(
            create_contract.order,
            create_contract.name,
            create_contract.description,
            project::id::equals(create_contract.project_id),
            vec![],
        )
        .include(full_task_set_contract::include())
        .exec()
        .await;
}

pub async fn update_task_set(
    db: DbState<'_>, update_contract: UpdateTaskSetContract,
) -> Result<full_task_set_contract::Data, QueryError> {
    return db
        .task_set()
        .update(
            task_set::id::equals(update_contract.id),
            vec![
                task_set::description::set(update_contract.description),
                task_set::order::set(update_contract.order),
            ],
        )
        .include(full_task_set_contract::include())
        .exec()
        .await;
}

pub async fn update_task_sets(
    db: DbState<'_>, update_contracts: Vec<UpdateTaskSetContract>,
) -> Result<Vec<task_set::Data>, QueryError> {
    let task_set_updates = update_contracts.into_iter().map(|update_contract| {
        db.task_set().update(
            task_set::id::equals(update_contract.id),
            vec![
                task_set::description::set(update_contract.description),
                task_set::order::set(update_contract.order),
            ],
        )
    });

    return db._batch(task_set_updates).await;
}

pub async fn delete_task_set(
    db: DbState<'_>, task_set_id: String,
) -> Result<full_task_set_contract::Data, QueryError> {
    return db
        .task_set()
        .delete(task_set::id::equals(task_set_id))
        .include(full_task_set_contract::include())
        .exec()
        .await;
}

pub async fn get_project_task_set(
    db: DbState<'_>, task_set_id: String,
) -> Result<Option<project_task_set_contract::Data>, QueryError> {
    return db
        .task_set()
        .find_first(vec![task_set::id::equals(task_set_id)])
        .include(project_task_set_contract::include())
        .exec()
        .await;
}
