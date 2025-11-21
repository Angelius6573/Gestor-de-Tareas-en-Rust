use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

#[derive(Serialize, Deserialize)]
pub struct Agregado {
    pub nombre: String,
    pub prioridad: i8,
    pub fecha: String,
    pub id: String,
}

pub fn leer_json(ruta: &str) -> Vec<Agregado> {
    let mut archivo = match File::open(ruta) {
        Ok(f) => f,
        Err(_) => return vec![],
    };

    let mut contenido = String::new();
    archivo.read_to_string(&mut contenido).unwrap();

    serde_json::from_str(&contenido).unwrap_or_else(|_| vec![])
}
pub fn escribir_json(ruta: &str, data: &Vec<Agregado>) {
    let json_string = serde_json::to_string_pretty(data).unwrap();

    let mut archivo = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(ruta)
        .unwrap();

    archivo.write_all(json_string.as_bytes()).unwrap();
}
