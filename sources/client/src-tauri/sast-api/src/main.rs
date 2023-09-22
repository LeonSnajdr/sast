// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[allow(warnings, unused)]
mod prisma;
mod contracts;
mod services;
mod commands;
mod utils;

use specta::{collect_types};
use std::sync::Arc;
use tauri_specta::ts;

use crate::prisma::*;
use crate::commands::project_commands::{get_projects, create_project};

#[tokio::main]
async fn main() {
  let db = PrismaClient::_builder().build().await.unwrap();

  #[cfg(debug_assertions)]
  ts::export(collect_types![get_projects, create_project], "../../src/bindings.ts").unwrap();

  #[cfg(debug_assertions)]
  db._db_push().await.unwrap();

  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![get_projects, create_project])
      .manage(Arc::new(db))
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}
