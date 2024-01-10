use std::net::UdpSocket;
use std::{io, str};

pub fn communicate(address: &str) -> Result<(), failure::Error> {
    //ポート0でOSが空きポートを選ぶ
    let socket = UdpSocket::bind("localhost:0")?;
    loop {
        //サーバーへ送信する
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        socket.send_to(input.as_bytes(), address)?;
        //サーバーから受信する
        let mut buffer = [0u8; 1024];
        socket.recv_from(&mut buffer).expect("failed to receive");
        print!(
            "{}",
            str::from_utf8(&buffer).expect("failed to converto to String")
        );
    }
}
