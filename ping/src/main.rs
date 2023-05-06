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
use std::{thread, process, time::{Duration, Instant}};
use std::sync::{
    Arc,
    Mutex,
};

struct State {
    requests: u32,
    replies: u32,
}

impl State {
    fn new() -> Self {
        State {
            requests: 0,
            replies: 0,
        }
    }
}

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
    let state = Arc::new(Mutex::new(State::new()));
   
    // clone state and give it to the closure handling Ctrl+C input
    let cstate = state.clone();
    ctrlc::set_handler(move || calc_stats(&cstate))
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
        //packet.set_payload(&[]);
        let sent = send.send_to(packet, ip);
        let now=Instant::now(); // begin timing the reply
        match sent {
            Ok(_) => {
                // send was successful, increment request count
                (*state.lock().unwrap()).requests += 1;
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
                // there was a reply, increment the reply count
                (*state.lock().unwrap()).replies += 1;
            },
            Err(e) => panic!("failed on recv, {}", e)
        }
        thread::sleep(timeout); 
    } 
}

fn calc_stats(state: &Mutex<State>) -> () {
    let mut rate: f32 = 0.0; 
    let state = state.lock().unwrap();
    if state.requests != 0 {
        rate = 100.0 * (((state.requests as f32) - (state.replies as f32)) / state.requests as f32);
    }

    println!("{} packets transmitted, {} packets received, {}% packet loss", state.requests, state.replies, rate);

    process::exit(0);
}
