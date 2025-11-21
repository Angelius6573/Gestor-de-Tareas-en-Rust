use crate::utilidades;
use crate::utilidades::archivo::ARCHIVO;

pub fn cargar() {
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
