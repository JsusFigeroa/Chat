use tokio::sync::mpsc::{Sender, Receiver};
use tokio::net::tcp::OwnedWriteHalf;
use tokio::net::{TcpStream};
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader};
use crate::controller::controller_aux::generate_identify;
use crate::type_receive_message::TypeReciveMesagges;
use crate::view::{self, get_username, retry_get_username};
use tokio::sync::mpsc::channel;
mod controller_aux;

pub struct Controller<T> {
    rx: Receiver<T>,
    tx: Sender<T>
}

impl<T> Controller<T> {
    fn new(rx: Receiver<T>, tx: Sender<T>) -> Controller<T> {
        Controller { rx, tx }
    }
}

/// Función que establece la conexión con un servidor y hace la identificación correspondiente, por último
/// si todo lo anterior resulto exitoso, manda a llamar una función que ejecuta el resto de funcionalidades
/// del cliente.
pub async  fn start() {
    view::clear_shell();
    let addr = view::get_addr();
    let socket = TcpStream::connect(addr).await.expect("Error al conectarse al servidor");
    let (sok_reader, mut sok_writer) = socket.into_split();
    let mut username = get_username().expect("El nombre de usuario ingresado es inválido");
    send_identifier(username, &mut sok_writer).await;
    let mut buf_reader = BufReader::new(sok_reader);
    match get_identify_response(&mut buf_reader).await {

        Ok(a) => {view::print_succes_identify(a);}

        Err(_) => {
            username = retry_get_username();
            send_identifier(username, &mut sok_writer).await;
            username = get_identify_response(&mut buf_reader).await.unwrap_or_else(|_| {panic!("El nombre elegido no es válido")} );
            view::print_succes_identify(username);
        }
    }

    //Hacer la función que enviará mensajes al servidor
    //Hacer transmisor para enviar mensajes al servidor
    //Hacer ciclo para obtener entrada del usuario


}

/// Manda a llamar las funciones respectivas para recibir mensajes del servidor y entrada del usuario, a su vez 
/// llama a las funciones que procesan estas solicitudes.
// La función que recibe mensajes del server
async fn work<T: AsyncRead + Unpin, R: AsyncWrite + Unpin>(mut reader:  T, mut writer: R, tx: Sender<Vec<u8>>) {
    let (msg_server_tx, mut msg_server_rx) = channel::<String>(100);
    let (msg_user_tx, mut msg_user_rx) = channel::<String>(100);
    let tx_for_disconect = tx.clone();
    tokio::spawn(async move {
        get_server_msg(msg_server_tx).await;
    });
    tokio::spawn(async move {
        view::get_usr_entry(msg_user_tx).await;
    });

    loop {
        tokio::select! {
            Some(msg) = msg_server_rx.recv() => {
                procces_server_msg(msg).await;
            }

            Some(msg) = msg_user_rx.recv() => {
                procces_user_msg(msg).await;
            }

            _ = tokio::signal::ctrl_c() => {
                //Mandar mensaje de desconexión
                //Avisar al usuario que ha sido desconectado
                break;
            }
        }
    }
}

async fn get_server_msg<T>(tx: Sender<T>) {

}

async fn procces_server_msg(message: String) {

}

async fn procces_user_msg(message: String) {

}


async fn send_identifier(name: String, socket: &mut OwnedWriteHalf) {
    let id_message = generate_identify(name);
    socket.write_all(&id_message).await.unwrap_or_default();
}

async fn get_identify_response<T: AsyncRead + Unpin>(socket: &mut BufReader<T>) -> Result<String, ()> {
    let mut response = String::new();
    let bytes = socket.read_line(&mut response).await.expect("No fue posible leer datos del socket");
    if bytes == 0 {
        panic!("La conexión se cerró")
    }
    
    match serde_json::from_str::<TypeReciveMesagges>(&response).expect("El mensaje recibido no era válido") {
        TypeReciveMesagges::Response { type_msg, operation, result, extra } => {
            if type_msg != "RESPONSE" {
                panic!("El mensaje no coincide con el protocolo")
            }
            if operation != "IDENTIFY" {
                panic!("El mensaje no coincide con el protocolo")
            }
            if result != "SUCCES" {
                if result == "USER_ALREADY_EXISTS" {
                    Err(())
                }
                else {
                    panic!("EL mensaje no coincide con el protocolo")
                }
            }
            else {
                Ok(extra)
            }
        }
        _ => panic!("La respuesta no es la esperada según el protocolo")
        
    }

}

async fn recieve_msg<R: AsyncRead + Unpin>(tx: Sender<TypeReciveMesagges>, mut reader: BufReader<R>) {
    let mut line = String::new();
    let result = reader.read_line(&mut line).await;
    loop {
        let Ok(bytes) = result else {
            // La vista tiene que imprimir que se desconectó el servidor.
            break; 
        };
        if bytes == 0 {
            // La vista tiene que imprimir que el servidor cerró su conexión.
            break;
        }
        let Ok(message) = serde_json::from_str::<TypeReciveMesagges>(&line) else {
            eprint!("El mensaje no coincide con el protocolo");
            break;
        };
        tx.send(message);
    }
}
///Función que recibe mensajes del servidor y se encarga de procesar cada uno.
async fn process_msg_from_server<T>(server_rx: Receiver<TypeReciveMesagges>) {
    
}

///Función que se encarga de procesar mensajes o peticiones provenientes del usuario.
async fn process_msg_from_client<T>(client_rx: Receiver<T>, tx: Sender<Vec<u8>>) {
    
}