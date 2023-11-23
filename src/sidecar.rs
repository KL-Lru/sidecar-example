use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread::sleep,
    time::Duration,
};

const MINIMAL_HEALTH_RESPONSE: &str = "HTTP/1.1 200 OK\r\n\r\n";

fn main() {
    ctrlc::set_handler(move || {
        println!("Sidecar container is shutting down!");
        std::process::exit(0);
    })
    .expect("Failed to set SIGTERM handler");

    println!("Sidecar container wake up!");
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).expect("Failed to bind Port");

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
