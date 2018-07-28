extern crate websocket;

use std::ops::Deref;
use websocket::sender::Writer;
use websocket::client::sync::Client;
use websocket::stream::sync::TcpStream;

pub type WsWriter = Writer<TcpStream>;

pub struct WrapperSender {
    username: String,
    sender: WsWriter,
}

impl WrapperSender {
    pub fn new(username: String, sender: WsWriter) -> WrapperSender {
        WrapperSender { username, sender}
    }
    pub fn get_username(&self) -> &str {
        &self.username
    }
}

impl  Deref for WrapperSender {

    type Target = WsWriter;

    fn deref(&self) -> &WsWriter {
        &self.sender
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