use randomizer::Randomizer;

pub fn id(tamaño: usize) -> String {
    let str_ret = Randomizer::ALPHANUMERIC(tamaño).string().unwrap();
    str_ret.to_string()
}
