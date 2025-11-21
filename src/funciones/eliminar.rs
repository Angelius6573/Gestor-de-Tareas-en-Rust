use crate::utilidades;
use crate::utilidades::archivo::ARCHIVO;
use utilidades::json::{escribir_json, leer_json};

pub fn eliminar() {
    loop {
        let ruta = ARCHIVO.lock().unwrap();
        let mut datos = leer_json(&ruta);

        println!("IDs disponibles: ");
        for (i, item) in datos.iter().enumerate() {
            println!("{}. {}", i + 1, item.id);
        }

        println!("Para cancelar presione \'X\' \n");

        println!("Seleccione el ID que desea eliminar: ");
        let id_eliminar = utilidades::entradas::std_input();

        if let Some(pos) = datos.iter().position(|x| x.id == id_eliminar) {
            datos.remove(pos);
            escribir_json(&ruta, &datos);
            println!("Tarea eliminada con éxito");
        } else if id_eliminar == "X" {
            break;
        } else {
            println!("ID no encontrado.")
        }
    }
}
