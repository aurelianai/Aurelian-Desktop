use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::persistence::schema::chats)]
pub struct Chat {
	pub id: i32,
	pub title: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::persistence::schema::chats)]
pub struct NewChat {
	pub title: String,
}

use crate::AppState;
use tauri::State;

#[tauri::command]
pub fn get_chats(state: State<AppState>) -> Result<Vec<Chat>, String> {
	use crate::persistence::schema::chats::dsl::chats;

	let Some(ref mut connection) = *state.db.lock().unwrap() else { panic!() };

	chats
		.select(Chat::as_select())
		.load(connection)
		.map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub fn insert_chat(state: State<AppState>, new_chat: NewChat) -> Result<i32, String> {
	use crate::persistence::schema::chats;
	use crate::persistence::schema::chats::dsl::*;

	let Some(ref mut connection) = *state.db.lock().unwrap() else { panic!() };

	diesel::insert_into(chats::table)
		.values(&new_chat)
		.returning(id)
		.get_result(connection)
		.map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_chat(state: State<AppState>, c: Chat) -> Result<usize, String> {
	use crate::persistence::schema::chats::dsl::*;

	let Some(ref mut connection) = *state.db.lock().unwrap() else { panic!() };

	diesel::update(chats)
		.filter(id.eq(c.id))
		.set(title.eq(c.title))
		.execute(connection)
		.map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_chat(state: State<AppState>, pkey: i32) -> Result<usize, String> {
	use crate::persistence::schema::chats::dsl::*;

	let Some(ref mut connection) = *state.db.lock().unwrap() else { panic!() };

	diesel::delete(chats.filter(id.eq(pkey)))
		.execute(connection)
		.map_err(|e| e.to_string())
}
