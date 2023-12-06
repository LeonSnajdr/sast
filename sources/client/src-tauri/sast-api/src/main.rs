// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod contracts;
#[allow(warnings, unused)]
mod prisma;
mod repositories;
mod services;
mod utils;
use std::{env, fs, path::PathBuf, sync::Arc};
use tauri::api::path;

use crate::prisma::*;
use crate::services::{
    placeholder_service, project_service, settings_service, task_service, task_set_service,
};

macro_rules! tauri_handlers {
	($($name:path),+) => {{
		#[cfg(debug_assertions)]
		tauri_specta::ts::export(specta::collect_types![$($name),+], "../../src/bindings.ts").unwrap();

		tauri::generate_handler![$($name),+]
	}};
}

#[tokio::main]
async fn main() {
    let db = prisma::new_client_with_url(get_database_url().as_str())
        .await
        .unwrap();

    #[cfg(debug_assertions)]
    db._db_push().await.unwrap();

    #[cfg(not(debug_assertions))]
    db._migrate_deploy().await.unwrap();

    tauri::Builder::default()
        .invoke_handler(tauri_handlers![
            settings_service::get_settings,
            settings_service::update_settings,
            project_service::get_list_projects,
            project_service::create_project,
            project_service::update_project,
            project_service::delete_project,
            placeholder_service::get_placeholders,
            placeholder_service::create_placeholder,
            placeholder_service::update_placeholder,
            placeholder_service::update_placeholders,
            placeholder_service::delete_placeholder,
            task_set_service::get_task_sets,
            task_set_service::get_full_task_set,
            task_set_service::create_task_set,
            task_set_service::update_task_set,
            task_set_service::update_task_sets,
            task_set_service::delete_task_set,
            task_set_service::start_task_set,
            task_service::create_task,
            task_service::update_task,
            task_service::update_tasks,
            task_service::delete_task
        ])
        .manage(Arc::new(db))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn get_database_url() -> String {
    let data_dir = path::data_dir()
        .unwrap_or_else(|| PathBuf::from("./"))
        .join("sast");

    #[cfg(debug_assertions)]
    let data_dir = data_dir.join("dev");

    fs::create_dir_all(&data_dir).expect("Data folder creation failed");

    return format!(
        "file:{}\\data.db",
        data_dir.into_os_string().into_string().unwrap()
    );
}
