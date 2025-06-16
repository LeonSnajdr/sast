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

use crate::placeholder::placeholder_commands::*;
use crate::project::project_commands::*;
use crate::setting::setting_commands::*;
use crate::task::task_commands::*;
use crate::task_set::session::task_set_session_commands::*;
use crate::task_set::session::task_set_session_events::*;
use crate::task_set::task_set_commands::*;
use crate::terminal::terminal_commands::*;
use crate::terminal::terminal_events::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	let builder = Builder::<tauri::Wry>::new()
		.commands(collect_commands![
			setting_initialize,
			setting_get_default,
			setting_update_one,
			project_create,
			project_get_all,
			project_get_one,
			project_get_last_opened,
			project_open,
			project_update_one,
			project_delete_one,
			terminal_create,
			terminal_shell_write,
			terminal_get_one_open,
			terminal_get_one_info,
			terminal_replace_history,
			terminal_get_many_info,
			terminal_shell_resize,
			terminal_close,
			placeholder_create,
			placeholder_get_many,
			placeholder_get_one,
			placeholder_update_one,
			placeholder_delete_one,
			task_create,
			task_get_one,
			task_get_many_info,
			task_update_one,
			task_delete_one,
			task_start_one,
			task_restart_one,
			task_stop_one,
			task_set_create,
			task_set_get_one,
			task_set_get_many_info,
			task_set_update_one,
			task_set_delete_one,
			task_set_start_one,
			task_set_restart_one,
			task_set_stop_one,
			task_set_session_get_many
		])
		.events(collect_events![
			TerminalClosedEvent,
			TerminalCreatedEvent,
			TerminalUpdatedEvent,
			TerminalShellStatusChangedEvent,
			TerminalShellReadEvent,
			TaskSetSessionStartedEvent,
			TaskSetSessionFinishedEvent,
			TaskSetSessionTaskStatusChangedEvent
		]);

	#[cfg(debug_assertions)]
	builder
		.export(Typescript::default(), "../utils/tauriBindings.ts")
		.expect("Failed to export typescript bindings");

	tauri::Builder::default()
		.plugin(
			tauri_plugin_log::Builder::new()
				.clear_targets()
				.level(log::LevelFilter::Debug)
				.level_for("sqlx", log::LevelFilter::Warn)
				.target(tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Stdout))
				.target(tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::LogDir {
					file_name: Some("sast".to_string()),
				}))
				.build(),
		)
		.plugin(tauri_plugin_updater::Builder::new().build())
		.plugin(tauri_plugin_single_instance::init(|app, _, _| {
			let _ = app.get_webview_window("main").expect("no main window").set_focus();
		}))
		.plugin(tauri_plugin_process::init())
		.plugin(tauri_plugin_opener::init())
		.invoke_handler(builder.invoke_handler())
		.setup(move |app| {
			builder.mount_events(app);

			app.handle().plugin(db::init_sqlx())?;
			Ok(())
		})
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
