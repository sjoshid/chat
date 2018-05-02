mod MyServer;

fn main() {
    // let mut users = MyServer::ChatServer{my_users: vec![String::from("")]};
    let mut users = MyServer::ChatServer{my_users: Vec::new()};
    users.add_user(String::from("Morgan"));
    println!("First user is {}", users.my_users[0]);
}
