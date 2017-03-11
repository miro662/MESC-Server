pub mod user {

  use std::io::{Read, Write, BufRead, BufReader};
  use std::net;
  use std::thread;
  use std::sync::mpsc;

  pub struct User
  {
    socket: net::TcpStream,
    sender: mpsc::Sender<Data>,
  }

  pub enum Data {
    Msg(String),
  }

  impl User {
    pub fn new(stream: net::TcpStream, sender: mpsc::Sender<Data>) -> User {
      User{socket: stream,
      sender: sender}
    }

    pub fn start_reading(&self) {
        let mut soc_clone = self.socket.try_clone().unwrap();
        let reader = BufReader::new(soc_clone);
        let tx = self.sender.clone();
        thread::spawn( move || {
          for data in reader.lines() {
            tx.send(Data::Msg(data.unwrap()));
          }
        });
    }

    pub fn send_msg(&mut self, data: &str){
      self.socket.write(data.as_bytes());
    }
  }
}