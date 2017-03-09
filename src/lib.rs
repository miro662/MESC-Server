pub mod user {

  pub struct User {
    socket: ::std::net::TcpStream
  }

  impl User {
    pub fn new(stream: ::std::net::TcpStream) -> User {
      User{socket: stream}
    }

    pub fn print_addr(&self) {
      ::std::thread::sleep(::std::time::Duration::from_millis(2000));
      println!("{}", self.socket.peer_addr().unwrap());
    }
  }
}
