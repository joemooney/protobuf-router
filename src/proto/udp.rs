use super::ProtoError;
use super::ProtoResult;
use std::net::UdpSocket;
use std::thread::{self, JoinHandle};

pub fn start_listener() -> ProtoResult<JoinHandle<Result<(), ProtoError>>> {
    let socket =
        UdpSocket::bind("127.0.0.1:34254").map_err(|source| ProtoError::UDPBindError { source })?;
    Ok(thread::spawn(move || listener(socket)))
}

pub fn send_message(msg: Vec<u8>) -> ProtoResult<()> {
    // binding to port zero will get an available port for the sender
    let socket =
        UdpSocket::bind("127.0.0.1:0").map_err(|source| ProtoError::UDPBindError { source })?;
    println!("Sending message");
    socket
        .send_to(&msg, "127.0.0.1:34254")
        .map_err(|source| ProtoError::UDPSendError { source })?;
    println!("Successfully sent message");
    Ok(())
}

pub fn listener(socket: UdpSocket) -> ProtoResult<()> {
    // Receives a single datagram message on the socket. If `buf` is too small to hold
    // the message, it will be cut off.
    let mut buf = [0; 10000];

    loop {
        match socket.recv_from(&mut buf) {
            Ok((amt, src)) => {
                let buf = &mut buf[..amt];
                super::receive_test_message(buf)?;
                buf.reverse();
                println!("received message, replying...");
                match socket.send_to(buf, &src) {
                    Ok(_) => (),
                    Err(e) => {
                        eprintln!("Failed to send data: {:?}", e);
                        break;
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed listening {:?}", e);
                break;
            }
        }
    }
    Ok(())
}
