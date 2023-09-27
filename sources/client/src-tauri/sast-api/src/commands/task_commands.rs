use specta;
use tauri;

use crate::services::task_service;

#[tauri::command]
#[specta::specta]
pub async fn run_command(command: String) -> String {
    return task_service::execute_command(command);
}
