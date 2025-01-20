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
		setting_commands::setting_initialize,
		setting_commands::setting_get_default,
		setting_commands::setting_update_one,
		project_commands::project_create,
		project_commands::project_get_all,
		project_commands::project_get_id_last_opened,
		project_commands::project_open,
		pty_commands::pty_spawn,
		pty_commands::pty_write,
		pty_commands::pty_read,
		pty_commands::pty_resize,
		pty_commands::pty_kill,
		pty_commands::pty_exitstatus,
		placeholder_commands::placeholder_create,
		placeholder_commands::placeholder_get_all_global,
		placeholder_commands::placeholder_get_all_project,
		placeholder_commands::placeholder_get_one,
		placeholder_commands::placeholder_update_one,
		placeholder_commands::placeholder_delete_one,
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
