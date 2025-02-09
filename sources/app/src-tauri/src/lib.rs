mod db;
mod error;
mod prelude;
mod project;
mod pty_session;
mod setting;
mod placeholder;

use specta_typescript::Typescript;
use tauri_specta::{collect_commands, collect_events, Builder};

use crate::project::project_commands;
use crate::pty_session::{pty_session_commands, pty_session_events};
use crate::setting::setting_commands;
use crate::placeholder::placeholder_commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	let builder = Builder::<tauri::Wry>::new()
		.commands(
			collect_commands![
				setting_commands::setting_initialize,
				setting_commands::setting_get_default,
				setting_commands::setting_update_one,
				project_commands::project_create,
				project_commands::project_get_all,
				project_commands::project_get_id_last_opened,
				project_commands::project_open,
				pty_session_commands::pty_session_spawn,
				pty_session_commands::pty_session_write,
				pty_session_commands::pty_session_get_read_history,
				pty_session_commands::pty_session_info_get_all,
				pty_session_commands::pty_session_resize,
				pty_session_commands::pty_session_kill,
				pty_session_commands::pty_session_get_exitstatus,
				placeholder_commands::placeholder_create,
				placeholder_commands::placeholder_get_all_global,
				placeholder_commands::placeholder_get_all_project,
				placeholder_commands::placeholder_get_one,
				placeholder_commands::placeholder_update_one,
				placeholder_commands::placeholder_delete_one,
			]
		)
		.events(
			collect_events![
				pty_session_events::PtySessionReadEvent,
				pty_session_events::PtySessionsUpdatedEvent
			]
		);

	#[cfg(debug_assertions)]
	builder
		.export(Typescript::default(), "../utils/tauriBindings.ts")
		.expect("Failed to export typescript bindings");

	tauri::Builder::default()
		.plugin(tauri_plugin_shell::init())
		.invoke_handler(builder.invoke_handler())
		.setup(move |app| {
			if cfg!(debug_assertions) {
				app.handle()
					.plugin(tauri_plugin_log::Builder::default().level(log::LevelFilter::Info).build())?;
			}

			builder.mount_events(app);

			app.handle().plugin(db::init_sqlx())?;
			Ok(())
		})
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
