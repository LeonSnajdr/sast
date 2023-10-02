use prisma_client_rust::QueryError;

use crate::contracts::placeholder_contracts::{
    CreatePlaceholderContract, UpdatePlaceholderContract,
};
use crate::prisma::placeholder;
use crate::repositories::placeholder_repository;
use crate::utils::db_utils::DbState;

pub async fn create_placeholder(
    db: DbState<'_>, create_contract: CreatePlaceholderContract,
) -> Result<placeholder::Data, QueryError> {
    return placeholder_repository::create_placeholder(db, create_contract).await;
}

pub async fn update_placeholder(
    db: DbState<'_>, update_contract: UpdatePlaceholderContract,
) -> Result<placeholder::Data, QueryError> {
    return placeholder_repository::update_placeholder(db, update_contract).await;
}

pub async fn delete_placeholder(
    db: DbState<'_>, placeholder_id: String,
) -> Result<placeholder::Data, QueryError> {
    return placeholder_repository::delete_placeholder(db, placeholder_id).await;
}
