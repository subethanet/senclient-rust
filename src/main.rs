use std::io::{stdin, stdout, Write, Read};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    stream.write(&[1]); // ignore the Result
    //println!("{}", stream.read())
}

fn server_listen() {
    let listener = TcpListener::bind("127.0.0.1:4242").unwrap();

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream.unwrap());
        println!("loop");
    }
}

fn client_connection() {
    let mut stream = TcpStream::connect("127.0.0.1:4242").unwrap();
    let _ = stream.write(&[1]); // ignore the Result
    let response = stream.read(&mut [0; 128]).unwrap(); // ignore this too
    println!("{}", response);
}

fn main() {
    println!("Hello, world!");

    let mut s=String::new();
    print!("Please enter some text: ");
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }

    if s == "server" {
        server_listen();
    }
    else {
        println!("assuming client mode");
        client_connection();
    }
}
