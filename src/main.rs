extern crate websocket;
mod MyServer;

use websocket::client::sync::Client;
use websocket::stream::sync::TcpStream;
use websocket::stream::Stream;
use websocket::stream::sync::AsTcpStream;
use std::ops::Deref;
use std::thread;
use websocket::OwnedMessage;
use websocket::sync::Server;

//#[derive(PartialEq, Eq, Hash)]
pub struct WrapperClient<S>(Client<S>) where S: AsTcpStream + Stream;

/*pub struct ChatClient<S> where S: AsTcpStream + Stream {
    username: String,
    wrapper_client: WrapperClient<Client<S>>
}*/

impl<S> WrapperClient<S> where S: AsTcpStream + Stream{
    fn new(x: Client<S>) -> WrapperClient<S> {
        WrapperClient(x)
    }
}

impl<S> Deref for WrapperClient<S> where S: AsTcpStream + Stream {
    type Target = Client<S>;

    fn deref(&self) -> &Client<S> {
        &self.0
    }
}

/*impl<S> PartialEq for WrapperClient<S> where S: AsTcpStream + Stream {
    fn eq(&self, other: &WrapperClient<S>) -> bool {
        let o = *other;
        let s = *self;
        //let last_ip = last_client.peer_addr().unwrap();
        //assert_eq!("127.0.0.1:8080".parse(), Ok(socket));

        let otherIp = o.peer_addr().unwrap();
        let selfIp = s.peer_addr().unwrap();

        otherIp.par
    }
}
impl Eq for Book {}*/

fn main() {
    let server = Server::bind("127.0.0.1:2794").unwrap();
    //let mut chat_server = MyServer::ChatServer{my_users: Vec::new()};

    for request in server.filter_map(Result::ok) {

        if !request.protocols().contains(&"rust-websocket".to_string()) {
            request.reject().unwrap();
            return;
        }

        let mut client = request.use_protocol("rust-websocket").accept().unwrap();
        let wc = &WrapperClient(client);
        //now do a deref here

    }
}
