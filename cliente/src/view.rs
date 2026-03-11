use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::io;
use std::io::stdin as stdio;
use std::process::Command;
use tokio::sync::mpsc::Sender;
use serde::{self,Deserialize,Serialize};

pub(super) fn clear_shell() {
    Command::new("clear").status().expect("No fue posible limpiar la terminal");
}

pub(crate)  fn get_addr() -> SocketAddrV4 {
    println!("Ingrese la dirección y puerto en que se conectará con el siguiente formato:");
    println!("127.0.0.1.4444");
    println!("La entrada anterior conecta al servidor en la dirección 127.0.0.1 y el puerto 4444");
    println!("En caso de entrada invalida se establecerá por defecto en localhost y puerto 4444");
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

pub(crate)  fn get_username() -> Result<String, ()> {
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

pub(crate)  fn retry_get_username() -> String {
    println!("El nombre selecionado ya está en uso");
    get_username().expect("El nombre de usuario no es válido")
}

pub(crate)  fn print_succes_identify(username: String) {
    println!("Entraste al chat con el nombre {}", username);
}

fn aux_get_addr(args: String) -> Result<SocketAddrV4, ()> {
    let arg: Vec<&str> = args.split('.').collect();
    if arg.len() != 5 {
        return Err(())
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

pub(crate) async fn get_usr_entry() -> Result<Action, ()>{
    let mut line = String::new();
    stdio().read_line(&mut line).expect("Error al leer de la entrada estándar");
    let line = String::from(line.trim());
    if line.starts_with('/') {
        let args: Vec<&str> = line.split_whitespace().collect();
        match args[0] {
            "/status" => {
                if args.len() < 2 {
                    return Err(())
                }
                match args[1] {
                    "away" => {return Ok(Action::Status { status: Status::Away })}
                    "busy" => {return Ok(Action::Status { status: Status::Busy })}
                    "active" => {return Ok(Action::Status { status: Status::Active })}
                    _ => {return Err(())}
                }
            }
            "/users" => {
                return Ok(Action::Users)
            }
            "/privateText" => {
                if args.len() < 3 {
                    return Err(());
                }
                return Ok(Action::PrivateText { username: String::from(args[1]), text: String::from(args[2]) })
            }
            "/disconnect" => {
                return Ok(Action::Disconnect);
            }
            "/help" => {return Ok(Action::Help)}
            _ => {
                return Err(());
            }
        }
    }
    return Ok(Action::PublicText { text: line });
}

pub(crate) fn print_not_valid_command() {
    unimplemented!()
}

pub(crate)  fn print_help_msg() {
    unimplemented!()
}

pub(crate) fn disconnected_by_server() {
    unimplemented!()
}

pub(crate) fn print_new_status(username: String, status: Status) {
    match status {
        Status::Active => {println!("El estado de {} ahora es Activo", username)}
        Status::Away => {println!("El estado de {} ahora es Inactivo", username)}
        Status::Busy => {println!("El estado de {} ahora es Ocupado", username)}
    }
}

pub(crate) fn print_public_text(username: String, text: String) {
    println!("{}: {}", username, text);
}

pub(crate) fn print_users(map: HashMap<String, Status>) {
    println!("Los usuarios actuales son:");
    for kv in map {
        println!("{}: {:?}", kv.0, kv.1);
    }
}

pub(crate) fn print_private_text(username: String, text: String){
    println!("Mensaje privado de {}: {}", username, text);
}

pub(crate) fn print_invalid_response() {
    println!("El mensaje enviado al servidor fue inválido");
}

pub(crate) fn print_text_response_no_such_usr(username: String) {
    println!("El usuario {} al que se intentó enviar mensaje privado no existe.", username);
}

pub(crate) enum Action {
    Status {
        status: Status
    },
    Users,
    PrivateText {
        username: String,
        text: String
    },
    Disconnect,
    PublicText {
        text: String
    },
    Help
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub(crate) enum Status {
    Away,
    Busy,
    Active
}
