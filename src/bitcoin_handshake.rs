mod bitcoin_resolver;
mod version_message;

use self::bitcoin_resolver::BitcoinDnsResolver;
use self::version_message::build_version_message;
use crate::{Config, Result, BITCOIN_PORT};
use std::io::{BufReader, Write};
use std::net::{IpAddr, Shutdown, SocketAddr, TcpStream};

use bitcoin::consensus::{encode, Decodable};
use bitcoin::p2p::message::{self};
use futures::StreamExt;

use tracing::{error, info};

pub async fn run_bitcoin_handshake(config: Config) -> Result<()> {
    let seed_nodes = config
        .dns_address
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>();
    let dns_resolver = BitcoinDnsResolver::new(seed_nodes.as_slice());

    dns_resolver
        .resolve_bitcoin_addresses()
        .await
        .for_each_concurrent(config.concurent as usize, |address| async move {
            let result = bitcoin_handshake(address).await;

            if result.is_err() {
                error!("Failed to handshake with {:?}", address);
            }
        })
        .await;

    Ok(())
}

async fn bitcoin_handshake(address: IpAddr) -> Result<()> {
    let address = SocketAddr::new(address, BITCOIN_PORT);
    let version_message = build_version_message(address);
    let mut stream = TcpStream::connect(address)?;
    let read_stream = stream.try_clone()?;
    let mut stream_reader = BufReader::new(read_stream);
    let first_message =
        message::RawNetworkMessage::new(bitcoin::Network::Bitcoin.magic(), version_message);

    // Send the message
    stream.write_all(encode::serialize(&first_message).as_slice())?;
    info!("Sent version message");

    loop {
        // Loop an retrieve new messages
        let reply = message::RawNetworkMessage::consensus_decode(&mut stream_reader)?;
        match reply.payload() {
            message::NetworkMessage::Version(_) => {
                info!("Received version message: {:?}", reply.payload());

                let second_message = message::RawNetworkMessage::new(
                    bitcoin::Network::Bitcoin.magic(),
                    message::NetworkMessage::Verack,
                );

                stream.write_all(encode::serialize(&second_message).as_slice())?;
                info!("Sent verack message");
            }
            message::NetworkMessage::Verack => {
                info!("Received verack message: {:?}", reply.payload());
                break;
            }
            _ => {
                info!("Received unknown message: {:?}", reply.payload());
                break;
            }
        }
    }

    stream.shutdown(Shutdown::Both)?;

    Ok(())
}
