use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:3000").expect("couldn't bind to address!");
    println!("Listening on 127.0.0.1:3000");
    let str = recv(&socket);

    while str != "BYE" {
        println!("{}", str);
        recv(&socket);
    }

}

fn recv(socket: &UdpSocket) -> String {
    let mut buf = [0; 10];
    let (amt, _) = socket.recv_from(&mut buf).expect("no message received!");
    let buf = &mut buf[..amt];


    let str = std::str::from_utf8(buf).expect("error converting data!").to_owned();
    str
}
