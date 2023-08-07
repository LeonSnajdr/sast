// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[allow(warnings, unused)]
mod prisma;

use prisma::*;
use prisma_client_rust::QueryError;
use serde::Deserialize;
use specta::{collect_types, Type};
use std::sync::Arc;
use tauri::State;
use tauri_specta::ts;

type DbState<'a> = State<'a, Arc<PrismaClient>>;

#[tauri::command]
#[specta::specta]
async fn get_projects(db: DbState<'_>) -> Result<Vec<project::Data>, QueryError> {
  db.project().find_many(vec![]).exec().await
}

#[tauri::command]
#[specta::specta]
async fn create_project(db: DbState<'_>, create_data: CreateProjectData) -> Result<project::Data, ()> {
  let data: project::Data = db.project().create(create_data.name,  vec![]).exec().await.unwrap();

  return Ok(data);
}

#[derive(Deserialize, Type)]
struct CreateProjectData {
  name: String,
}

#[tokio::main]
async fn main() {
  let db = PrismaClient::_builder().build().await.unwrap();

  #[cfg(debug_assertions)]
  ts::export(collect_types![get_projects, create_project], "../src/bindings.ts").unwrap();

  #[cfg(debug_assertions)]
  db._db_push().await.unwrap();

  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![get_projects, create_project])
      .manage(Arc::new(db))
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}
