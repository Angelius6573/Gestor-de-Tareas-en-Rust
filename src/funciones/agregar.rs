use crate::utilidades;
use crate::utilidades::archivo::ARCHIVO;
use crate::utilidades::json::{Agregado, escribir_json, leer_json};

pub fn agregar() {
    println!("Ingrese el nombre de la tarea: ");
    /*
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Fail");
    let name: String = input.trim().parse().expect("Fail");
    */
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

    let id_tarea = utilidades::id_random::id(5);
    let fecha_ahora = utilidades::fecha::fecha_en_momento();

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
