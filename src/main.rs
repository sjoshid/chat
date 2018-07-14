extern crate websocket;
mod incomplete_server;
mod common;

use websocket::sync::Server;
use std::collections::HashMap;
use websocket::header::Cookie;

fn main() {
    let server = Server::bind("127.0.0.1:2794").unwrap();
    let mut sync_server = incomplete_server::ChatServer{clients: HashMap::new()};

    for ws_upgrade in server.filter_map(Result::ok) {

        if !ws_upgrade.protocols().contains(&"rust-websocket".to_string()) {
            ws_upgrade.reject().unwrap();
            return;
        }

        let h = ws_upgrade.request.headers.clone();
        let c = ws_upgrade.use_protocol("rust-websocket").accept().unwrap();
        let user_id: String;
        {
            //let headrs = c.headers();
            let cookie: &Cookie = match h.get() {
                Some(c) => c,
                None => {
                    println!("Unable to determine user_id");
                    continue;
                }
            };
            let v = &*cookie;
            let id = v[0].clone();
            user_id = id.split('=').collect::<Vec<&str>>().get(1).unwrap().to_string();
        }
        print!("User id is {}", user_id);
        let wc2 = common::WrapperClient::new(user_id, c);
        sync_server.add_client(wc2);

        let (mut receiver, mut sender) = c.split().unwrap();

        for message in receiver.incoming_messages() {
            let raw_message_from_client = message.unwrap();
        }
    }
}
