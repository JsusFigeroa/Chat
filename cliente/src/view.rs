use std::net::{Ipv4Addr, SocketAddrV4};
use std::io;



pub fn get_addr() -> SocketAddrV4 {
    println!("Ingrese la dirección y puerto en que se conectará");
    println!("Ingresalo separado por espacios de la siguiente forma:");
    println!("127 0 0 0 4444");
    println!("La entrada anterior conecta al servidor en la dirección y puerto:");
    println!("127.0.0.1:4444");
    println!("En caso de entrada invalida se establecerá por defecto en:");
    println!("127.0.0.1:4444");
    let mut args = String::new();
    io::stdin().read_line(&mut args).expect("No fue posible obtener la dirección");
    let addr = aux_get_addr(args).unwrap_or_else(|_| {
        let ip = Ipv4Addr::new(127, 0, 0, 1);
        let port = 4444;
        let addr = SocketAddrV4::new(ip, port);
        addr
    });
    addr
}

pub fn get_username() -> Result<String, ()> {
    println!("Escribe tu nombre de usuario, recuerda que debe ser de máximo 8 caracteres");
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Error el obtener nombre de usuario");
    if username.len() > 8 {
        println!("El nombre de usuario debe ser menor a 8 caracteres");
        println!("Escribe tu nombre de usuario");
        username.clear();
        io::stdin().read_line(&mut username).expect("Error el obtener nombre de usuario");
        if username.len() > 8 {
            Err(())
        }
        else {
            Ok(username)
        }
    }
    else {
        Ok(username)
    }
}

pub fn retry_get_username() -> String {
    println!("El nombre selecionado ya está en uso");
    get_username().expect("El nombre de usuario no es válido")
}

pub fn print_succes_identify(username: String) {
    println!("Entraste al chat con el nombre {}", username);
}

fn aux_get_addr(args: String) -> Result<SocketAddrV4, ()> {
    let arg: Vec<&str> = args.split_whitespace().collect();
    if arg.len() != 5 {
        ()
    }
    let a: u8 = arg[0].parse().map_err(|_| ())?;
    let b: u8 = arg[1].parse().map_err(|_| ())?;
    let c: u8 = arg[2].parse().map_err(|_| ())?;
    let d: u8 = arg[3].parse().map_err(|_| ())?;
    let port: u16 = arg[4].parse().map_err(|_| ())?;
    let ip = Ipv4Addr::new(a, b, c, d);
    let addr = SocketAddrV4::new(ip, port);
    Ok(addr)

}