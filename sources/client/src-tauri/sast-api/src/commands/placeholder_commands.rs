use prisma_client_rust::QueryError;
use specta;
use tauri;

use crate::contracts::placeholder_contracts::CreatePlaceholderContract;
use crate::prisma::placeholder;
use crate::services::placeholder_service;
use crate::utils::db_utils::DbState;

#[tauri::command]
#[specta::specta]
pub async fn create_placeholder(
    db: DbState<'_>, create_contract: CreatePlaceholderContract,
) -> Result<placeholder::Data, QueryError> {
    return placeholder_service::create_placeholder(db, create_contract).await;
}
