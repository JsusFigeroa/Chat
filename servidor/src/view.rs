use std::io;

#[must_use]
pub fn get_port() -> u16 {
    println!("Escribe el puerto del servidor");
    println!("Presiona enter al terminar");
    println!("En caso de entrada invalida se utilizara por defecto el puerto 8080");
    let mut response = String::new();
    let Ok(_) = io::stdin().read_line(&mut response) else {
        return 8080;
    };

    if response.len() < 4 {
        return 8080;
    }
    let Ok(port) = response.parse() else {
        return 8080;
    };
    port
}
