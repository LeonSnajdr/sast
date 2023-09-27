use prisma_client_rust::QueryError;

use crate::contracts::placeholder_contracts::CreatePlaceholderContract;
use crate::prisma::{placeholder, project};
use crate::utils::db_utils::DbState;

pub async fn create_placeholder(
    db: DbState<'_>, create_contract: CreatePlaceholderContract,
) -> Result<placeholder::Data, QueryError> {
    return db
        .placeholder()
        .create(
            create_contract.name,
            create_contract.variety,
            project::id::equals(create_contract.project_id),
            vec![],
        )
        .exec()
        .await;
}
