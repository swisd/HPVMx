
use std::env;
use std::fs;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Executable: {}", args[0]);

    if args.len() > 1 {
        let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
        println!("Connected to server");
        println!("Command sent, waiting for response...");

        // 2. Read the response into a buffer
        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer).unwrap();

        if bytes_read > 0 {
            let response = String::from_utf8_lossy(&buffer[..bytes_read]);
            println!("Server says: {}", response);
        } else {
            println!("Server closed the connection.");
        }
        let second_arg = &args[1];

        let file = fs::read_to_string(second_arg).unwrap();
        let clean = file.replace("\r", "");
        let text = clean.split("\n").collect::<Vec<&str>>();
        for sentence in text {
            if !sentence.starts_with("--") {
                println!("{}", sentence);
                stream.write_all((sentence.to_owned() + "\n").as_bytes()).unwrap();
                sleep(Duration::from_millis(500));
            }
            // println!("Command sent, waiting for response...");

            // // 2. Read the response into a buffer
            // let mut buffer = [0; 1024];
            // let bytes_read = stream.read(&mut buffer).unwrap();
            //
            // if bytes_read > 0 {
            //     let response = String::from_utf8_lossy(&buffer[..bytes_read]);
            //     println!("Server says: {}", response);
            // } else {
            //     println!("Server closed the connection.");
            // }
        }



    } else {
        println!("No arguments provided.");
    }
}