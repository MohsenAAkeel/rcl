use std::net::{IpAddr, Ipv4Addr};
use pnet::packet::ip::IpNextHeaderProtocols::Icmp;
use pnet::transport::{icmp_packet_iter, transport_channel, TransportChannelType::Layer3};
use pnet::packet::icmp::{echo_request::MutableEchoRequestPacket, echo_reply::EchoReplyPacket};

fn main() {
    let ipadd = Ipv4Addr::new(192, 168, 50, 7);
    let _res = ping(IpAddr::V4(ipadd));
}

fn ping(ip: IpAddr) -> std::io::Result<()>{
    let prot = Icmp;
    let chan = transport_channel(1500, Layer3(prot)); 
    
    let (mut send, mut recv) = match chan {
        Ok(s) => s,
        Err(e) => panic!("could not create channel: {}", e)
    };
    println!("min: {}", MutableEchoRequestPacket::minimum_packet_size());
    let mut buf = [0u8; 1500];

    let packet = MutableEchoRequestPacket::new(&mut buf).unwrap();
    let sent = send.send_to(packet, ip);
    match sent {
        Ok(m) => println!("received: {}", m),
        Err(e) => panic!("failed to send, {}", e)
    }
    let mut piter = icmp_packet_iter(&mut recv);
    println!("running");
    loop {
        match piter.next() {
            Ok(r) => {
                let (p, _ipadd) = r; 
                println!("checksum: {:?}", p);
            },
            Err(e) => panic!("failed on recv, {}", e)
        }
        println!("running");
        
    } 
    Ok(())
}
