use crate::letter::Letter;
use crate::room::Room;
use crate::server::server_mesagges::{
    generate_new_status_msg, generate_new_user_msg, generate_not_identified_msg,
    generate_public_text_from, generate_succes_identify_response, generate_text_from_msg,
    generate_users_msg,
};
use crate::type_recive_messages::TypeReciveMessages;
use crate::user::{State, User};
use async_channel::{self, Receiver as Receiverr, Sender as Senderr};
use dashmap::DashMap;
use dashmap::Entry;
use serde_json::{self};
use std::net::{Ipv4Addr, SocketAddrV4};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::{self, Receiver, Sender};

pub(super) mod aux_functions;
pub(super) mod server_mesagges;

pub struct Server {
    users: Arc<DashMap<String, Arc<User>>>,
    port: u16,
    rooms: Arc<DashMap<String, Arc<Room>>>,
}

impl Server {
    pub fn new(port: u16) -> Arc<Server> {
        let users = Arc::new(DashMap::new());
        let rooms = Arc::new(DashMap::new());
        Arc::new(Server { users, port, rooms })
    }

    pub async fn run(self: Arc<Self>) {
        println!("Aceptando conexiones");
        self.get_conections().await;
    }

    async fn get_conections(self: Arc<Self>) {
        let addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), self.port);
        let listener = TcpListener::bind(addr).await.unwrap();
        let (tx, rx) = async_channel::bounded::<Letter<Vec<u8>>>(124);
        self.clone().build_msg_processors(rx.clone());
        loop {
            let (socket, _) = listener.accept().await.unwrap();
            println!("Conexión aceptada");
            let server_for_client = Arc::clone(&self);
            let tx_for_client = tx.clone();
            tokio::spawn(async move {
                server_for_client
                    .process_conection(socket, tx_for_client)
                    .await
            });
        }
    }

    async fn process_conection(
        self: Arc<Self>,
        socket: TcpStream,
        global_tx: Senderr<Letter<Vec<u8>>>,
    ) {
        let (sok_reader, mut sok_writter) = socket.into_split();
        let (user_tx, user_rx) = mpsc::channel::<Vec<u8>>(124);
        let mut buff_reader = BufReader::new(sok_reader);
        let mut line = String::with_capacity(1024);
        let read_limit = 1024;
        match (&mut buff_reader)
            .take(read_limit as u64)
            .read_line(&mut line)
            .await
        {
            Ok(0) => {
                return;
            }
            Ok(n) if n >= read_limit && !line.ends_with('\n') => {
                return;
            }
            Ok(_) => {}
            Err(_) => {
                return;
            }
        }
        let clean_line = line.trim_matches(|c| c == '\0');
        let username;
        match self
            .clone()
            .try_identify(clean_line, &mut buff_reader, &mut sok_writter)
            .await
        {
            Ok(name) => username = name,
            Err(_) => return,
        }
        let username_normalized = username.to_lowercase();
        let usr_tx_clone = user_tx.clone();
        let global_tx_clone = global_tx.clone();
        let username_clone = username.clone();
        tokio::spawn(async move {
            Server::build_msg_client_processor(
                usr_tx_clone,
                username_clone,
                buff_reader,
                global_tx_clone,
            )
            .await;
        });
        let msg = generate_succes_identify_response(&username);
        let _ = sok_writter.write_all(&msg).await;
        tokio::spawn(async move {
            Server::build_msg_sender_to_client(user_rx, sok_writter).await;
        });
        println!("Nuevo usuario con nombre {} conectado", username);
        let new_user = User::new(username, user_tx);
        let msg_new_user = TypeReciveMessages::Identify {
            username: new_user.name.clone(),
        };
        let letter = Letter::new(new_user.name.clone(), msg_new_user, new_user.tx.clone());
        self.users.insert(username_normalized, Arc::new(new_user));
        let _ = global_tx.send(letter).await;
    }

    async fn try_identify<T: AsyncRead + Unpin, R: AsyncWrite + Unpin>(
        self: Arc<Self>,
        recived_line: &str,
        reader: &mut BufReader<T>,
        writter: &mut R,
    ) -> Result<String, ()> {
        let Ok(message) = serde_json::from_str::<TypeReciveMessages>(recived_line) else {
            let msg = server_mesagges::generate_not_valid_msg();
            let _ = writter.write_all(&msg).await;
            return Err(());
        };
        if let TypeReciveMessages::Identify { mut username } = message {
            loop {
                let username_normalized = &username.to_lowercase();
                if !self.users.contains_key(username_normalized) {
                    return Ok(username);
                }
                let msg = server_mesagges::generate_user_already_exists_response(&username);
                let _ = writter.write_all(&msg).await;
                match aux_functions::retry_identify(reader).await {
                    Ok(new_username) => username = new_username,
                    Err(_) => {
                        let msg = server_mesagges::generate_not_valid_msg();
                        let _ = writter.write_all(&msg).await;
                        return Err(());
                    }
                }
            }
        }
        let msg = generate_not_identified_msg();
        let _ = writter.write_all(&msg).await;
        return Err(());
    }

    fn build_msg_processors(self: Arc<Self>, rx: Receiverr<Letter<Vec<u8>>>) {
        for _ in 0..5 {
            let rx_clone = rx.clone();
            let self_clone = self.clone();
            tokio::spawn(async move {
                self_clone.process_letter(rx_clone).await;
            });
        }
    }

    /// Recibe una carta por la cola de mensajes y se encarga de procesar y llevar a cabo la acción correspondiente.
    /// Ejecuta las acciones correspondientes para cada mensaje, en particular para disconect solo elimina al usuario
    /// de la lista de usuarios (y grupos).
    async fn process_letter(self: Arc<Self>, rx: Receiverr<Letter<Vec<u8>>>) {
        while let Ok(msg) = rx.recv().await {
            self.clone().procces_letter_aux(msg).await;
        }
    }

    async fn build_msg_sender_to_client<T: AsyncWrite + Unpin>(
        mut rx: Receiver<Vec<u8>>,
        mut writer: T,
    ) {
        while let Some(mut msg) = rx.recv().await {
            msg.push(0);
            let Ok(_) = writer.write_all(&msg).await else {
                return;
            };
        }
    }

    ///Función que genera el receptor de mensajes del cliente, si hay algún mensaje se lee
    /// y se envia al procesador global de mensajes, si se sobrepasa el limite del búffer
    /// o hay algun error de lectura se cierra la conexión, en caso de mensaje inválido
    /// o de mensaje de desconexión termina el proceso y envia la carta correspondiente
    /// a el procesador de mensajes, como la identificación se produce previamente a
    /// el inicio de este proceso, si un cliente reenvia el mensaje de identificación
    /// se lo desconecta.
    async fn build_msg_client_processor<T: AsyncRead + Unpin>(
        user_tx: Sender<Vec<u8>>,
        username: String,
        mut reader: BufReader<T>,
        global_tx: Senderr<Letter<Vec<u8>>>,
    ) {
        let mut line = String::new();
        let limit = 1024;
        loop {
            match (&mut reader).take(limit as u64).read_line(&mut line).await {
                Ok(0) => {
                    break;
                }
                Ok(n) if n >= limit && !line.ends_with('\n') => {
                    break;
                }
                Ok(_n) => {
                    let clean_line = line.trim_matches(|b| b == '\0');
                    let Ok(message) = serde_json::from_str::<TypeReciveMessages>(clean_line) else {
                        let msg = server_mesagges::generate_not_valid_msg();
                        let _ = user_tx.send(msg).await;
                        break;
                    };
                    if let TypeReciveMessages::Disconect = message {
                        break;
                    }
                    if let TypeReciveMessages::Identify { username: _ } = message {
                        let msg = server_mesagges::generate_not_valid_msg();
                        let _ = user_tx.send(msg).await;
                        break;
                    }
                    let letter = Letter::new(username.clone(), message, user_tx.clone());
                    let _ = global_tx.send(letter).await;
                    line.clear();
                    continue;
                }
                Err(_) => {
                    break;
                }
            }
        }
        let disconnect = TypeReciveMessages::Disconect;
        println!("El usuario {} se ha desconectado", username);
        let letter = Letter::new(username, disconnect, user_tx.clone());
        drop(reader);
        drop(user_tx);
        let _ = global_tx.send(letter).await;
    }

    async fn procces_letter_aux(self: Arc<Self>, letter: Letter<Vec<u8>>) {
        match letter.msg {
            TypeReciveMessages::PublicText { text } => {
                let mut transmisors = Vec::new();
                for kv in self.users.iter() {
                    if kv.name.to_lowercase() == letter.usr_sender.to_lowercase() {
                        continue;
                    }
                    transmisors.push(kv.tx.clone());
                }
                let msg = generate_public_text_from(&letter.usr_sender, &text);
                for tx in transmisors {
                    let Ok(_) = tx.send(msg.clone()).await else {
                        return;
                    };
                }
            }
            TypeReciveMessages::Status { status } => {
                let usr_sender_lower = &letter.usr_sender.to_lowercase();
                let user = {
                    if let Some(kv) = self.users.get(usr_sender_lower) {
                        kv.value().clone()
                    } else {
                        return;
                    }
                };
                let mut status_guard = user.state.lock().await;
                *status_guard = status.clone();
                let mut transmisors = Vec::new();
                let msg = generate_new_status_msg(&letter.usr_sender, status);
                for kv in self.users.iter() {
                    if kv.key() == usr_sender_lower {
                        continue;
                    }
                    transmisors.push(kv.tx.clone());
                }
                for transmisor in transmisors {
                    let _ = transmisor.send(msg.clone()).await;
                }
            }
            TypeReciveMessages::Users => {
                let map = aux_functions::generate_map_users(self.users.clone()).await;
                let msg = generate_users_msg(map);
                let _ = letter.reply_to.send(msg).await;
            }
            TypeReciveMessages::Text { username, text } => {
                let username_lower = &username.to_lowercase();
                let msg = generate_text_from_msg(&letter.usr_sender, &text);
                let user = {
                    if let Some(kv) = self.users.get_mut(username_lower) {
                        kv.value().clone()
                    } else {
                        let msg = server_mesagges::generate_text_user_not_exist(&username);
                        let _ = letter.reply_to.send(msg).await;
                        return;
                    }
                };
                let _ = user.tx.send(msg).await;
            }
            TypeReciveMessages::Identify { username } => {
                let msg = generate_new_user_msg(&username);
                let mut transmisors = Vec::with_capacity(self.users.len());
                for kv in self.users.iter() {
                    if kv.key() == &username.to_lowercase() {
                        continue;
                    } else {
                        transmisors.push(kv.tx.clone());
                    }
                }
                for tx in transmisors {
                    let _ = tx.send(msg.clone()).await;
                }
            }

            TypeReciveMessages::Disconect => {
                if let Some((name, user)) = self.users.remove(&letter.usr_sender.to_lowercase()) {
                    let guard_room_keys = user.rooms_keys.lock().await;
                    let mut user_rooms = Vec::with_capacity((*guard_room_keys).len());
                    for room_key in (*guard_room_keys).iter() {
                        let room = {
                            if let Some(kv) = self.rooms.get(room_key) {
                                kv.value().clone()
                            } else {
                                continue;
                            }
                        };
                        user_rooms.push(room);
                        // room.remove_disconected_user(&name).await;
                    }
                    drop(guard_room_keys);
                    for room in user_rooms {
                        room.remove_disconected_user(&name).await;
                    }
                    let guard_invitation_room_keys = user.invitations_room_keys.lock().await;
                    for room_key in (*guard_invitation_room_keys).iter() {
                        let room = {
                            if let Some(kv) = self.rooms.get(room_key) {
                                kv.value().clone()
                            } else {
                                continue;
                            }
                        };
                        room.remove_invitation(&name);
                    }
                    drop(guard_invitation_room_keys);
                    let mut transmisors = Vec::new();
                    for kv in self.users.iter() {
                        transmisors.push(kv.tx.clone());
                    }
                    let msg = server_mesagges::generate_disconected_msg(&letter.usr_sender);
                    for tx in transmisors {
                        let _ = tx.send(msg.clone()).await;
                    }
                };
            }

            TypeReciveMessages::Invite {
                roomname,
                usernames,
            } => {
                if self.rooms.contains_key(&roomname.to_lowercase()) {
                    let opt_user = usernames
                        .iter()
                        .find(|&k| !self.users.contains_key(&k.to_lowercase()));
                    match opt_user {
                        Some(username) => {
                            let msg = server_mesagges::no_such_user_invite_msg(username);
                            let _ = letter.reply_to.send(msg).await;
                        }
                        None => {
                            let mut users = Vec::new();
                            for username in usernames {
                                let opt_user = self
                                    .users
                                    .get(&username.to_lowercase())
                                    .map(|kv| kv.value().clone());
                                if let Some(user) = opt_user {
                                    users.push(user);
                                };
                            }
                            let room = {
                                if let Some(kv) = self.rooms.get(&roomname.to_lowercase()) {
                                    kv.value().clone()
                                } else {
                                    return;
                                }
                            };
                            let _ = room.process_invitation(users, &letter.usr_sender).await;
                        }
                    }
                } else {
                    let msg = server_mesagges::no_such_room_invite_msg(&roomname);
                    let _ = letter.reply_to.send(msg).await;
                }
            }
            TypeReciveMessages::JoinRoom { roomname } => {
                let roomname_lower = &roomname.to_lowercase();
                let room = {
                    if let Some(kv) = self.rooms.get(roomname_lower) {
                        kv.value().clone()
                    } else {
                        let msg = server_mesagges::no_such_room_join_room_msg(&roomname);
                        let _ = letter.reply_to.send(msg).await;
                        return;
                    }
                };
                let _ = room
                    .accept_invitation(&letter.usr_sender, letter.reply_to)
                    .await;
            }
            TypeReciveMessages::NewRoom { roomname } => {
                if let Some(user) = self
                    .users
                    .get(&letter.usr_sender.to_lowercase())
                    .map(|k| k.value().clone())
                {
                    let roomname_lower = roomname.to_lowercase();
                    let room = Arc::new(Room::new(roomname.clone(), user));
                    self.rooms.insert(roomname_lower, room);
                    let msg = server_mesagges::new_room_success(&roomname);
                    let _ = letter.reply_to.send(msg).await;
                }
            }
            TypeReciveMessages::LeaveRoom { roomname } => {
                if self.rooms.contains_key(&roomname.to_lowercase()) {
                    let entry = self.rooms.entry(roomname.to_lowercase());
                    match entry {
                        Entry::Occupied(mut locked_entry) => {
                            let room = locked_entry.get_mut();
                            let num_users =
                                room.remove_user(letter.reply_to, letter.usr_sender).await;
                            if num_users == 0 {
                                locked_entry.remove();
                            }
                        }
                        Entry::Vacant(_) => {}
                    }
                } else {
                    let msg = server_mesagges::leave_room_not_such_room(&roomname);
                    let _ = letter.reply_to.send(msg).await;
                }
            }
            TypeReciveMessages::RoomText { roomname, text } => {
                if self.rooms.contains_key(&roomname.to_lowercase()) {
                    let kv_opt = self.rooms.get(&roomname.to_lowercase());
                    if let Some(kv) = kv_opt {
                        let room = kv.value();
                        let _ = room
                            .send_msg(letter.usr_sender, text, letter.reply_to)
                            .await;
                    }
                } else {
                    let msg = server_mesagges::room_text_no_such_room(&roomname);
                    let _ = letter.reply_to.send(msg).await;
                }
            }
            TypeReciveMessages::RoomUsers { roomname } => {
                let roomname_lower = &roomname.to_lowercase();
                let room = {
                    if let Some(kv) = self.rooms.get(roomname_lower) {
                        kv.value().clone()
                    } else {
                        let msg = server_mesagges::room_users_no_such_room(&roomname);
                        let _ = letter.reply_to.send(msg).await;
                        return;
                    }
                };
                let _ = room.send_users(letter.reply_to, letter.usr_sender).await;
            }
        }
    }
}
