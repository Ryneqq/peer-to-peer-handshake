use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long)]
    network: Option<String>,
    #[arg(long)]
    dns_address: Option<String>,
}

pub enum Network {
    Bitcoin,
    Testnet,
    Regtest,
}
