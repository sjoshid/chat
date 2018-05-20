extern crate websocket;

use websocket::client::sync::Client;
use websocket::stream::sync::TcpStream;
use websocket::stream::Stream;
use std::ops::Deref;

#[derive(PartialEq, Eq, Hash)]
pub struct WrapperClient(Client<TcpStream>);


impl<C> Deref for WrapperClient {
    type Target = C;

    fn deref(&self) -> &C {
        &self.0
    }
}