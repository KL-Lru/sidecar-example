use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread::sleep,
    time::Duration,
};

const MINIMAL_HEALTH_RESPONSE: &str = "HTTP/1.1 200 OK\r\n\r\n";

fn main() {
    println!("Sidecar container wake up!");

    let listener = TcpListener::bind("0.0.0.0:8080").expect("Failed to bind Port 8080");

    sleep(Duration::from_secs(10));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => match handle_health(stream) {
                Ok(_) => {
                    println!("Return HEALTH");
                }
                Err(e) => {
                    eprintln!("Unable to handle request: {}", e);
                }
            },
            Err(e) => {
                eprintln!("Unable to connect: {}", e);
            }
        }
    }
}

fn handle_health(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;
    stream.write(MINIMAL_HEALTH_RESPONSE.as_bytes())?;
    stream.flush()?;

    Ok(())
}
