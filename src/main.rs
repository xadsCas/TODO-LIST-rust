//importamos modulos
mod conexion;
use clap::{Parser, Subcommand};
use conexion::conectar;
mod acciones;
use acciones::eliminar;
use acciones::insertar;
use acciones::mostrar;
use acciones::marcar_completado;
//implementamos derive parser a una estrucutra
#[derive(Parser)]
struct Args{

    #[command(subcommand)]
   comando:Comandos,
}

#[derive(Subcommand)]
enum Comandos{
    //comandos en cli
    List,
    Add{tarea: String},
    Delete{id:u32},
    Done {id:u32},
}
//funcion principal
fn main() {
    //conexion a base de datos + variables cli
    let conn = conectar().expect("Error al conectar a la base de datos");
    let cli= Args::parse();

    //subcomando
    match cli.comando{
        Comandos::Add{tarea}=>{
            insertar(&conn, tarea).expect("Error al añadir la tarea");
            println!("Tarea agregada");
        }
        Comandos::List=>{
            println!("Listando tareas");
            mostrar(&conn).expect("Error al mostrar");
            
        }
        Comandos::Delete { id }=>{
            eliminar(&conn, id).expect("Error al eliminar");
            println!("ELiminando tarea {}",id);
        }
        Comandos::Done { id }=>{
            marcar_completado(&conn, id).expect("Error insano paso");

        }

    }
}
