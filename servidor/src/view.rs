use std::net::Ipv4Addr;
use std::io;


pub fn get_addr() -> (Ipv4Addr, u16) {
    println!("Escribe la dirección y el puerto del servidor en formato 127 0 0 1 4444 para
    definir la dirección 127.0.0.1 y el puerto 4444");
    println!("Presiona enter al terminar");
    println!("En caso de entrada inválida o vacía se establecera al servidor con la 
    dirección 127.0.0.1 y el puerto 4444");
    let mut response = String::new();
    io::stdin().read_line(&mut response).expect("Error al leer la línea");
    let parts: Vec<&str> = response.split_whitespace().collect();
    match verify_adress(parts) {
        Ok((ip, port)) => {
            println!("Configurando al servidor en la dirección {} y puerto {}", ip, port);
            (ip, port)
        }
        Err(_) => {
            println!("Configurando al servidor en la dirección por defecto");
            let ip = Ipv4Addr::new(127, 0, 0, 1);
            let port = 4444;
            (ip, port)
        }
    }
}

fn verify_adress(parts: Vec<&str>) -> Result<(Ipv4Addr, u16), ()>{
    if parts.len() != 5 {
        return Err(())
    }
    let a: u8 = parts[0].parse().map_err(|_| ())?;
    let b: u8 = parts[1].parse().map_err(|_| ())?;
    let c: u8 = parts[2].parse().map_err(|_| ())?;
    let d: u8 = parts[3].trim().parse().map_err(|_| ())?;
    let port: u16 = parts[4].parse().map_err(|_| ())?;
    let addres = Ipv4Addr::new(a, b, c, d);
    Ok((addres, port))
}