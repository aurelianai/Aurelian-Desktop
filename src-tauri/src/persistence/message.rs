use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::persistence::schema::messages)]
pub struct Message {
	pub role: String,
	pub content: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::persistence::schema::messages)]
pub struct NewMessage {
	pub role: String,
	pub content: String,
	pub chat_id: i32,
}

use crate::DB;
use tauri::State;

#[tauri::command(rename_all = "snake_case")]
pub fn get_messages(state: State<DB>, c_id: i32) -> Result<Vec<Message>, String> {
	use crate::persistence::schema::messages::dsl::*;

	let Some(ref mut connection) = *state.0.lock().unwrap() else { panic!() };

	messages
		.filter(chat_id.eq(c_id))
		.select(Message::as_select())
		.load(connection)
		.map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub fn insert_message(state: State<DB>, new_message: NewMessage) -> Result<usize, String> {
	use crate::persistence::schema::messages;

	let Some(ref mut connection) = *state.0.lock().unwrap() else { panic!() };

	diesel::insert_into(messages::table)
		.values(&new_message)
		.execute(connection)
		.map_err(|e| e.to_string())
}
