use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::{SystemTime, UNIX_EPOCH};

use bitcoin::p2p::{self, address, message, message_network};
use rand::{thread_rng, Rng};

/// Building version message, see https://en.bitcoin.it/wiki/Protocol_documentation#version
pub fn build_version_message(address: SocketAddr) -> message::NetworkMessage {
    let my_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0);

    // "bitfield of features to be enabled for this connection"
    let services = p2p::ServiceFlags::NONE;

    // "standard UNIX timestamp in seconds"
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time error")
        .as_secs();

    // "The network address of the node receiving this message"
    let addr_recv = address::Address::new(&address, p2p::ServiceFlags::NONE);

    // "The network address of the node emitting this message"
    let addr_from = address::Address::new(&my_address, p2p::ServiceFlags::NONE);

    // "Node random nonce, randomly generated every time a version packet is sent. This nonce is used to detect connections to self."
    let nonce: u64 = thread_rng().gen();

    // "User Agent (0x00 if string is 0 bytes long)"
    let user_agent = String::from("rust-example");

    // "The last block received by the emitting node"
    let start_height: i32 = 0;

    // Construct the message
    message::NetworkMessage::Version(message_network::VersionMessage::new(
        services,
        timestamp as i64,
        addr_recv,
        addr_from,
        nonce,
        user_agent,
        start_height,
    ))
}
