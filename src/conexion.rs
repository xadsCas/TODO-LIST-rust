//importamos dependencias de sqlite
use rusqlite::{Connection, Result};

//Conexion a la base de datos sqlite
pub fn conectar() -> Result<Connection> {
    let conn = Connection::open("todo.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tareas (
            id INTEGER PRIMARY KEY,
            titulo TEXT NOT NULL,
            completado INTEGER NOT NULL default 0
        )",
        [],
    )?;

    Ok(conn)
}


