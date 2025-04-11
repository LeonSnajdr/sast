use once_cell::sync::OnceCell;
use sqlx::{sqlite::SqliteConnectOptions, Pool, Sqlite, SqlitePool};
use std::fs::create_dir_all;
use std::str::FromStr;
use tauri::{
	plugin::{Builder as PluginBuilder, TauriPlugin},
	Manager, Runtime,
};

static POOL: OnceCell<Pool<Sqlite>> = OnceCell::new();

fn build_data_dir<R: Runtime>(app: &tauri::AppHandle<R>) -> Option<String> {
	if cfg!(debug_assertions) {
		log::info!("Running in debug mode, using 'dev.db' as data directory");
		return Some("dev.db".to_string());
	}

	let mut data_dir = match app.path().app_data_dir().ok() {
		Some(dir) => dir,
		None => {
			log::error!("Could not obtain application data directory, returning None");
			return None;
		}
	};

	if let Err(err) = create_dir_all(&data_dir) {
		log::error!("Failed to create application data directory: {}", err);
		return None;
	}

	data_dir.push("sast.db");
	let data_dir_str = match data_dir.to_str() {
		Some(p) => p.to_string(),
		None => {
			log::error!("Failed to convert path to string for the database directory");
			return None;
		}
	};

	log::info!("Using database directory: {}", data_dir_str);
	Some(data_dir_str)
}

pub fn init_sqlx<R: Runtime>() -> TauriPlugin<R> {
	PluginBuilder::new("sqlx")
		.setup(|app, _| {
			tauri::async_runtime::block_on(async move {
				log::info!("Initializing database...");

				let data_dir_str = match build_data_dir(app) {
					Some(dir) => dir,
					None => {
						log::error!("Failed to build data directory for the database");
						return Err("Unknown application data path")?;
					}
				};

				log::debug!("Data directory for database is: {}", data_dir_str);

				let config = match SqliteConnectOptions::from_str(&data_dir_str) {
					Ok(opts) => opts.create_if_missing(true),
					Err(err) => {
						log::error!("Failed to parse SqliteConnectOptions: {}", err);
						return Err("Invalid database configuration")?;
					}
				};

				let pool = match SqlitePool::connect_with(config).await {
					Ok(p) => p,
					Err(err) => {
						log::error!("Failed to connect to database: {}", err);
						return Err("Could not connect to the database")?;
					}
				};

				log::info!("Successfully connected to the database. Running migrations...");

				if let Err(err) = sqlx::migrate!().run(&pool).await {
					log::error!("Failed to run database migrations: {}", err);
					return Err("Database migration failed")?;
				}

				log::info!("Migrations completed successfully");

				if POOL.set(pool).is_err() {
					log::error!("Database pool was already set.");
					return Err("Database pool initialization conflict")?;
				}

				log::info!("Database initialization completed successfully");
				Ok(())
			})
		})
		.build()
}

pub fn get_pool() -> &'static Pool<Sqlite> {
	POOL.get().expect("Database not initialized; call init_sqlx first")
}
