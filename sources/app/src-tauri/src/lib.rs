mod db;
mod error;
mod prelude;
mod project;
mod pty;
mod setting;
mod placeholder;

use specta_typescript::Typescript;
use tauri_specta::{collect_commands, Builder};

use crate::project::project_commands;
use crate::pty::pty_commands;
use crate::setting::setting_commands;
use crate::placeholder::placeholder_commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	let builder = Builder::<tauri::Wry>::new().commands(collect_commands![
		setting_commands::initialize_setting,
		setting_commands::get_setting,
		setting_commands::update_setting,
		project_commands::create_project,
		project_commands::get_all_projects,
		project_commands::get_last_opened_project_id,
		project_commands::open_project,
		pty_commands::pty_spawn,
		pty_commands::pty_write,
		pty_commands::pty_read,
		pty_commands::pty_resize,
		pty_commands::pty_kill,
		pty_commands::pty_exitstatus,
		placeholder_commands::create_placeholder,
		placeholder_commands::get_all_placeholders,
		placeholder_commands::get_placeholder,
	]);

	#[cfg(debug_assertions)]
	builder
		.export(Typescript::default(), "../utils/tauriBindings.ts")
		.expect("Failed to export typescript bindings");

	tauri::Builder::default()
		.plugin(tauri_plugin_shell::init())
		.setup(|app| {
			if cfg!(debug_assertions) {
				app.handle()
					.plugin(tauri_plugin_log::Builder::default().level(log::LevelFilter::Info).build())?;
			}

			app.handle().plugin(db::init_sqlx())?;
			Ok(())
		})
		.invoke_handler(builder.invoke_handler())
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
