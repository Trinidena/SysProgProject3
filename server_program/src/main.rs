use std::net::{TcpListener, TcpStream};
use std::io::{self, Read, Write};
use std::fs;
use std::thread;
use std::thread::JoinHandle;
use log::{error, info};
use env_logger;

mod lib;

fn main() {
    env_logger::init();
    fs::create_dir_all("data").expect("Unable to create data directory");
    start_listening("127.0.0.1:7878").unwrap_or_else(|e| {
        panic!("Error: {}", e);
    });
}

fn start_listening(server_addr: &str) -> io::Result<()> {
    let listener = TcpListener::bind(server_addr)?;
    log::info!("Server listening on {}", server_addr);

    let mut handles: Vec<JoinHandle<()>> = vec![];

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                log::info!("Stream OK");
                let handle = thread::spawn(|| handle_client(stream));
                handles.push(handle);
                log::info!("Thread spawned");
            }
            Err(e) => {
                log::info!("Error accepting connection: {}", e);
            }
        }
    }

    for handle in handles {
        log::info!("Joining Handle");
        handle.join().expect("Error joining thread");
        log::info!("Handle Joined");
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream) {
    log::info!("Handling Client");
    log::info!("Stream: {:?}", stream);
    log::info!("SERVER reading message into branch code");
    let branch_code = read_message(&mut stream).unwrap();
    log::info!("Received branch code: {}", branch_code);

    let branch_code_str = branch_code.to_string();

    let directory_path = format!("data/{}", branch_code_str);
    
    if let Err(e) = fs::create_dir_all(&directory_path) {
        log::info!("Error creating branch directory: {:?}", e);
        write_message(&mut stream, &format!("ERROR: {:?}", e)).expect("Error sending ERROR to client");
        return;
    }

    write_message(&mut stream, "OK").expect("Error sending OK to client");

    let encoded_content = read_message(&mut stream).unwrap();
    log::info!("Received Base64 content: {}", encoded_content);

    write_message(&mut stream, "OK").expect("Error sending OK to client");

    let decoded_content = lib::decode_from_base64(&encoded_content);
    log::info!("Decoded content: {:?}", decoded_content);

    let file_path = format!("data/{}/branch_weekly_sales.txt", branch_code_str);
    if let Err(e) = fs::write(&file_path, decoded_content) {
        log::info!("Error writing to file: {:?}", e);
        write_message(&mut stream, &format!("ERROR: {:?}", e)).expect("Error sending ERROR to client");
        return;
    }

    log::info!("File saved to: {}", file_path);
    write_message(&mut stream, "OK").expect("Error sending OK to client");
}


fn write_message(stream: &mut TcpStream, message: &str) -> io::Result<()> {
    stream.write(message.as_bytes())?;
    Ok(())
}

fn read_message(stream: &mut TcpStream) -> io::Result<String> {
    log::info!("Server Reading Message");
    let mut buffer = Vec::new();
    log::info!("Server stream.read_to_end() called");

    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            log::info!("Server stream.read_to_end() finished. Bytes read: {}", bytes_read);

            if bytes_read == 0 {
                log::info!("Connection closed by sender");
                return Ok(String::new());
            }

            let message = String::from_utf8_lossy(&buffer).to_string();
            log::info!("{}", message);
            Ok(message)
        }
        Err(e) => {
            log::info!("Error reading from stream: {}", e);
            Err(e)
        }
    }
}

