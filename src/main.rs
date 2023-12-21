use std::io::{BufReader, Write};
use std::net::{Shutdown, SocketAddr, TcpStream};

use bitcoin::consensus::{encode, Decodable};
use bitcoin::p2p::message;
use clap::Parser;
use futures::StreamExt;
use peer_to_peer_handshake::{build_version_message, Args, DnsResolver};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream as AsyncTcpStream;
use tokio::task;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let dns_resolver = DnsResolver::default();
    let mut possible_addresses = dns_resolver.resolve_bitcoin_addresses().await;
    let address = possible_addresses
        .next()
        .await
        .expect("Failed to resolve IP address");
    let address = SocketAddr::new(address, 8333); // TODO incorporate it into the DnsResolver with an enum regarding type of network we are trying to reach
    let version_message = build_version_message(address);
    let first_message =
        message::RawNetworkMessage::new(bitcoin::Network::Bitcoin.magic(), version_message);
    let mut stream = AsyncTcpStream::connect(address).await.unwrap();
    // Send the message
    let _ = stream
        .write_all(encode::serialize(&first_message).as_slice())
        .await;

    println!("Sent version message");

    // Setup StreamReader
    let read_stream = stream.try_clone().unwrap();
    let mut stream_reader = AsyncBufReader::new(read_stream);

    loop {
        // Loop an retrieve new messages
        let reply = message::RawNetworkMessage::consensus_decode(&mut stream).unwrap();
        match reply.payload() {
            message::NetworkMessage::Version(_) => {
                println!("Received version message: {:?}", reply.payload());

                let second_message = message::RawNetworkMessage::new(
                    bitcoin::Network::Bitcoin.magic(),
                    message::NetworkMessage::Verack,
                );

                let _ = stream.write_all(encode::serialize(&second_message).as_slice());
                println!("Sent verack message");
            }
            message::NetworkMessage::Verack => {
                println!("Received verack message: {:?}", reply.payload());
                break;
            }
            _ => {
                println!("Received unknown message: {:?}", reply.payload());
                break;
            }
        }
    }
    let _ = stream.shutdown(Shutdown::Both);
}
