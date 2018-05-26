extern crate websocket;
mod MyServer;
mod client;

use websocket::sync::Server;


fn main() {
    let server = Server::bind("127.0.0.1:2794").unwrap();
    //let mut chat_server = MyServer::ChatServer{my_users: Vec::new()};

    for request in server.filter_map(Result::ok) {

        if !request.protocols().contains(&"rust-websocket".to_string()) {
            request.reject().unwrap();
            return;
        }

        let mut c = request.use_protocol("rust-websocket").accept().unwrap();
        let wc2 = client::WrapperClient::new(String::from("Sujit"), c);
    }
}
