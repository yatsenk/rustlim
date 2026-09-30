use pnet::datalink::Channel::Ethernet;
use pnet::datalink;
use pnet::packet::ethernet::EthernetPacket;
use pnet::packet::Packet;
use pnet::packet::PacketSize;
use pnet::packet::FromPacket;
use std::thread;

fn main() {
    let interfaces = datalink::interfaces();
    let mut handles = vec![];

    for interface in interfaces {
        let handle = thread::spawn(move || {
            capture_packets(interface);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn capture_packets(interface: datalink::NetworkInterface) {
    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type: {}", &interface),
        Err(e) => panic!(
            "An error occurred when creating the datalink channel: {}",
            e
        ),
    };

    println!("Start thread reading packet on interface: {}", &interface);
        loop {
            match rx.next() {
                Ok(packet) => {
                    if let Some(ethernet_packet) = EthernetPacket::new(packet) {
                        println!("New packet on {}", interface.name);
                        println!("{} => {} packet size: {}",
                            ethernet_packet.get_destination(),
                            ethernet_packet.get_source(),
                            ethernet_packet.packet_size(),
                        );
                        let packet = ethernet_packet.packet();
                        println!("packet: {:?}", packet);
                        println!("---");
                    }
                }
                Err(e)=> {
                    panic!("An error occurred while reading: {}", e);
                }
            }
        }
    }


