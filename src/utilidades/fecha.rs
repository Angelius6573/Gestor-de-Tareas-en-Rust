use chrono::prelude::*;

pub fn fecha_en_momento() -> String {
    let current_local: DateTime<Local> = Local::now();
    let custom_format = current_local.format("%d/%m/%y");

    custom_format.to_string()
}
