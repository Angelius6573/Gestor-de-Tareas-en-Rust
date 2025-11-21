use std::io;

pub fn std_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Fail");
    let salida: String = input.trim().parse().expect("Fail");

    salida
}

pub fn std_input_iocho(limit: i8) -> i8 {
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
