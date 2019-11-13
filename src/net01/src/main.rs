use std::io::prelude::*;
use std::net::TcpStream;
use std::io::{self, Read, Error};
use std::thread;
use std::net::{SocketAddrV4, Ipv4Addr, TcpListener};


fn main() -> Result<(), Error> {
    let loopback = Ipv4Addr::new(127, 0, 0, 1);
    let socket = SocketAddrV4::new(loopback, 42853);
    let listener = TcpListener::bind(socket)?;
    let port = listener.local_addr()?;
    println!("Listening on {}, access this port to end the program", port);

    let (mut tcp_stream, addr) = listener.accept()?; //block  until requested
    println!("Connection received! {:?} is sending data.", addr);

    let mut input = String::new();

    let _ = tcp_stream.read_to_string(&mut input)?;

    println!("{:?} says {}", addr, input);
    Ok(())
}


fn main11() {
    let mut stream = TcpStream::connect("127.0.0.1:7878")
        .expect("Couldn't connect to the server...");
    stream.set_nonblocking(true).expect("set_nonblocking call failed");

    let mut buf = vec![];
    loop {
        match stream.read_to_end(&mut buf) {
            Ok(_) => break,
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // wait until network socket is ready, typically implemented
                // via platform-specific APIs such as epoll or IOCP
                //  wait_for_fd();
                println!("wait_for_fd");
            }
            Err(e) => panic!("encountered IO error: {}", e),
        };
    };
    println!("bytes: {:?}", buf);
}






/*fn main11() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:3306").expect("Couldn't connect to the server...");

    let mut stream_clone = stream.try_clone().expect("clone failed...");

    stream.write(&[1])?;
    stream_clone.read(&mut [0; 128])?;
    Ok(())
}*/

