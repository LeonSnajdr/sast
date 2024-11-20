mod commands;
mod contracts;
mod db;
mod error;
mod mappers;
mod models;
mod prelude;
mod repositories;
mod services;

use specta_typescript::Typescript;
use tauri_specta::{collect_commands, Builder};

use crate::commands::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	let builder = Builder::<tauri::Wry>::new().commands(collect_commands![
		setting_commands::initialize_setting,
		setting_commands::get_is_setting_initialized,
		setting_commands::get_setting,
		project_commands::create_project,
		project_commands::get_all_projects,
		project_commands::get_last_opened_project_id,
		project_commands::open_project,
	]);

	#[cfg(debug_assertions)]
	builder
		.export(Typescript::default(), "../utils/tauriBindings.ts")
		.expect("Failed to export typescript bindings");

	tauri::Builder::default()
		.setup(|app| {
			if cfg!(debug_assertions) {
				app.handle().plugin(
					tauri_plugin_log::Builder::default()
						.level(log::LevelFilter::Info)
						.build(),
				)?;
			}

			app.handle().plugin(db::init_sqlx())?;
			Ok(())
		})
		.invoke_handler(builder.invoke_handler())
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
