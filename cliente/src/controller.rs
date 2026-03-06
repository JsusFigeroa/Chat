use tokio::net::tcp::OwnedWriteHalf;
use tokio::net::{TcpStream};
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncWriteExt, BufReader};

use crate::controller::controller_aux::generate_identify;
use crate::type_receive_message::TypeReciveMesagges;
use crate::view::{self, get_username, retry_get_username};
mod controller_aux;

//
pub async  fn start() {
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