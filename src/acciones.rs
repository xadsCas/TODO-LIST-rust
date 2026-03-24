//conexion BD
use rusqlite::{Connection, Result};


//Insertamos en la bd
pub fn insertar(conn: &Connection, tarea: String) -> Result<()> {
    conn.execute(
        "INSERT INTO tareas (titulo) VALUES (?1)",
        [&tarea],
    )?;

    Ok(())
}

//Mostramos todo en la bd
pub fn mostrar(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, titulo, completado FROM tareas")?;

    let tareas = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, i32>(2)?,
        ))
    })?;

    for tarea in tareas {
        let (id, nombre, completado) = tarea?;

        if completado == 1 {
            println!("{} - [x] {}", id, nombre);
        } else {
            println!("{} - [ ] {}", id, nombre);
        }
    }

    Ok(())
}

//eliminar segun id
pub fn eliminar(conn: &Connection, id: u32) -> Result<()> {
    conn.execute(
        "DELETE FROM tareas WHERE id = ?1",
        [id],
    )?;

    Ok(())
}

//DONE
pub fn marcar_completado(conn: &Connection, id: u32) -> Result<()> {
    let filas = conn.execute(
        "UPDATE tareas SET completado = 1 WHERE id = ?1",
        [id],
    )?;

    if filas == 0 {
        println!("No se encontró la tarea {}", id);
    } else {
        println!("Tarea {} marcada como completada", id);
    }

    Ok(())
}