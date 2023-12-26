use peer_to_peer_handshake::{run_bitcoin_handshake, Args, Config, Network, Result};
use structopt::StructOpt;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::from_args();
    info!("Starting with args: {:?}", args);
    let config = Config::from(args);
    let subscriber = FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set tracing subscriber");

    match config.network {
        Network::Bitcoin => run_bitcoin_handshake(config).await?,
    }

    Ok(())
}
