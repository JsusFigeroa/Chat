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
    #[must_use]
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
        let addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, self.port);
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
                    .await;
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
            Ok(n) if n >= read_limit && !line.ends_with('\n') => {
                return;
            }
            Ok(_) => {}
            _ => {
                return;
            }
        }
        let clean_line = line.trim_matches(|c| c == '\0');
        let Ok(username) = self
            .clone()
            .try_identify(clean_line, &mut buff_reader, &mut sok_writter)
            .await
        else {
            return;
        };
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
        println!("Nuevo usuario con nombre {username} conectado");
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
                if let Ok(new_username) = aux_functions::retry_identify(reader).await {
                    username = new_username;
                } else {
                    let msg = server_mesagges::generate_not_valid_msg();
                    writter.write_all(&msg).await;
                    return Err(());
                }
            }
        }
        let msg = generate_not_identified_msg();
        let _ = writter.write_all(&msg).await;
        Err(())
    }

    fn build_msg_processors(self: Arc<Self>, rx: Receiverr<Letter<Vec<u8>>>) {
        for _ in 0..4 {
            let rx_clone = rx.clone();
            let self_clone = self.clone();
            tokio::spawn(async move {
                self_clone.process_letter(rx_clone).await;
            });
        }
        tokio::spawn(async move {
            self.process_letter(rx).await;
        });
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
            let Ok(()) = writer.write_all(&msg).await else {
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
                }
                _ => {
                    break;
                }
            }
        }
        let disconnect = TypeReciveMessages::Disconect;
        println!("El usuario {username} se ha desconectado");
        let letter = Letter::new(username, disconnect, user_tx.clone());
        drop(reader);
        drop(user_tx);
        let _ = global_tx.send(letter).await;
    }

    async fn procces_letter_aux(self: Arc<Self>, letter: Letter<Vec<u8>>) {
        match letter.msg {
            TypeReciveMessages::PublicText { text } => {
                self.procces_public_text_msg(&letter.usr_sender, &text)
                    .await;
            }
            TypeReciveMessages::Status { status } => {
                self.process_status_msg(&letter.usr_sender, status).await;
            }
            TypeReciveMessages::Users => {
                self.process_users_msg(letter.reply_to).await;
            }
            TypeReciveMessages::Text { username, text } => {
                self.process_private_text_msg(
                    &username,
                    &text,
                    &letter.usr_sender,
                    letter.reply_to,
                )
                .await;
            }
            TypeReciveMessages::Identify { username } => {
                self.process_identify_msg(&username).await;
            }

            TypeReciveMessages::Disconect => {
                self.process_disconect_msg(&letter.usr_sender).await;
            }

            TypeReciveMessages::Invite {
                roomname,
                usernames,
            } => {
                self.process_invite_msg(&roomname, usernames, letter.reply_to, &letter.usr_sender)
                    .await;
            }
            TypeReciveMessages::JoinRoom { roomname } => {
                self.process_join_room_msg(&roomname, letter.reply_to, &letter.usr_sender)
                    .await;
            }
            TypeReciveMessages::NewRoom { roomname } => {
                self.process_new_room_msg(&roomname, &letter.usr_sender, letter.reply_to)
                    .await;
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
                        room.send_msg(letter.usr_sender, text, letter.reply_to)
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
                room.send_users(letter.reply_to, letter.usr_sender).await;
            }
        }
    }
    async fn procces_public_text_msg(self: Arc<Self>, user_sender: &str, text: &str) {
        let mut transmisors = Vec::new();
        for user in self.users.iter() {
            if user.name.to_lowercase() == user_sender.to_lowercase() {
                continue;
            }
            transmisors.push(user.tx.clone());
        }
        let msg = generate_public_text_from(user_sender, text);
        for tx in transmisors {
            let Ok(()) = tx.send(msg.clone()).await else {
                return;
            };
        }
    }

    async fn process_status_msg(self: Arc<Self>, user_sender: &str, status: State) {
        let usr_sender_lower = user_sender.to_lowercase();
        let user = {
            if let Some(kv) = self.users.get(&usr_sender_lower) {
                kv.value().clone()
            } else {
                return;
            }
        };
        let mut status_guard = user.state.lock().await;
        *status_guard = status;
        drop(status_guard);
        let mut transmisors = Vec::new();
        let msg = generate_new_status_msg(user_sender, status);
        for kv in self.users.iter() {
            if kv.key() == &usr_sender_lower {
                continue;
            }
            transmisors.push(kv.tx.clone());
        }
        for transmisor in transmisors {
            let _ = transmisor.send(msg.clone()).await;
        }
    }

    async fn process_users_msg(self: Arc<Self>, reply_to: Sender<Vec<u8>>) {
        let map = aux_functions::generate_map_users(self.users.clone()).await;
        let msg = generate_users_msg(map);
        let _ = reply_to.send(msg).await;
    }

    async fn process_private_text_msg(
        self: Arc<Self>,
        username: &str,
        text: &str,
        user_sender: &str,
        reply_to: Sender<Vec<u8>>,
    ) {
        let username_lower = &username.to_lowercase();
        let msg = generate_text_from_msg(user_sender, text);
        let user = {
            if let Some(kv) = self.users.get_mut(username_lower) {
                kv.value().clone()
            } else {
                let msg = server_mesagges::generate_text_user_not_exist(username);
                let _ = reply_to.send(msg).await;
                return;
            }
        };
        let _ = user.tx.send(msg).await;
    }
    async fn process_identify_msg(self: Arc<Self>, username: &str) {
        let msg = generate_new_user_msg(username);
        let mut transmisors = Vec::with_capacity(self.users.len());
        for kv in self.users.iter() {
            if kv.key() == &username.to_lowercase() {
                continue;
            }
            transmisors.push(kv.tx.clone());
        }
        for tx in transmisors {
            let _ = tx.send(msg.clone()).await;
        }
    }

    async fn process_disconect_msg(self: Arc<Self>, user_sender: &str) {
        if let Some((name, user)) = self.users.remove(&user_sender.to_lowercase()) {
            let guard_room_keys = user.rooms_keys.lock().await;
            let mut user_rooms = Vec::with_capacity((*guard_room_keys).len());
            for room_key in &*guard_room_keys {
                let room = {
                    if let Some(kv) = self.rooms.get(room_key) {
                        kv.value().clone()
                    } else {
                        continue;
                    }
                };
                user_rooms.push(room);
            }
            drop(guard_room_keys);
            for room in user_rooms {
                room.remove_disconected_user(&name).await;
            }
            let guard_invitation_room_keys = user.invitations_room_keys.lock().await;
            for room_key in &*guard_invitation_room_keys {
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
            let msg = server_mesagges::generate_disconected_msg(user_sender);
            for tx in transmisors {
                let _ = tx.send(msg.clone()).await;
            }
        }
    }

    async fn process_invite_msg(
        self: Arc<Self>,
        roomname: &str,
        usernames: Vec<String>,
        reply_to: Sender<Vec<u8>>,
        user_sender: &str,
    ) {
        if self.rooms.contains_key(&roomname.to_lowercase()) {
            let opt_user = usernames
                .iter()
                .find(|&k| !self.users.contains_key(&k.to_lowercase()));
            if let Some(username) = opt_user {
                let msg = server_mesagges::no_such_user_invite_msg(username);
                let _ = reply_to.send(msg).await;
            } else {
                let mut users = Vec::new();
                for username in usernames {
                    let opt_user = self
                        .users
                        .get(&username.to_lowercase())
                        .map(|kv| kv.value().clone());
                    if let Some(user) = opt_user {
                        users.push(user);
                    }
                }
                let room = {
                    if let Some(kv) = self.rooms.get(&roomname.to_lowercase()) {
                        kv.value().clone()
                    } else {
                        return;
                    }
                };
                room.process_invitation(users, user_sender).await;
            }
        } else {
            let msg = server_mesagges::no_such_room_invite_msg(roomname);
            let _ = reply_to.send(msg).await;
        }
    }

    async fn process_join_room_msg(
        self: Arc<Self>,
        roomname: &str,
        reply_to: Sender<Vec<u8>>,
        user_sender: &str,
    ) {
        let roomname_lower = &roomname.to_lowercase();
        let room = {
            if let Some(kv) = self.rooms.get(roomname_lower) {
                kv.value().clone()
            } else {
                let msg = server_mesagges::no_such_room_join_room_msg(roomname);
                let _ = reply_to.send(msg).await;
                return;
            }
        };
        room.accept_invitation(user_sender, reply_to).await;
    }

    async fn process_new_room_msg(
        self: Arc<Self>,
        roomname: &str,
        user_sender: &str,
        reply_to: Sender<Vec<u8>>,
    ) {
        if let Some(user) = self
            .users
            .get(&user_sender.to_lowercase())
            .map(|k| k.value().clone())
        {
            let roomname_lower = roomname.to_lowercase();
            let room = Arc::new(Room::new(roomname.to_string(), user));
            self.rooms.insert(roomname_lower, room);
            let msg = server_mesagges::new_room_success(roomname);
            let _ = reply_to.send(msg).await;
        }
    }
}
