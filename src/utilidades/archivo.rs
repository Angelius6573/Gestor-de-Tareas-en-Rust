use once_cell::sync::Lazy;
use std::sync::Mutex;

pub static ARCHIVO: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::from("waoss.json")));
