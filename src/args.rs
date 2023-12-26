use structopt::StructOpt;

pub const BITCOIN_PORT: u16 = 8333;

#[derive(StructOpt, Debug, Default)]
#[structopt(name = "peer-to-peer-handshake")]
pub struct Args {
    #[structopt(long, default_value = "bitcoin")]
    network: String,
    #[structopt(long, default_value = "0")]
    concurrent: u8,
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
            concurent: args.concurrent,
            dns_address: args.dns_address,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Network {
    Bitcoin,
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("bitcoin" => Network::Bitcoin; "TEST_1")]
    #[test_case("BITCOIN" => Network::Bitcoin; "TEST_2")]
    #[test_case("BiTcOiN" => Network::Bitcoin; "TEST_3")]
    #[test_case("Etherium" => panics "Unsupported network"; "TEST_4")]
    fn test_config_from_args(network: &str) -> Network {
        let mut args = Args::default();
        args.network = network.to_string();

        Config::from(args).network
    }
}
