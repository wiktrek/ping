use std::fs::read_to_string;
use std::net::TcpStream;
struct Port {
    name: String,
    port: u16,
}
struct Ip {
    ip: String,
    ports: Vec<Port>,
}
fn main() -> std::io::Result<()> {
    println!("online: {}", check_online("1.1.1.1", 80));
    read_json();
    Ok(())
}
fn check_online(ip: &str, port: u16) -> bool {
    if let Ok(stream) = TcpStream::connect(format!("{}:{}", ip, port)) {
        println!("Connected to the server! {:?}", stream);
        return true;
    } else {
        println!("Couldn't connect to server...");
    };
    false
}
fn read_json() {
    println!("{}", read_to_string("./ips.json").expect("error"));
}
