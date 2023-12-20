use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long)]
    network: String,
    #[arg(long)]
    dns_address: String,
}

pub enum Network {
    Bitcoin,
    Testnet,
    Regtest,
}
