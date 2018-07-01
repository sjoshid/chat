extern crate websocket;

use websocket::client::sync::Client;
use websocket::stream::sync::TcpStream;

pub type WsClient = Client<TcpStream>;

pub struct WrapperClient {
    username: String,
    c: WsClient,
}

impl WrapperClient {
    pub fn new(username: String, c: WsClient) -> WrapperClient {
        WrapperClient { username, c }
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }
}

impl PartialEq for WrapperClient {
    fn eq(&self, other: &WrapperClient) -> bool {
        let s = self.get_username();
        let o = other.get_username();

        s == o
    }
}

impl Eq for WrapperClient {}