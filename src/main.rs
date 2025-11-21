/*
// use chrono::prelude::*;
// use randomizer::Randomizer;
// use serde::{Deserialize, Serialize};

// use once_cell::sync::Lazy;
// use std::sync::Mutex;

// use funciones::agregar::agregar;
// use utilidades::json::{Agregado, escribir_json, leer_json};
*/
use std::process;
mod funciones;
mod utilidades;
/*
// <IM> Este #... curiosamente si es necesario jajajaja
#[derive(Serialize, Deserialize)]
struct Agregado {
    nombre: String,
    prioridad: i8,
    fecha: String,
    id: String,
} // Estructura para llenar archivos .json
*/
// static ARCHIVO: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::from("waoss.json")));

fn main() {
    loop {
        println!(
            "1. Agregar
2. Listar
3. Eliminar 
4. Cargar 
5. Cerrar
Ingrese su opción por favor: "
        );
        let valor: i8 = utilidades::entradas::std_input_iocho(5);
        /*
        loop {
            let mut input = String::new();

            io::stdin().read_line(&mut input).expect("Fail");

            match input.trim().parse::<i8>() {
                Ok(v) if (1..=6).contains(&v) => {
                    valor = v;
                    break;
                }
                _ => {
                    println!("Valor inválido, ingresa uno entre 1 y 6:");
                }
            }
        }
        */
        interfaz(valor);
    }
}
/*
fn std_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Fail");
    let salida: String = input.trim().parse().expect("Fail");

    salida
}


fn std_input_iocho(limit: i8) -> i8 {
    let variable: i8;
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Fail");
        match input.trim().parse::<i8>() {
            Ok(p) if (1..=limit).contains(&p) => {
                variable = p;
                break;
            }
            _ => {
                println!("Valor no válido.")
            }
        }
    }
    variable
}

*/
fn interfaz(accion: i8) {
    match accion {
        1 => funciones::agregar::agregar(),
        2 => funciones::listar::listar(),
        3 => funciones::eliminar::eliminar(),
        4 => funciones::cargar::cargar(),
        5 => cerrar(),
        _ => unreachable!("Valor fuera de rango."),
    }
}

/*
/*
fn leer_json(ruta: &str) -> Vec<Agregado> {
    let mut archivo = match File::open(ruta) {
        Ok(f) => f,
        Err(_) => return vec![],
    };

    let mut contenido = String::new();
    archivo.read_to_string(&mut contenido).unwrap();

    serde_json::from_str(&contenido).unwrap_or_else(|_| vec![])
}
fn escribir_json(ruta: &str, data: &Vec<Agregado>) {
    let json_string = serde_json::to_string_pretty(data).unwrap();

    let mut archivo = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(ruta)
        .unwrap();

    archivo.write_all(json_string.as_bytes()).unwrap();
}
*/

/*
fn fecha_en_momento() -> String {
    let current_local: DateTime<Local> = Local::now();
    let custom_format = current_local.format("%d/%m/%y");

    custom_format.to_string()
}
fn id(tamaño: usize) -> String {
    let str_ret = Randomizer::ALPHANUMERIC(tamaño).string().unwrap();
    str_ret.to_string()
}

// El sistema de agregación esta terminado
fn agregar() {
    println!("Ingrese el nombre de la tarea: ");
    /*let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Fail");
    let name: String = input.trim().parse().expect("Fail");*/
    let name = utilidades::entradas::std_input();

    print!(
        "Ingrese el nivel de prioridad:
1. Baja
2. Media
3. Alta
"
    );
    let _priority: i8 = utilidades::entradas::std_input_iocho(3);
    /*
    loop {
        let mut waos = String::new();
        io::stdin().read_line(&mut waos).expect("Fail");
        match waos.trim().parse::<i8>() {
            Ok(p) if (1..=3).contains(&p) => {
                _priority = p;
                break;
            }
            _ => {
                println!("Valor no válido para la prioridad");
            }
        }
    }
    */

    let id_tarea = id(5);
    let fecha_ahora = fecha_en_momento();

    let datos = Agregado {
        nombre: name,
        prioridad: _priority,
        fecha: fecha_ahora,
        id: id_tarea,
    };

    {
        let ruta = ARCHIVO.lock().unwrap(); // <IM> Uso de una variable global
        let mut lista = leer_json(&ruta);
        lista.push(datos);
        escribir_json(&ruta, &lista);
    }

    /*
    //let json_string = serde_json::to_string_pretty(&datos).unwrap();
    //let mut archivo = File::create("waoss.json").unwrap();
    //archivo.write_all(json_string.as_bytes()).unwrap();
    // <IM> Esto sobreescribe el contenido del json anterior
     */

    println!("\n");
}
*/

/*
// Listado esta terminado
fn listar() {
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
            println!("Regresando a menú...");
            break;
        }
    }
    /*
        loop {
            println!(
                "¿Qué desea hacer?
1. Todas las tareas
2. ID
3. Regresar al menú"
            );
            let seleccion = std_input().to_string();
            let mut _selection: i8 = seleccion.parse().expect("No es un string");
            loop {
                let mut input = String::new();

                io::stdin().read_line(&mut input).expect("Fail");

                match input.trim().parse::<i8>() {
                    Ok(v) if (1..=3).contains(&v) => {
                        _selection = v;
                        break;
                    }
                    _ => {
                        println!("Valor inválido, ingresa uno entre 1 y 3:");
                    }
                }
            }
            if _selection == 1 {
                let lista = leer_json("waoss.json");

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
            }
            if _selection == 2 {
                println!("Todas");
            }
            if _selection == 3 {
                break;
            }
            break;
        }
        //let id_consultado = std_input();

        /*
            let lista = leer_json("waoss.json");

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

            //println!("listar {}", id_consultado);
        */
    */
}
*/

/*
// Eliminación Finalizada
fn eliminar() {
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
*/

/*
fn cargar() {
    println!(
        "Ingrese la opción que desea elegir:
1. Cargar archivo desde la raíz del proyecto o ingrese la ruta completa
2. Cancelar"
    );
    let eleccion = utilidades::entradas::std_input_iocho(2);
    println!("\n");
    if eleccion == 1 {
        let ruta = &utilidades::entradas::std_input();
        // let mut new_rute = utilidades::json::leer_json(ruta);
        {
            let mut ruta_modificada = ARCHIVO.lock().unwrap();
            ruta_modificada.clear();
            ruta_modificada.push_str(ruta);
        }
    }
}
*/
*/

fn cerrar() {
    println!("Finalizando programa...");
    process::exit(1);
}
