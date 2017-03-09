extern crate MESC;

use std::net;
use std::thread;
use std::sync::mpsc;
use MESC::user;

fn main() {
  let listener = net::TcpListener::bind("127.0.0.1:2345").unwrap();

  for stream in listener.incoming() {
    match stream {
      Ok(stream) => handle_client(stream),
      Err(e) => panic!(e)
    }
  }
}

fn handle_client(stream: net::TcpStream) {
  thread::spawn(|| {
    let (tx, rx) = mpsc::channel();
    let u = user::User::new(stream, tx);
    u.start_reading();
    thread::spawn(move || {
      while let Ok(data) =  rx.recv() {
        println!("{:?}", String::from_utf8(data).unwrap());
      }
      println!("closed!");
    });
  });
}