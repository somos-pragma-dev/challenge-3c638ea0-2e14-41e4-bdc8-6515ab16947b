use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use std::env;

pub fn establish_connection(database_url: &str) -> SqliteConnection {
SqliteConnection::establish(database_url)
.expect(&format!("Error al conectar a la base de datos en {}", database_url))
}