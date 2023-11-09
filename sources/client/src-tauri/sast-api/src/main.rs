// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod contracts;
#[allow(warnings, unused)]
mod prisma;
mod repositories;
mod services;
mod utils;
use std::{env, fs, path::PathBuf, sync::Arc};
use tauri::api::path;

use crate::commands::{placeholder_commands, project_commands, task_commands, task_set_commands};
use crate::prisma::*;

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
            project_commands::get_full_project,
            project_commands::get_list_projects,
            project_commands::create_project,
            project_commands::update_project,
            project_commands::delete_project,
            placeholder_commands::create_placeholder,
            placeholder_commands::update_placeholder,
            placeholder_commands::delete_placeholder,
            task_set_commands::create_task_set,
            task_set_commands::update_task_set,
            task_set_commands::delete_task_set,
            task_set_commands::start_task_set,
            task_commands::create_task,
            task_commands::update_task,
            task_commands::delete_task
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
