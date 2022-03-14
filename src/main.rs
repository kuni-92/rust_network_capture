extern crate pnet;

use pnet::packet::{MutablePacket, Packet};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::transport::TransportChannelType::Layer4;
use pnet::transport::TransportProtocol::Ipv4;
use pnet::transport::{transport_channel, udp_packet_iter};
use pnet::packet::udp::MutableUdpPacket;

fn main() {
    let  protocol = Layer4(Ipv4(IpNextHeaderProtocols::Test1));

    let(mut tx, mut rx) = match transport_channel(4096, protocol) {
        Ok((tx, rx)) => (tx, rx),
        Err(e) => panic!("An error occurred when creating the transport channel: {}", e),
    };

    let mut iter = udp_packet_iter(&mut rx);

    loop {
        println!("Start capture");
        match iter.next() {
            Ok((packet, addr)) => {
                let mut vec: Vec<u8> = vec![0; packet.packet().len()];
                println!("Receve packet: {:?}", vec);
                let mut new_packet = MutableUdpPacket::new(&mut vec[..]).unwrap();
                
                new_packet.clone_from(&packet);

                new_packet.set_source(packet.get_destination());
                new_packet.set_destination(packet.get_source());

                match tx.send_to(new_packet, addr) {
                    Ok(n) => assert_eq!(n, packet.packet().len()),
                    Err(e) => panic!("failed to send packet: {}", e),
                }
            }
            Err(e) => {
                panic!("An error occurred while reading: {}", e);
            }
        }
    }
}
