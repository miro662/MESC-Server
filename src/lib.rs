pub mod user {

  use std::io::{Read, Write};
  use std::net;
  use std::thread;
  use std::sync::mpsc;

  pub struct User {
    socket: net::TcpStream,
    sender: mpsc::Sender<Data>,
  }

  pub enum Data {
    Msg(Vec<u8>),
  }

  impl User {
    pub fn new(stream: net::TcpStream, sender: mpsc::Sender<Data>) -> User {
      User{socket: stream,
      sender: sender}
    }

    pub fn start_reading(&self) {
        let mut soc_clone = self.socket.try_clone().unwrap();
        let tx = self.sender.clone();
        thread::spawn( move || {
          while let Some(data) = soc_clone.read_line(){
            tx.send(Data::Msg(data));
          }
        });
    }

    pub fn send_msg(&mut self, data: Vec<u8>){
      self.socket.write(&data);
    }
  }

  trait Reader {
    fn read_line(&mut self) -> Option<Vec<u8>>;
  }
  impl Reader for net::TcpStream {
    fn read_line(&mut self) -> Option<Vec<u8>> {
      let mut data: Vec<u8> = Vec::new();
      loop {
        let mut text = [0; 20];
        let n = self.read(&mut text).unwrap_or_default();
        if n > 0 {
          data.append(&mut text[0..n].to_vec());
          if text[n-1] == 10 { // if last read character is newline
              break;
          }
          continue;
        }
        return Option::None;
      }
      let _ = data.pop();
      Option::Some(data)
    }
  }
  
}