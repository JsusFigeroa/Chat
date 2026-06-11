use std::io;

pub fn get_port() -> u16 {
    println!("Escribe el puerto del servidor");
    println!("Presiona enter al terminar");
    println!("En caso de entrada invalida se utilizara por defecto el puerto 8080");
    let mut response = String::new();
    io::stdin()
        .read_line(&mut response)
        .expect("Error al leer la línea");
    if response.len() < 4 {
        return 8080;
    }
    let port: u16 = response.parse().unwrap_or_else(|_| 8080);
    port
}
