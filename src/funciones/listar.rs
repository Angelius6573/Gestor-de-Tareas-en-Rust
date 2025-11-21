use crate::utilidades;
use crate::utilidades::archivo::ARCHIVO;
use crate::utilidades::json::leer_json;

pub fn listar() {
    loop {
        println!(
            "\n ¿Qué desea hacer?
1. Todas las tareas
2. ID
3. Regresar al menú"
        );
        let ruta = ARCHIVO.lock().unwrap();
        let ver: i8 = utilidades::entradas::std_input_iocho(3);
        let lista = leer_json(&ruta);

        if ver == 1 {
            if lista.is_empty() {
                println!("No hay datos");
                return;
            }
            for (i, item) in lista.iter().enumerate() {
                println!("\n Registro #{}", i + 1);
                println!("Nombre: {}", item.nombre);
                println!("prioridad:  {}", item.prioridad);
                println!("Fecha de creación: {}", item.fecha);
                println!("ID: {}", item.id);
            }
        } else if ver == 2 {
            if lista.is_empty() {
                println!("No hay datos");
                return;
            }
            for (i, item) in lista.iter().enumerate() {
                println!("\n Registro #{}", i + 1);
                println!("ID: {}", item.id);
            }
        } else {
            println!("Regresan a menú...");
            break;
        }
    }
}
