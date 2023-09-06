// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod inference;
mod persistence;

use crate::inference::*;
use crate::persistence::{chat::*, message::*, *};
use diesel::SqliteConnection;
use std::fs::{create_dir_all, File};
use std::sync::Mutex;
use tauri::Manager;

fn main() {
	tauri::Builder::default()
		.manage(DB(Mutex::new(None)))
		.manage(ModelState(Mutex::new(None)))
		.invoke_handler(tauri::generate_handler![
			get_chats,
			insert_chat,
			update_chat,
			delete_chat,
			get_messages,
			insert_message,
			update_message,
			load_default_model,
			complete,
		])
		.setup(setup)
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}

/// Setup function called before app starts
/// Ensures $APPDATA exists and contains `aurelian.db`
/// Initializes db connection in [AppState] and runs any pending migrations
fn setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
	let mut app_data_dir = app.path_resolver().app_data_dir().unwrap();
	create_dir_all(app_data_dir.clone())?;

	app_data_dir.push("aurelian.db");
	if !std::path::Path::new(&app_data_dir).exists() {
		File::create(app_data_dir)?;
	}

	connect_db(app.state::<DB>(), app.app_handle()).unwrap();

	let state = app.state::<DB>();
	let Some(ref mut connection) = *state.0.lock().unwrap() else {
		panic!()
	};
	run_migrations(connection).unwrap();

	Ok(())
}

pub struct DB(Mutex<Option<SqliteConnection>>);
pub struct ModelState(Mutex<Option<ModelManager>>);
