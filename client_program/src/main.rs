use std::net::TcpStream;
use std::io::{self, Read, Write};
use std::fs::File;
use log::{error, info};
use env_logger;
mod lib;

fn main() {
    env_logger::init();
    start_data_transfer("127.0.0.1:7878", "CTONGA").unwrap_or_else(|e| {
        panic!("Error: {}", e);
    });
}  

fn start_data_transfer(server_addr: &str, branch_code: &str) -> io::Result<()> {
    let content = read_file_content("C:\\Users\\csuser\\SysProgProject3\\summary.txt")?;
    let encoded_content = lib::encode_to_base64(&content);

    let mut stream = TcpStream::connect(server_addr)?;
    println!("{} , {} , {} , {}", server_addr , branch_code , content , encoded_content);
    println!("Connected to server");

    write_message(&mut stream, &format!("bcode~{}", branch_code))?;
    println!("CLIENT FIRST write_message() SUCCESSFUL");
    let response = read_message(&mut stream)?;
    println!("{}", response);

    if response.trim() == "OK" {
        println!("respone == OK");
        write_message(&mut stream, &format!("~{}~", encoded_content))?;
        println!("CLIENT SECOND write_message() SUCCESSFUL");
        stream.shutdown(std::net::Shutdown::Write)?;
        println!("CLIENT stream.shutdown(std::net::Shutdown::Write) SUCCESSFUL");
        let response = read_message(&mut stream)?;

        if response.trim() == "OK" {
            println!("File transferred successfully");
        } else {
            panic!("Unexpected response from server");
        }
    } else {
        panic!("Unexpected response from server");
    }

    Ok(())
}

fn read_file_content(file_path: &str) -> io::Result<String> {
    let mut content = String::new();
    let mut file = File::open(file_path)?;
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn write_message(stream: &mut TcpStream, message: &str) -> io::Result<()> {
    println!("Writing Message");
    stream.write(message.as_bytes())?;
    println!("stream.write_all()");
    stream.flush()?;
    println!("stream.flush()");
    Ok(())
}

fn read_message(stream: &mut TcpStream) -> io::Result<String> {
    println!("Client reading Message");
    let mut buffer = [0; 1024];

    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            println!("Client Side stream.read() finished. Bytes read: {}", bytes_read);

            if bytes_read == 0 {
                println!("Connection closed by sender");
                return Ok(String::new());
            }

            let message = String::from_utf8_lossy(&buffer[0..bytes_read]).to_string();
            println!("{}", message);
            Ok(message)
        }
        Err(e) => {
            println!("Error reading from stream: {}", e);
            Err(e)
        }
    }
}


