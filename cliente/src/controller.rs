use tokio::sync::mpsc::{Sender, Receiver};
use tokio::net::tcp::OwnedWriteHalf;
use tokio::net::{TcpStream};
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader};
use crate::controller::controller_aux::{generate_identify};
use crate::type_receive_message::{OperationType, TypeReciveMesagges};
use crate::type_send_message::TypeSendMessage;
use crate::view::{self, Action, get_username, retry_get_username};
use tokio::sync::mpsc::channel;
mod controller_aux;


/// Función que establece la conexión con un servidor y hace la identificación correspondiente, por último
/// si todo lo anterior resulto exitoso, manda a llamar una función que ejecuta el resto de funcionalidades
/// del cliente.
pub async  fn start() {
    let addr = view::get_addr();
    let socket = TcpStream::connect(addr).await.expect("Error al conectarse al servidor");
    let (sok_reader, mut sok_writer) = socket.into_split();
    let mut username = get_username().expect("El nombre de usuario ingresado es inválido");
    send_identifier(username, &mut sok_writer).await;
    let mut buf_reader = BufReader::new(sok_reader);
    match get_identify_response(&mut buf_reader).await {

        Ok(a) => {view::print_succes_identify(a);}

        Err(name) => {
            username = retry_get_username(&name);
            send_identifier(username, &mut sok_writer).await;
            username = get_identify_response(&mut buf_reader).await.unwrap_or_else(|_| {panic!("El nombre elegido no es válido")} );
            view::print_succes_identify(username);
        }
    }

    work(buf_reader, sok_writer).await;

    //Hacer la función que enviará mensajes al servidor
    //Hacer transmisor para enviar mensajes al servidor
    //Hacer ciclo para obtener entrada del usuario


}

/// Manda a llamar las funciones respectivas para recibir mensajes del servidor y entrada del usuario, a su vez 
/// llama a las funciones que procesan estas solicitudes.
// La función que recibe mensajes del server
async fn work<T: AsyncRead + Unpin + Send + 'static, R: AsyncWrite + Unpin + Send + 'static>(reader:  BufReader<T>, writer: R) {
    let (msg_server_tx, mut msg_server_rx) = channel::<String>(100);
    let (tx_for_msg_usr_proccesor, rx_for_msg_sender) = channel::<Vec<u8>>(100);
    let (tx_for_user_entry, mut rx_for_user_entry) = channel::<Result<Action, ()>>(100);
    //let _tx_for_disconect = tx.clone();
    tokio::spawn(async move {
        send_msg_to_server(writer, rx_for_msg_sender).await
    });
    tokio::spawn(async move {
        get_server_msg(msg_server_tx, reader).await;
    });
    tokio::spawn(async move {
        loop {
            let tx_for_user_entry_clone = tx_for_user_entry.clone();
            view::get_usr_entry(tx_for_user_entry_clone).await;
        }
    });

    loop {
        let tx_for_msg_usr_proccesor_clone = tx_for_msg_usr_proccesor.clone();
        tokio::select! {
            response = msg_server_rx.recv() => {
                match response {
                    Some(msg) => {
                        procces_server_msg(msg).await;
                    }
                    None  => {
                        view::disconnected_by_server();
                        std::process::exit(0);
                    }
                }
            }

            Some(resultado) = rx_for_user_entry.recv() => {
                match resultado {
                    //Manejar aqui el caso de Disconnect
                    Ok(action) => {
                        //Manejar aqui el caso de Disconnect
                        procces_user_msg(action, tx_for_msg_usr_proccesor_clone).await;
                    }
                    Err(_) => {view::print_not_valid_command()}
                }
            }

            _ = tokio::signal::ctrl_c() => {
                //Mandar mensaje de desconexión
                //Avisar al usuario que ha sido desconectado
                return;
            }
        }
    }
}

async fn get_server_msg<R: AsyncRead + Unpin>(tx: Sender<String>, mut reader: BufReader<R>) {
    loop {
        let mut line = String::new();
        let Ok(_) = reader.read_line(&mut line).await else {
            view::print_server_close_conection();
            return ;
        };
        let trim_line = line.trim();
        let clean_line = trim_line.trim_matches(|b| b == '\0');
        let _ = tx.send(clean_line.to_string()).await;
        line.clear();
    }

}

async fn procces_server_msg(message: String) {
    let Ok(msg) = serde_json::from_str(&message) else {
        return ;
    };
    procces_server_msg_aux(msg);
}

fn procces_server_msg_aux(message: TypeReciveMesagges) {
    match message {
        TypeReciveMesagges::Disconnected { username } => {
            view::user_disconnected(username);
        }
        TypeReciveMesagges::NewStatus { username, status } => {
            view::print_new_status(username, status);
        }
        TypeReciveMesagges::PublicTextFrom { username, text } => {
            view::print_public_text(username, text);
        }
        TypeReciveMesagges::Response { operation: OperationType::NewRoom, result: Resultado::Success, extra } => {
            if let Some(extra) = extra {
                view::print_success_new_room_created(&extra);
            }
        }
        TypeReciveMesagges::Response { operation: OperationType::NewRoom, result: Resultado::RoomAlreadyExists, extra } => {
            if let Some(roomname) = extra {
                view::print_room_already_exist_result(&roomname);
            }
        }
        TypeReciveMesagges::Response { operation: OperationType::Invite, result: Resultado::NoSuchRoom, extra } => {
            if let Some(roomname) = extra {
                view::print_no_such_room_to_invite(&roomname);
            }
        }
        TypeReciveMesagges::Response { operation: OperationType::Invite, result: Resultado::NoSuchUser, extra } => {
            if let Some(username) = extra {
                view::print_no_such_user_to_invite(&username);
            }
        }
        TypeReciveMesagges::Response { operation: OperationType::JoinRoom, result: Resultado::Success, extra } => {
            if let Some(roomname) = extra {
                view::print_success_join_room(&roomname);
            };
        }
        TypeReciveMesagges::Response { operation: OperationType::JoinRoom, result: Resultado::NoSuchRoom, extra } => {
            if let Some(roomname) = extra {
                view::print_no_such_room_to_join(&roomname);
            };
        }
        TypeReciveMesagges::Response { operation: OperationType::JoinRoom, result: Resultado::NotInvited, extra } => {
            if let Some(roomname) = extra {
                view::print_not_invited_to_room(&roomname);
            }
        }
        TypeReciveMesagges::Response { operation: OperationType::RoomUsers, result: Resultado::NoSuchRoom, extra } => {
            if let Some(roomname) = extra {
                view::print_no_such_room_to_get_users(&roomname);
            }
        }
        TypeReciveMesagges::Response { operation: OperationType::RoomUsers, result: Resultado::NotJoined, extra } => {
            if let Some(roomname) = extra {
                view::print_not_joined_room_to_get_users(&roomname);
            }
        }
        TypeReciveMesagges::Response { operation: OperationType::RoomText, result: Resultado::NoSuchRoom, extra } => {
            if let Some(roomname) = extra {
                view::print_no_such_room_to_send_room_text(&roomname);
            }
        }
        TypeReciveMesagges::Response { operation: OperationType::RoomText, result: Resultado::NotJoined, extra } => {
            if let Some(roomname) = extra {
                view::print_not_joined_to_send_room_text(&roomname);
            }
        }
        TypeReciveMesagges::Response { operation: OperationType::LeaveRoom, result: Resultado::NoSuchRoom, extra } => {
            if let Some(roomname) = extra {
                view::print_no_such_room_to_leave(&roomname);
            }
        }
        TypeReciveMesagges::Response { operation: OperationType::LeaveRoom, result: Resultado::NotJoined, extra } => {
            if let Some(roomname) = extra {
                view::print_not_joined_to_leave(&roomname);
            }
        }
        TypeReciveMesagges::Response { operation: OperationType::Invalid, result: Resultado::Invalid, extra: _ } => {
            view::print_invalid_response();
        }
        TypeReciveMesagges::Response { operation: OperationType::Text, result: Resultado::NoSuchUser, extra } => {
            if let Some(extra) = extra {
                view::print_text_response_no_such_usr(extra);
            };
        }
        TypeReciveMesagges::UserList { users } => {
            view::print_users(users);
        }
        TypeReciveMesagges::TextFrom { username, text } => {
            view::print_private_text(username, text);
        }
        TypeReciveMesagges::NewUser { username } => {
            view::print_new_user_connected(username);
        }
        TypeReciveMesagges::Invitation { username, roomname } => {
            view::print_invitation_to_room(&username, &roomname);
        }
        TypeReciveMesagges::JoinedRoom { roomname, username } => {
            view::print_user_joined_room(&username, &roomname);
        }
        TypeReciveMesagges::LeftRoom { roomname, username } => {
            view::print_user_leaved_room(&username, &roomname);
        }
        TypeReciveMesagges::RoomTextFrom { roomname, username, text } => {
            view::print_room_text_from(&username, &roomname, &text);
        }
        TypeReciveMesagges::RoomUserList { roomname, users } => {
            view::print_room_users(&roomname, users);
        }
        _ => {panic!("El mensaje recibido no coincide con el protocolo")}
    }
}

async fn procces_user_msg(action: Action, tx: Sender<Vec<u8>>) {
    match action {
        Action::Disconnect => {
            let disconect = TypeSendMessage::DISCONNECT;
            let mut msg = serde_json::to_vec(&disconect).unwrap();
            msg.push(b'\n');
            let Ok(_) = tx.send(msg).await else {
                return ;
            };
        }
        Action::Help => {

        }
        Action::PrivateText { username, text } => {
            let private_text = TypeSendMessage::Text { username, text };
            let mut msg = serde_json::to_vec(&private_text).unwrap();
            msg.push(b'\n');
            let Ok(_) = tx.send(msg).await else {
                return ;
            };
        }
        Action::Status { status } => {
            let status = TypeSendMessage::Status { status };
            let mut msg = serde_json::to_vec(&status).unwrap();
            msg.push(b'\n');
            let Ok(_) = tx.send(msg).await else {
                return ;
            };
        }
        Action::Users => {
            let users = TypeSendMessage::Users;
            let mut msg = serde_json::to_vec(&users).unwrap();
            msg.push(b'\n');
            let Ok(_) = tx.send(msg).await else {
                return ;
            };
        }
        Action::PublicText { text } => {
            let text = TypeSendMessage::PublicText { text };
            let mut msg = serde_json::to_vec(&text).unwrap();
            msg.push(b'\n');
            let Ok(_) = tx.send(msg).await else {
                return ;
            };
        }
    }
}

async fn send_msg_to_server<T: AsyncWrite + Unpin>(mut writer: T, mut rx: Receiver<Vec<u8>>){
    while let Some(msg) = rx.recv().await {
        let Ok(_) = writer.write_all(&msg).await else {
            return ;
        };
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
    println!("{}" , response);
    match serde_json::from_str::<TypeReciveMesagges>(response.trim()).expect("El mensaje recibido no era del tipo de mensajes que recibe el cliente") {
        TypeReciveMesagges::Response { operation, result, extra } => {
            if operation != OperationType::Identify {
                panic!("La respuesta no es la correspondiente de acuerdo al protocolo")
            }
            if result != "SUCCESS" {
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
