use serde::{Deserialize, Serialize};
use std::fs::read_to_string;
use std::net::TcpStream;
#[derive(Debug, Serialize, Deserialize)]
struct Port {
    name: String,
    port: u16,
}
#[derive(Debug, Serialize, Deserialize)]
struct Ip {
    ip: String,
    ports: Vec<Port>,
}
fn main() -> std::io::Result<()> {
    println!("online: {}", check_online("1.1.1.1", 80));
    let v = read_json();
    for ip in v {
        println!("{:?}", ip)
    }
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
fn read_json() -> Vec<Ip> {
    let data = read_to_string("./ips.json").expect("error");
    // println!("{}", data);
    let v: Vec<Ip> = serde_json::from_str(&data).expect("error");
    v
}
