use std::net::{IpAddr, Ipv4Addr};
use pnet::packet::{
    ip::IpNextHeaderProtocols::Icmp,
    FromPacket,
    Packet,
    icmp::{
        echo_request::MutableEchoRequestPacket, 
        IcmpPacket
    }
};
use pnet::transport::{TransportProtocol, icmp_packet_iter, transport_channel, TransportChannelType::Layer4};
use std::{thread, time::{Duration, Instant}};

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
    let mut requests = 0; // the echo request count 
    let mut replies = 0; // the recho reply count
    let mut min: Duration;
    let mut max: Duration;
    let mut rtts: Vec<Duration>;
    ctrlc::set_handler(move || calc_stats(requests, replies, min, max, rtts))
        .expect("Could not calculate stats");
    let prot = TransportProtocol::Ipv4(Icmp);
    // even though ICMP is a layer 3 (network layer) protocol,
    // pnet needs to open a layer 4 (transport layer) channel
    let chan = transport_channel(64, Layer4(prot)); 
    
    // destructure the channel to access the send and recv structs
    let (mut send, mut recv) = match chan {
        Ok(s) => s,
        Err(e) => panic!("could not create channel: {}", e)
    };
   
    let mut buf = [0u8; 64]; // buffer for the Echo Request packet

    // packet iterator that takes receiver struct and returns each
    // echo reply on request
    let mut piter = icmp_packet_iter(&mut recv);
    println!("pinging");

    let timeout = Duration::from_millis(freq);  // the wait time between echo
                                                // requests

    loop {
        let packet = MutableEchoRequestPacket::new(&mut buf).unwrap();
        //packet.set_payload(&[requests]);
        let sent = send.send_to(packet, ip);
        let now=Instant::now(); 
        match sent {
            Ok(_) => {
                requests += 1;
                ()
            },
            Err(e) => panic!("Failed to send ICMP packet, {}", e)
        }

        match piter.next() {
            Ok((pkt, adr)) => {
                println!("{} bytes from {}: icmp_seq={:?} time={:?}", 
                    IcmpPacket::packet_size(&(pkt.from_packet())), 
                    adr, 
                    pkt.payload(), 
                    now.elapsed()
                );
                replies += 1;
            },
            Err(e) => panic!("failed on recv, {}", e)
        }
        thread::sleep(timeout); 
    } 
}

fn calc_stats(requests: u32, replies: u32, min: Duration, max: Duration, rtts: Vec<Duration>) -> () {
    let mut rate: f32 = 0.0; 
    if requests != 0 {
        rate = 100.0 * replies as f32 / requests as f32;
    }

    println!("{} packets transmitted, {} packets received, {}% packet loss", requests, replies, rate);
    println!("rount-trip min/avg/max = {:?}/{:?}/{:?}", min, rtts, max);
}
