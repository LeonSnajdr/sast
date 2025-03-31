mod db;
mod error;
mod placeholder;
mod prelude;
mod project;
mod setting;
mod task;
mod task_set;
mod terminal;

use specta_typescript::Typescript;
use tauri::Manager;
use tauri_specta::{collect_commands, collect_events, Builder};

use crate::placeholder::placeholder_commands;
use crate::project::project_commands;
use crate::setting::setting_commands;
use crate::task::task_commands;
use crate::task_set::task_set_commands;
use crate::terminal::{terminal_commands, terminal_events};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	let builder = Builder::<tauri::Wry>::new()
		.commands(collect_commands![
			setting_commands::setting_initialize,
			setting_commands::setting_get_default,
			setting_commands::setting_update_one,
			project_commands::project_create,
			project_commands::project_get_all,
			project_commands::project_get_last_opened,
			project_commands::project_open,
			terminal_commands::terminal_spawn,
			terminal_commands::terminal_write,
			terminal_commands::terminal_get_read_history,
			terminal_commands::terminal_get_many_info,
			terminal_commands::terminal_resize,
			terminal_commands::terminal_kill,
			terminal_commands::terminal_delete,
			placeholder_commands::placeholder_create,
			placeholder_commands::placeholder_get_many,
			placeholder_commands::placeholder_get_one,
			placeholder_commands::placeholder_update_one,
			placeholder_commands::placeholder_delete_one,
			task_commands::task_create,
			task_commands::task_get_one,
			task_commands::task_get_many_info,
			task_commands::task_update_one,
			task_commands::task_delete_one,
			task_commands::task_start_one,
			task_commands::task_restart_one,
			task_commands::task_stop_one,
			task_set_commands::task_set_create,
			task_set_commands::task_set_get_one,
			task_set_commands::task_set_get_many_info,
			task_set_commands::task_set_update_one,
			task_set_commands::task_set_delete_one,
			task_set_commands::task_set_start_one,
			task_set_commands::task_set_restart_one,
			task_set_commands::task_set_stop_one,
		])
		.events(collect_events![
			terminal_events::TerminalDeletedEvent,
			terminal_events::TerminalCreatedEvent,
			terminal_events::TerminalStatusChangedEvent,
			terminal_events::TerminalShellReadEvent,
		]);

	#[cfg(debug_assertions)]
	builder
		.export(Typescript::default(), "../utils/tauriBindings.ts")
		.expect("Failed to export typescript bindings");

	tauri::Builder::default()
		.plugin(tauri_plugin_single_instance::init(|app, _, _| {
			let _ = app.get_webview_window("main").expect("no main window").set_focus();
		}))
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
