mod commands;
mod db;
mod dtos;
mod error;
mod prelude;
mod repositories;
mod services;

use commands::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
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
		.invoke_handler(tauri::generate_handler![
			project_commands::create_project,
			project_commands::get_all_projects
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
