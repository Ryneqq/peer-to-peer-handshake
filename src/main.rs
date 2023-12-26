use std::io;
use std::io::{BufReader, Write};
use std::net::{IpAddr, Shutdown, SocketAddr, TcpStream};

use bitcoin::consensus::{encode, Decodable};
use bitcoin::p2p::message::{self, RawNetworkMessage, MAX_MSG_SIZE};
use futures::{future, stream, StreamExt, TryStreamExt};
use peer_to_peer_handshake::{
    build_version_message, Args, Config, DnsResolver, Error, Result, BITCOIN_PORT,
};
use std::sync::Arc;
use structopt::StructOpt;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream as AsyncTcpStream;
use tokio::sync::Mutex;
use tokio::task;
use tracing::{error, info};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    let args = Args::from_args();
    let config = Config::from(args);
    let subscriber = FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set tracing subscriber");
    let dns_resolver = DnsResolver::default();

    dns_resolver
        .resolve_bitcoin_addresses()
        .await
        .for_each_concurrent(config.concurent as usize, |address| async move {
            let result = handshake(address).await;

            if result.is_err() {
                error!("Failed to handshake with {:?}", address);
            }
        })
        .await;
}

async fn handshake(address: IpAddr) -> Result<()> {
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
