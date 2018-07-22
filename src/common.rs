extern crate websocket;

use std::ops::Deref;
use websocket::sender::Writer;
use websocket::client::sync::Client;
use websocket::stream::sync::TcpStream;

pub type WsWriter = Writer<TcpStream>;

pub struct WrapperSender<'a> {
    username: String,
    sender: &'a mut WsWriter,
}

impl<'a> WrapperSender<'a> {
    pub fn new(username: String, sender: &'a mut WsWriter) -> WrapperSender<'a> {
        WrapperSender { username, sender}
    }
    pub fn get_username(&self) -> &str {
        &self.username
    }
}

impl <'a> Deref for WrapperSender<'a> {

    type Target = WsWriter;

    fn deref(&self) -> &WsWriter {
        self.sender
    }
}

impl <'a> PartialEq for WrapperSender<'a> {
    fn eq(&self, other: &WrapperSender) -> bool {
        let s = self.get_username();
        let o = other.get_username();

        s == o
    }
}

impl <'a> Eq for WrapperSender<'a> {}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageDetails {
    pub sender_username: String,
    pub receiver_username: String,
    pub message: String
}