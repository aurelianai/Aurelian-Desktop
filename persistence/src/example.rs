/// Holds Model Definitions and Tauri commands for doing CRUD ops the db 
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::servers)]
pub struct ServerDetails {
    pub id: i32,
    pub url: String,
    pub name: String,
    pub description: String,
    pub auth_type: String,
    pub auth_payload: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::servers)]
pub struct NewServer {
    pub url: String,
    pub name: String,
    pub description: String,
    pub auth_type: String,
    pub auth_payload: String,
}

use crate::DatabaseConnectionState;
use tauri::{command, State};

#[command]
/// Gets all servers in servers table.
pub fn get_servers(state: State<DatabaseConnectionState>) -> Result<Vec<ServerDetails>, String> {
    use crate::schema::servers::dsl::servers;

    let Some(ref mut connection) = *state.conn.lock().unwrap() else { panic!() };
    servers
        .select(ServerDetails::as_select())
        .load(connection)
        .map_err(|e| e.to_string())
}

#[command]
/// Adds Server to Database and returns it's assigned primary key.
pub fn insert_server(state: State<DatabaseConnectionState>, s: NewServer) -> Result<i32, String> {
    use crate::schema::servers;
    use crate::schema::servers::dsl::*;

    let Some(ref mut connection) = *state.conn.lock().unwrap() else { panic!() };
    diesel::insert_into(servers::table)
        .values(&s)
        .returning(id)
        .get_result(connection)
        .map_err(|e| e.to_string())
}

#[command]
/// Deletes server row by pkey
pub fn delete_server(state: State<DatabaseConnectionState>, pkey: i32) -> Result<usize, String> {
    use crate::schema::servers::dsl::*;

    let Some(ref mut connection) = *state.conn.lock().unwrap() else { panic!() };
    diesel::delete(servers.filter(id.eq(pkey)))
        .execute(connection)
        .map_err(|e| e.to_string())
}
