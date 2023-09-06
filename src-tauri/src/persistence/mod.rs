pub mod chat;
pub mod message;
mod schema;

use crate::DB;
use diesel::prelude::*;
use diesel::sqlite::Sqlite;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use tauri::{AppHandle, State};

/// Creates a [DatabaseConnection] to be managed by [tauri::State]. This is called in the setup function in the main Aurelian-Desktop crate `main.rs`
/// This must return a Result see https://github.com/tauri-apps/tauri/issues/2533
pub fn connect_db(state: State<'_, DB>, app_handle: AppHandle) -> Result<(), ()> {
	// Panics if mutex is poisoned
	let db_url = get_conn_string(app_handle);
	let mut db_conn_ptr = state.0.lock().unwrap();
	*db_conn_ptr = Some(SqliteConnection::establish(&db_url).unwrap());
	Ok(())
}

/// Helper to Get sqlite db connection string from [tauri::AppHandle]
pub fn get_conn_string(app_handle: AppHandle) -> String {
	let mut app_data_dir = app_handle.path_resolver().app_data_dir().unwrap();
	app_data_dir.push("aurelian.db");
	return String::from(app_data_dir.to_str().unwrap());
}

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
/// Run pending migrations against the local database
pub fn run_migrations(
	connection: &mut impl MigrationHarness<Sqlite>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
	connection.run_pending_migrations(MIGRATIONS).unwrap();
	Ok(())
}
