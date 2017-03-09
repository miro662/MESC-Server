extern crate MESC;

use std::net;
use std::thread;
use MESC::user;

fn main() {
  let listener = net::TcpListener::bind("127.0.0.1:2345").unwrap();

  for stream in listener.incoming() {
    match stream {
      Ok(stream) => {
        handle_client(stream);
      },
      Err(e) => {

      },
    }
  }
}

fn handle_client(stream: net::TcpStream) {
  thread::spawn(|| {
    user::User::new(stream).print_addr();
  });
}