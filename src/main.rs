extern crate MESC;

use std::net;
use std::thread;
use std::sync::mpsc;
use MESC::user;
use MESC::user::Data;

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
    let mut u = user::User::new(stream, tx);
    u.start_reading();
    thread::spawn(move || {
      while let Ok(Data::Msg(mut data)) =  rx.recv() {
        data.push('\n'); // Adding new line at the end
        u.send_msg(&data);
      }
      println!("closed!");
    });
  });
}