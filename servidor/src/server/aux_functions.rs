use crate::server::server_mesagges::{generate_not_identified_msg, generate_succes_identify_response, generate_not_valid_msg, generate_user_already_exists_response};
use crate::type_recive_messages::TypeReciveMessages;
use crate::user::User;
use::tokio::io::{BufReader, BufWriter, AsyncBufReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use std::sync::Arc;
use tokio::sync::Mutex;
use super::*;

pub(super) async fn retry_identify(mut reader: BufReader<OwnedReadHalf>, mut writer: BufWriter<OwnedWriteHalf>, server: Arc<Mutex<Server>>){
        let mut line = String::new();
        let readed_bytes = reader.read_line(&mut line).await.unwrap_or_default();

        if readed_bytes == 0 {
            writer.shutdown().await.unwrap_or_default();
            return;
        }
        
        let mensagge: TypeReciveMessages;
            match serde_json::from_str(&line) {
                Ok(text) => {
                    mensagge = text;
                }
                Err(_) => {
                    let msg = generate_not_valid_msg().expect("Error al generar el mensaje de json inválido");
                    writer.write_all(&msg).await.expect("No fue posible escribir en el socket");
                    writer.flush().await.unwrap_or_default();
                    writer.shutdown().await.expect("Error al cerrar la conexión");
                    return;
                }
            }
            if let TypeReciveMessages::Identify { type_msg, username } = mensagge {

                if type_msg != "IDENTIFY" {
                    let msg = generate_not_identified_msg().expect("Ocurrio un error al generar el mensaje");
                    writer.flush().await.unwrap_or_default();
                    writer.write_all(&msg).await.expect("Error al escribir en socket");
                    writer.shutdown().await.unwrap_or_default();
                    return;
                }

                let username_lowercase = username.to_lowercase();
                let mut locked_server = server.lock().await;

                if locked_server.users.contains_key(&username_lowercase) {
                    let msg = generate_user_already_exists_response(&username).expect("No fue posible generar el mensaje");
                    writer.write_all(&msg).await.expect("No fue posible escribir en el stream");
                    writer.flush().await.unwrap_or_default();
                    writer.shutdown().await.unwrap_or_default();
                    return;
                }

                let msg = generate_succes_identify_response(&username).expect("No fue posible generar el mensaje");
                writer.write_all(&msg).await.expect("No fue posible escribir");
                writer.flush().await.unwrap_or_default();
                println!("Nuevo usuario conectado con nombre {}", username);
                let new_user = User::new(username, reader, writer);
                locked_server.users.insert(username_lowercase, new_user);
            }
    }