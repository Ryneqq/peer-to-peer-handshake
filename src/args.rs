use structopt::StructOpt;

pub const BITCOIN_PORT: u16 = 8333;

#[derive(StructOpt, Debug)]
#[structopt(name = "p2p-handshake")]
pub struct Args {
    #[structopt(long, default_value = "bitcoin")]
    network: String,
    #[structopt(long, default_value = "0")]
    concurent: u8,
    #[structopt(long)]
    dns_address: Vec<String>,
}

pub struct Config {
    pub network: Network,
    pub concurent: u8,
    pub dns_address: Vec<String>,
}

impl From<Args> for Config {
    fn from(args: Args) -> Self {
        let network = match args.network.to_lowercase().as_str() {
            "bitcoin" => Network::Bitcoin,
            _ => panic!("Unsupported network"),
        };

        Self {
            network,
            concurent: args.concurent,
            dns_address: args.dns_address,
        }
    }
}

pub enum Network {
    Bitcoin,
}
