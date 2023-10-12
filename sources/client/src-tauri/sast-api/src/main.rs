// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod contracts;
#[allow(warnings, unused)]
mod prisma;
mod repositories;
mod services;
mod utils;

use specta::collect_types;
use std::sync::Arc;
use tauri_specta::ts;

use crate::commands::placeholder_commands::{
    create_placeholder, delete_placeholder, update_placeholder,
};
use crate::commands::project_commands::{
    create_project, delete_project, get_full_project, get_list_projects, update_project,
};
use crate::commands::task_commands::{create_task, delete_task, update_task};
use crate::commands::task_set_commands::{create_task_set, start_task_set};
use crate::prisma::*;

#[tokio::main]
async fn main() {
    let db = PrismaClient::_builder().build().await.unwrap();

    #[cfg(debug_assertions)]
    ts::export(
        collect_types![
            get_full_project,
            get_list_projects,
            create_project,
            update_project,
            delete_project,
            create_placeholder,
            update_placeholder,
            delete_placeholder,
            create_task_set,
            start_task_set,
            create_task,
            update_task,
            delete_task
        ],
        "../../src/bindings.ts",
    )
    .unwrap();

    #[cfg(debug_assertions)]
    db._db_push().await.unwrap();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_full_project,
            get_list_projects,
            create_project,
            update_project,
            delete_project,
            create_placeholder,
            update_placeholder,
            delete_placeholder,
            create_task_set,
            start_task_set,
            create_task,
            update_task,
            delete_task
        ])
        .manage(Arc::new(db))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
