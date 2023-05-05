use std::net::{IpAddr, Ipv4Addr};
use pnet::packet::ip::IpNextHeaderProtocols::Icmp;
use pnet::transport::{TransportProtocol, icmp_packet_iter, transport_channel, TransportChannelType::Layer4};
use pnet::packet::icmp::{echo_request::MutableEchoRequestPacket, IcmpPacket};
use std::{thread, time::{Duration, Instant}};
use pnet::packet::FromPacket;


fn main() {
    let ipadd = Ipv4Addr::new(192, 168, 50, 87);
    let _res = ping(IpAddr::V4(ipadd), 1000);
}

/* the `ping` method provides the major functionality of this crate.
 * It sends and receives ICMP packets.
 *
 * @params:
 *  ip - IpAddr enum - holds the target IP for the Echo Request
 *  freq - u64 - holds the frequency, in milliseconds, with which
 *      to send Echo Requests
 *
 *  @returns:
 *      no return value
 *
 */

fn ping(ip: IpAddr, freq: u64) -> (){
    let prot = TransportProtocol::Ipv4(Icmp);
    // even though ICMP is a layer 3 (network layer) protocol,
    // pnet needs to open a layer 4 (transport layer) channel
    let chan = transport_channel(1500, Layer4(prot)); 
    
    // destructure the channel to access the send and recv structs
    let (mut send, mut recv) = match chan {
        Ok(s) => s,
        Err(e) => panic!("could not create channel: {}", e)
    };
   
    let mut buf = [0u8; 1500]; // buffer for the Echo Request packet

    // packet iterator that takes receiver struct and returns each
    // echo reply on request
    let mut piter = icmp_packet_iter(&mut recv);
    println!("running");

    let timeout = Duration::from_millis(freq); 
    let mut seq = 1;
    let mut resp = 0;
    loop {
        let packet = MutableEchoRequestPacket::new(&mut buf).unwrap();
        let sent = send.send_to(packet, ip);
        let now=Instant::now(); 
        match sent {
            Ok(_) => (),
            Err(e) => panic!("Failed to send ICMP packet, {}", e)
        }

        match piter.next() {
            Ok((pkt, adr)) => {
                println!("{} bytes from {}: icmp_seq={} time={:?}", IcmpPacket::packet_size(&(pkt.from_packet())), adr, seq, now.elapsed());
                seq += 1;
            },
            Err(e) => panic!("failed on recv, {}", e)
        }
        thread::sleep(timeout); 
    } 
}
