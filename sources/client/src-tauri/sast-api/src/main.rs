// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[allow(warnings, unused)]
mod prisma;

use prisma::*;
use serde::Deserialize;
use specta::{collect_types, Type};
use std::sync::Arc;
use tauri::State;
use tauri_specta::ts;

type DbState<'a> = State<'a, Arc<PrismaClient>>;

#[tauri::command]
#[specta::specta]
async fn greet(db: DbState<'_>, create: CreateProjectData) -> Result<String, String> {
  let result = "Test".to_string();

  return Ok(result);
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
  ts::export(collect_types![greet, create_project], "../src/bindings.ts").unwrap();

  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![greet])
      .manage(Arc::new(db))
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}
