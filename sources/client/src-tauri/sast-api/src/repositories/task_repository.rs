use prisma_client_rust::QueryError;

use crate::contracts::task_contracts::CreateTaskContract;
use crate::prisma::{task, task_set};
use crate::utils::db_utils::DbState;

pub async fn create_task(
    db: DbState<'_>, create_contract: CreateTaskContract,
) -> Result<task::Data, QueryError> {
    return db
        .task()
        .create(
            create_contract.variety,
            create_contract.command,
            create_contract.working_directory,
            create_contract.delay,
            task_set::id::equals(create_contract.task_set_id),
            vec![],
        )
        .exec()
        .await;
}
