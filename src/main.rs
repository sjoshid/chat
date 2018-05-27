extern crate websocket;
mod server;
mod client;

use websocket::sync::Server;
use std::collections::HashMap;


fn main() {
    let server = Server::bind("127.0.0.1:2794").unwrap();
    let mut clients = server::ChatServer{clients: HashMap::new()};

    for request in server.filter_map(Result::ok) {

        if !request.protocols().contains(&"rust-websocket".to_string()) {
            request.reject().unwrap();
            return;
        }

        let mut c = request.use_protocol("rust-websocket").accept().unwrap();
        let wc2 = client::WrapperClient::new(String::from("Sujit"), c);
        clients.add_client(wc2);
    }
}
