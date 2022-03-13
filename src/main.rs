use std::env;

use pnet::{datalink::{self, NetworkInterface, Channel::Ethernet}, packet::MutablePacket};
use pnet::packet::{Packet, ethernet::{EthernetPacket, MutableEthernetPacket}};

fn main() {
    let net_interface = env::args().nth(1).unwrap();

    let interfaces = datalink::interfaces();
    let interface_match = |interface: &NetworkInterface| interface.name == net_interface;
    let interface =interfaces.into_iter().filter(interface_match).next().unwrap();
    println!("Interface: {}", interface.name);

    let (mut tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Not ethernet handler."),
        Err(e) => panic!("Create ethernet handle error. {}", e),
    };

    loop {
        match rx.next() {
            Ok(packet) => {
                let packet = EthernetPacket::new(packet).unwrap();

                tx.build_and_send(1, packet.packet().len(),
                &mut |new_packet| {
                    let mut new_packet = MutableEthernetPacket::new(new_packet).unwrap();
                    new_packet.clone_from(&packet);

                    new_packet.set_source(packet.get_destination());
                    new_packet.set_destination(packet.get_source());
                });
            },
            Err(e) => panic!("ethernet read error. {}", e),
        }
    }
}
