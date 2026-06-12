use crate::type_recive_messages::TypeReciveMessages;
use crate::user::State;
use crate::user::User;
use ::dashmap::DashMap;
use ::tokio::io::{AsyncBufReadExt, AsyncRead, AsyncReadExt, BufReader};
use std::collections::HashMap;
use std::sync::Arc;

/// Función que devuelve Ok si el usuario respondió con la identificación de acuerdo al protocolo y
/// error en otro caso.
pub(super) async fn retry_identify<T: Unpin + AsyncRead>(
    mut reader: &mut BufReader<T>,
) -> Result<String, ()> {
    let mut line = String::new();
    match (&mut reader).take(1024).read_line(&mut line).await {
        Ok(0) => Err(()),
        Ok(n) if n >= 1024 && !line.ends_with('\n') => Err(()),
        Ok(_n) => {
            let clean_line = line.trim_matches(|b| b == '\0');
            let Ok(message) = serde_json::from_str(clean_line) else {
                return Err(());
            };
            if let TypeReciveMessages::Identify { username } = message {
                return Ok(username);
            }
            Err(())
        }
        _ => Err(()),
    }
}

/// Esta función genera un mapa del cual cada llave corresponde a el nombre de usuario y
/// el valor es su estado actual.
pub(super) async fn generate_map_users(
    users: Arc<DashMap<String, Arc<User>>>,
) -> HashMap<String, State> {
    let mut map = HashMap::new();
    let mut user_list = Vec::new();
    for kv in users.iter() {
        user_list.push(kv.value().clone());
    }
    for user in user_list {
        let status_guard = user.state.lock().await;
        let state = *status_guard;
        let username = user.name.clone();
        map.insert(username, state);
    }
    map
}
