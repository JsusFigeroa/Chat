use std::{io, net::Ipv4Addr, sync::Arc};
use servidor::server::Server;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    println!("Escribe la dirección y el puerto del servidor en formato 127 0 0 1 4444 para
    definir la dirección 127.0.0.1 y el puerto 4444");
    println!("Presiona enter al terminar");
    println!("En caso de entrada inválida o vacía se establecera al servidor con la 
    dirección 127.0.0.1 y el puerto 4444");

    let mut response = String::new();
    io::stdin().read_line(&mut response).expect("Error al leer la línea");
    let parts: Vec<&str> = response.split_whitespace().collect();

    if parts.len() != 5 {
        println!("Configurando servidor en la dirección y puerto por defecto");
        let server = Arc::new(Mutex::new(Server::new_default()));
        Server::run(server).await;
    }
    else {
        let (adress, port) = verify_adress(parts).unwrap_or_else(|_| {
                println!("Configurando el servidor por defecto");
                (Ipv4Addr::new(127, 0, 0, 1), 4444)
            }
        );
        let server = Arc::new(Mutex::new(Server::new(adress, port)));
        Server::run(server).await;
    }
    

}

fn verify_adress(parts: Vec<&str>) -> Result<(Ipv4Addr, u16), std::num::ParseIntError>{
    let a: u8 = parts[0].parse()?;
    let b: u8 = parts[1].parse()?;
    let c: u8 = parts[2].parse()?;
    let d: u8 = parts[3].trim().parse()?;
    let port: u16 = parts[4].parse()?;
    let addres = Ipv4Addr::new(a, b, c, d);
    Ok((addres, port))
}
