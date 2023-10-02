use prisma_client_rust::QueryError;

use crate::contracts::placeholder_contracts::{
    CreatePlaceholderContract, UpdatePlaceholderContract,
};
use crate::prisma::{placeholder, project};
use crate::utils::db_utils::DbState;

pub async fn create_placeholder(
    db: DbState<'_>, create_contract: CreatePlaceholderContract,
) -> Result<placeholder::Data, QueryError> {
    return db
        .placeholder()
        .create(
            create_contract.name,
            create_contract.value,
            project::id::equals(create_contract.project_id),
            vec![],
        )
        .exec()
        .await;
}

pub async fn update_placeholder(
    db: DbState<'_>, update_contract: UpdatePlaceholderContract,
) -> Result<placeholder::Data, QueryError> {
    return db
        .placeholder()
        .update(
            placeholder::id::equals(update_contract.id),
            vec![placeholder::value::set(update_contract.value)],
        )
        .exec()
        .await;
}

pub async fn delete_placeholder(
    db: DbState<'_>, placeholder_id: String,
) -> Result<placeholder::Data, QueryError> {
    return db
        .placeholder()
        .delete(placeholder::id::equals(placeholder_id))
        .exec()
        .await;
}
