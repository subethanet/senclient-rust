use std::io::{stdin, stdout, Write, Read};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

struct Link<'a> {
    connections: Vec<Sender<&'a str>>,
}

fn new_client_connection() -> Link<'static> {
    return Link {
        connections: Vec::new()
    };
}

struct TcpWrapper<'a> {
    to_send: Receiver<&'a str>,
    stream: TcpStream
}

fn inbound_loop(mut tcp_wrapper: TcpWrapper) {
    loop {
        // get message from channel
        let msg = tcp_wrapper.to_send.recv().unwrap();

        let write_resp = tcp_wrapper.stream.write(msg.as_bytes()).unwrap(); // ignore the Result
        println!("Write resp: {}", write_resp);
        let resp = tcp_wrapper.stream.read(&mut [0; 128]); // ignore this too
        //println!("{}", resp)
    }
}

fn server_listen(mut link: Link) {
    let listener = TcpListener::bind("127.0.0.1:4242").unwrap();

    // accept connections and process them serially
    for mut stream in listener.incoming() {
        let (sender, receiver): (Sender<&str>, Receiver<&str>) = mpsc::channel();

        let mut tcp_wrapper = TcpWrapper {
            to_send: receiver,
            stream: stream.unwrap(),
        };

        link.connections.push(sender);
        link.connections[0].send("hello");

        inbound_loop(tcp_wrapper);
        println!("got connection");
    }
}

fn client_connection() {
    let mut stream = TcpStream::connect("127.0.0.1:4242").unwrap();
    loop {
        let _ = stream.write(&[1]); // ignore the Result

        let mut data: [u8; 128] = [0; 128];
        let response = stream.read(&mut data).unwrap(); // ignore this too
        println!("read response: {} {}", response, data[0]);
    }
}

fn main() {
    println!("Hello, world!");
    let mut conn = new_client_connection();

    let mut s= String::new();
    print!("Please enter some text: ");
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }

    if s == "server" {
        server_listen(conn);
    }
    else {
        println!("assuming client mode");
        client_connection();
    }
}
