use std::net::{TcpListener, TcpStream};
use std::io::{self, prelude::*};

fn main() -> std::io::Result<()> {
    println!("Initializing echo server...");
    let server = TcpListener::bind("0.0.0.0:80")?;

    println!("echo server ready.");
    for stream in server.incoming() {
        match stream {
            Ok(stream) => {
                println!("Handling a connection.");
                match echo(stream) {
                    Ok(()) => println!("Success."),
                    Err(e) => eprintln!("Echo error: {e}")
                }
            }
            Err(e) => eprintln!("Connection failed: {e}")
        }
    }
    Ok(())
}

fn echo(mut stream: TcpStream) -> std::io::Result<()> {
    let mut write_stream = stream.try_clone()?;
    io::copy(&mut stream, &mut write_stream).expect("Error copying stream");
    stream.read(&mut [0; 128])?;
    stream.write(&[1])?;
    Ok(())
}
