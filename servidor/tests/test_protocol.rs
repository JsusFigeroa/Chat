use std::collections::HashMap;
use std::time::Duration;

use serde_json::{self, Value, json};
use servidor::type_recive_messages::TypeReciveMessages;
use servidor::type_send_messages::{Result as Resultado, TypeSendMessages};
use servidor::user::State;
use servidor::{server::Server, type_send_messages::Operations};
use tokio::time::timeout;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

use crate::common::{clean_readed_line, generate_vec_msg, identify_client, start_server};
mod common;

// Testea la identificación de un usuario;
#[tokio::test]
async fn test_identify() {
    let server = Server::new(4000);
    tokio::spawn(async move {
        server.run().await;
    });
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    let connection = TcpStream::connect("127.0.0.1:4000").await;
    let socket = connection.expect("Error al conectarse al servidor");
    let (sok_reader, mut sok_writer) = socket.into_split();
    let mut reader = BufReader::new(sok_reader);
    let msg = json!({"type":"IDENTIFY", "username":"Jesus"});
    let mut message = serde_json::to_vec(&msg).unwrap();
    message.push(b'\n');
    let Ok(()) = sok_writer.write_all(&message).await else {
        panic!()
    };
    let mut line = String::new();
    let bytes = reader.read_line(&mut line).await.unwrap();
    assert_ne!(bytes, 0);
    let clean_line = clean_readed_line(&line);
    if let TypeSendMessages::Response {
        operation: Operations::Identify,
        result: Resultado::Success,
        extra,
    } = serde_json::from_str(clean_line).unwrap()
    {
        if let Some(name) = extra {
            assert_eq!(name, "Jesus".to_string());
        }
    } else {
        panic!()
    }
}

// Testea el mensaje al identificarse con un nombre de usuario ya en uso.
#[tokio::test]
async fn test_usr_already_exist_identify() {
    let server = Server::new(4001);
    tokio::spawn(async move {
        server.run().await;
    });
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    let connection = TcpStream::connect("127.0.0.1:4001").await;
    let socket = connection.expect("Error al conectarse al servidor");
    let (_sok_reader, mut sok_writer) = socket.into_split();
    let msg = json!({"type":"IDENTIFY", "username":"Jesus"});
    let mut message = serde_json::to_vec(&msg).unwrap();
    message.push(b'\n');
    let Ok(()) = sok_writer.write_all(&message).await else {
        panic!()
    };
    let other_connection = TcpStream::connect("127.0.0.1:4001").await;
    let other_socket = other_connection.expect("Error al conectarse al servidor");
    let (other_sok_reader, mut other_sok_writer) = other_socket.into_split();
    let mut reader = BufReader::new(other_sok_reader);
    let Ok(()) = other_sok_writer.write_all(&message).await else {
        panic!()
    };
    let mut line = String::new();
    let bytes = reader.read_line(&mut line).await.unwrap();
    assert_ne!(bytes, 0);
    let clean_line = clean_readed_line(&line);
    if let TypeSendMessages::Response {
        operation: Operations::Identify,
        result: Resultado::UserAlreadyExists,
        extra,
    } = serde_json::from_str(clean_line).unwrap()
    {
        if let Some(username) = extra {
            assert_eq!(username, "Jesus".to_string());
        } else {
            panic!();
        }
    }
}

// Testea la respuesta a el mensaje de solicitud de usuarios.
#[tokio::test]
async fn test_users() {
    start_server(4002).await;
    let (mut reader, mut writer) = identify_client("127.0.0.1:4002", "Jesus").await;
    let message = TypeReciveMessages::Users;
    let mut msg = serde_json::to_vec(&message).unwrap();
    msg.push(b'\n');
    writer.write_all(&msg).await.unwrap();
    let mut line = String::new();
    let bytes = reader.read_line(&mut line).await.unwrap();
    assert_ne!(bytes, 0);
    line.clear();
    let bytes = reader.read_line(&mut line).await.unwrap();
    assert_ne!(bytes, 0);
    let clean_line = clean_readed_line(&line);
    let recived_json: Value = serde_json::from_str(clean_line).unwrap();
    let mut map_user = HashMap::new();
    map_user.insert("Jesus".to_string(), State::Active);
    let expected_msg = TypeSendMessages::UserList { users: map_user };
    let expected_json = serde_json::to_value(expected_msg).unwrap();
    assert_eq!(recived_json, expected_json);
}

#[tokio::test]
async fn test_change_status() {
    start_server(4003).await;
    let (mut reader, mut writer) = identify_client("127.0.0.1:4003", "Jesus").await;
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    let (mut second_reader, mut _second_writer) =
        identify_client("127.0.0.1:4003", "Kimberly").await;
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    let mut line = String::new();
    let mut bytes = timeout(Duration::from_secs(2), reader.read_line(&mut line))
        .await
        .expect("Error esperando mensaje de confirmación de identificación")
        .unwrap();
    assert_ne!(bytes, 0);
    line.clear();
    bytes = timeout(Duration::from_secs(2), second_reader.read_line(&mut line))
        .await
        .expect("Error en la espera de confirmación de segunda identificación")
        .unwrap();
    assert_ne!(bytes, 0);
    line.clear();
    let message = TypeReciveMessages::Status {
        status: State::Away,
    };
    let msg = generate_vec_msg(&message);
    writer.write_all(&msg).await.unwrap();
    bytes = timeout(Duration::from_secs(2), second_reader.read_line(&mut line))
        .await
        .expect("Error esperando mensaje de status")
        .unwrap();
    assert_ne!(bytes, 0);
    let clean_line = clean_readed_line(&line);
    let recived_json: Value = serde_json::from_str(clean_line).unwrap();
    let expected_type_message = TypeSendMessages::NewStatus {
        username: "Jesus".to_string(),
        status: State::Away,
    };
    let expected_msg = serde_json::to_value(expected_type_message).unwrap();
    assert_eq!(recived_json, expected_msg);
}
