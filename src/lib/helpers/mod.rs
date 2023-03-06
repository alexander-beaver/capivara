pub fn get_copyright() -> String{
    return String::from("(c) 2023 Alexander Beaver\
    \n \
    https://alexbeaver.com");
}

pub fn modulo(a: u16, b: u16) -> u16{
    return (a % b + b) % b;
}