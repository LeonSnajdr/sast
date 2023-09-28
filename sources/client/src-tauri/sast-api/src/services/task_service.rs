use prisma_client_rust::QueryError;

use crate::contracts::task_contracts::CreateTaskContract;
use crate::prisma::task;
use crate::repositories::task_repository;
use crate::utils::db_utils::DbState;

pub async fn create_task(
    db: DbState<'_>, create_contract: CreateTaskContract,
) -> Result<task::Data, QueryError> {
    return task_repository::create_task(db, create_contract).await;
}
