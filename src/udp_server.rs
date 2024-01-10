use std::net::UdpSocket;
use std::str;

pub fn serve(address: &str) -> Result<(), failure::Error> {
    //一つのソケットが全てのクライアントとの通信を捌く
    let server_socket = UdpSocket::bind(address)?;

    loop {
        //クライアントから受信する
        let mut buf = [0u8; 1024];
        let (size, src) = server_socket.recv_from(&mut buf)?;
        debug!("Handling data from {}", src);
        print!("{}", str::from_utf8(&buf[..size])?);
        //クライアントへ送り返す
        server_socket.send_to(&buf, src)?;
    }
}
