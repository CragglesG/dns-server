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

struct DnsQuestion {
    name: String,
    typ: u16,
    class: u16,
}

struct DnsAnswer {
    name: String,
    typ: u16,
    class: u16,
    ttl: u32,
    rdlength: u16,
    rdata: Vec<u8>,
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
                let id = [buf[0], buf[1]];
                let opcode = (buf[2] & 0b01111000) >> 3;
                let rd = buf[2] & 0b00000001;
                let rcode: u8;
                if opcode == 0 {
                    rcode = 0;
                } else {
                    rcode = 4;
                }

                let header = DnsHeader {
                    id,
                    qr: 1,
                    opcode,
                    aa: 0,
                    tc: 0,
                    rd,
                    ra: 0,
                    z: 0,
                    rcode,
                    qdcount: 1,
                    ancount: 1,
                    nscount: 0,
                    arcount: 0,
                };

                let mut header_res: Vec<u8> = Vec::new();

                let flags: u16 = ((header.qr as u16) << 15)
                    | ((header.opcode as u16) << 11)
                    | ((header.aa as u16) << 10)
                    | ((header.tc as u16) << 9)
                    | ((header.rd as u16) << 8)
                    | ((header.ra as u16) << 7)
                    | ((header.z as u16) << 4)
                    | (header.rcode as u16);

                header_res.extend_from_slice(&header.id);
                header_res.extend_from_slice(&flags.to_be_bytes());
                header_res.extend_from_slice(&header.qdcount.to_be_bytes());
                header_res.extend_from_slice(&header.ancount.to_be_bytes());
                header_res.extend_from_slice(&header.nscount.to_be_bytes());
                header_res.extend_from_slice(&header.arcount.to_be_bytes());

                let question = DnsQuestion {
                    typ: 1,
                    class: 1,
                    name: "\x0ccodecrafters\x02io\x00".to_string(),
                };

                let mut question_res = Vec::new();

                question_res.extend_from_slice(question.name.as_bytes());
                question_res.extend_from_slice(&question.typ.to_be_bytes());
                question_res.extend_from_slice(&question.class.to_be_bytes());

                let answer = DnsAnswer {
                    name: "\x0ccodecrafters\x02io\x00".to_string(),
                    typ: 1,
                    class: 1,
                    ttl: 60,
                    rdlength: 4,
                    rdata: vec![8, 8, 8, 8],
                };

                let mut answer_res = Vec::new();

                answer_res.extend_from_slice(answer.name.as_bytes());
                answer_res.extend_from_slice(&answer.typ.to_be_bytes());
                answer_res.extend_from_slice(&answer.class.to_be_bytes());
                answer_res.extend_from_slice(&answer.ttl.to_be_bytes());
                answer_res.extend_from_slice(&answer.rdlength.to_be_bytes());
                answer_res.extend_from_slice(&answer.rdata);

                udp_socket
                    .send_to(&([header_res, question_res, answer_res].concat()), source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
