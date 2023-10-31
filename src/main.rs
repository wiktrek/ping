use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    if let Ok(stream) = TcpStream::connect("1.1.1.1:80") {
        println!("Connected to the server! {:?}", stream);
    } else {
        println!("Couldn't connect to server...");
    };
    Ok(())
}
