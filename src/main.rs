use clap::{App, Arg};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::{thread, time};
use chrono::{Timelike, Utc};

fn handle_read(mut stream: &TcpStream) {
    let mut buf = [0u8; 4096];
    match stream.read(&mut buf) {
        Ok(_) => {
            let req_str = String::from_utf8_lossy(&buf);
            println!("{}", req_str);
        }
        Err(e) => println!("Unable to read stream: {}", e),
    }
}

fn handle_write(mut stream: TcpStream, name: String) {
    let now = Utc::now();
    let time = format!("{}:{}:{}", now.hour(), now.minute(), now.second());
    let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>\'{}\' sent you this message at {}</body></html>\r\n", name, time);

    match stream.write(response.as_bytes()) {
        Ok(_) => println!("Response sent"),
        Err(e) => println!("Failed sending response: {}", e),
    }
}

fn handle_client(stream: TcpStream, name: String) {
    handle_read(&stream);
    handle_write(stream, name);
}

fn main() {
    let matches = App::new("slow-http-server")
        .version("1.0")
        .author("lor-enz <git@lorenz.kiwi>")
        .about("Is slow, but reliably slow!")
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .value_name("port")
                .help("The port the http will run on.")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("slowness")
                .short("s")
                .long("slowness")
                .value_name("slowness")
                .help("The delay in seconds the server will take to respond.")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("name")
                .short("n")
                .long("name")
                .value_name("name")
                .help("The name of the server which will be shown in the body.")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let port = matches
        .value_of("port")
        .unwrap()
        .to_string()
        .parse::<i32>()
        .unwrap();

    let slowness: u64 = matches
        .value_of("slowness")
        .unwrap()
        .to_string()
        .parse::<u64>()
        .unwrap();

    let name: String = matches.value_of("name").unwrap().to_string();

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    println!(
        "\'{}\' is listening for connections on {} and will respond with delay of {} seconds.",
        name,
        format!("127.0.0.1:{}", port),
        slowness
    );

    for stream in listener.incoming() {
        let name = name.clone();
        match stream {
            Ok(stream) => {
                // let's make things slow
                thread::sleep(time::Duration::from_secs(slowness));
                thread::spawn(move || handle_client(stream, name));
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}
