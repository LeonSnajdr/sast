use once_cell::sync::OnceCell;
use sqlx::{sqlite::SqliteConnectOptions, Pool, Sqlite, SqlitePool};
use std::fs::create_dir_all;
use std::str::FromStr;
use tauri::{
	plugin::{Builder as PluginBuilder, TauriPlugin},
	Manager, Runtime,
};

pub mod types;

static POOL: OnceCell<Pool<Sqlite>> = OnceCell::new();

fn build_data_dir<R: Runtime>(app: &tauri::AppHandle<R>) -> Option<String> {
	if cfg!(debug_assertions) {
		return Some("dev.db".to_string());
	}

	let mut data_dir = app.path().app_data_dir().ok()?;

	create_dir_all(&data_dir).ok()?;

	data_dir.push("sast.db");

	return Some(data_dir.to_str()?.to_string());
}

pub fn init_sqlx<R: Runtime>() -> TauriPlugin<R> {
	PluginBuilder::new("sqlx")
		.setup(|app, _| {
			tauri::async_runtime::block_on(async move {
				let data_dir_str = build_data_dir(app).ok_or(tauri::Error::UnknownPath)?;

				println!("{}", data_dir_str);

				let config = SqliteConnectOptions::from_str(&data_dir_str)?.create_if_missing(true);

				let pool = SqlitePool::connect_with(config).await?;

				sqlx::migrate!().run(&pool).await?;

				POOL.set(pool).unwrap();

				Ok(())
			})
		})
		.build()
}

pub fn get_pool() -> &'static Pool<Sqlite> {
	let pool = POOL.get().expect("Database not initialized");

	return pool;
}
