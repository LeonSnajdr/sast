use prisma_client_rust::QueryError;

use crate::contracts::placeholder_contracts::CreatePlaceholderContract;
use crate::prisma::placeholder;
use crate::repositories::placeholder_repository;
use crate::utils::db_utils::DbState;

pub async fn create_placeholder(db: DbState<'_>, create_contract: CreatePlaceholderContract) -> Result<placeholder::Data, QueryError> {
    return placeholder_repository::create_placeholder(db, create_contract).await;
}
