extern crate websocket;

use std::ops::{Deref, DerefMut};
use websocket::sender::Writer;
use websocket::client::sync::Client;
use websocket::stream::sync::TcpStream;

pub type WsWriter = Writer<TcpStream>;

pub struct WrapperSender {
    pub username: String,
    pub src: WsWriter,
}

impl WrapperSender {
    pub fn new(username: String, mut sender: WsWriter) -> WrapperSender {
        WrapperSender { username, src: sender }
    }
    pub fn get_username(&self) -> &str {
        &self.username
    }
}

impl Deref for WrapperSender {
    type Target = WsWriter;
    fn deref(&self) -> &WsWriter {
        &self.src
    }
}

impl DerefMut for WrapperSender {
    fn deref_mut(&mut self) -> &mut WsWriter {
        &mut self.src
    }
}

impl  PartialEq for WrapperSender {
    fn eq(&self, other: &WrapperSender) -> bool {
        let s = self.get_username();
        let o = other.get_username();

        s == o
    }
}

impl  Eq for WrapperSender {}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageDetails {
    pub sender_username: String,
    pub receiver_username: String,
    pub message: String
}