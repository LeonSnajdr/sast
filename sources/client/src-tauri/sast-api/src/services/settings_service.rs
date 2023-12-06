use prisma_client_rust::QueryError;

use crate::prisma::settings;
use crate::repositories::settings_repository;
use crate::utils::db_utils::DbState;

#[tauri::command]
#[specta::specta]
pub async fn get_settings(db: DbState<'_>) -> Result<Option<settings::Data>, QueryError> {
    // TODO error handling

    settings_repository::upsert_settings(&db).await.unwrap();

    return settings_repository::get_settings(&db).await;
}
