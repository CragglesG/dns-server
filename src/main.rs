#[allow(unused_imports)]
use std::net::UdpSocket;

struct DnsHeader {
    id: [u8; 2],
    qr: u8,
    opcode: u8,
    aa: u8,
    tc: u8,
    rd: u8,
    ra: u8,
    z: u8,
    rcode: u8,
    qdcount: u16,
    ancount: u16,
    nscount: u16,
    arcount: u16,
}

struct DnsQuestion {}
struct DnsAnswer {}

struct DnsMessage {
    header: DnsHeader,
    question: DnsQuestion,
    answer: DnsAnswer,
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf: [u8; 512] = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);
                let response = DnsHeader {
                    id: [0x04, 0xd2],
                    qr: 0x80,
                    opcode: 0,
                    aa: 0,
                    tc: 0,
                    rd: 0,
                    ra: 0,
                    z: 0,
                    rcode: 0,
                    qdcount: 0,
                    ancount: 0,
                    nscount: 0,
                    arcount: 0,
                };
                let mut response_list: Vec<u8> = Vec::new();
                for byte in response.id.iter() {
                    response_list.push(*byte);
                }
                response_list.push(response.qr);
                response_list.push(response.opcode);
                response_list.push(response.aa);
                response_list.push(response.tc);
                response_list.push(response.rd);
                response_list.push(response.ra);
                response_list.push(response.z);
                response_list.push(response.rcode);
                response_list.push(response.qdcount as u8);
                response_list.push(response.ancount as u8);
                response_list.push(response.nscount as u8);
                response_list.push(response.arcount as u8);
                udp_socket
                    .send_to(&response_list, source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
