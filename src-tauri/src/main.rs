// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod inference;
use core::time::Duration;
use persistence::*;
use std::collections::HashMap;
use std::fs::{create_dir_all, File};
use tauri::api::process::{Command, CommandEvent};
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .manage(DatabaseConnectionState {
            conn: Default::default(),
        })
        .invoke_handler(tauri::generate_handler![check_server_health])
        .setup(setup)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let mut app_data_dir = app.path_resolver().app_data_dir().unwrap();
    app_data_dir.push("aurelian.db");
    if !std::path::Path::new(&app_data_dir).exists() {
        create_dir_all(app_data_dir.clone())?;
        File::create(app_data_dir)?;
    }

    connect_db(app.state::<DatabaseConnectionState>(), app.app_handle()).unwrap();

    let state = app.state::<DatabaseConnectionState>();
    let Some(ref mut connection) = *state.conn.lock().unwrap() else { panic!() };
    run_migrations(connection).unwrap();

    start_server();

    Ok(())
}
