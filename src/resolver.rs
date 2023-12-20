use std::net::IpAddr;

use trust_dns_resolver::config::{ResolverConfig, ResolverOpts};
use trust_dns_resolver::Resolver;

const DEFAULT_SEED_NODES: &[&str] = &[
    "seed.bitcoin.sipa.be",
    "dnsseed.bluematt.me",
    "seed.bitcoinstats.com",
    "seed.bitcoin.jonasschnelli.ch",
    "seed.btc.petertodd.org",
];

#[derive(Debug)]
pub struct DnsResolver<'a> {
    seed_nodes: &'a [&'a str],
}

impl<'a> Default for DnsResolver<'a> {
    fn default() -> Self {
        Self::new(DEFAULT_SEED_NODES)
    }
}

impl<'a> DnsResolver<'a> {
    pub fn new(seed_nodes: &'a [&'a str]) -> Self {
        let seed_nodes = if seed_nodes.is_empty() {
            DEFAULT_SEED_NODES
        } else {
            seed_nodes
        };

        Self { seed_nodes }
    }

    pub fn resolve_bitcoin_address(&self) -> impl Iterator<Item = IpAddr> + 'a {
        // Create a new resolver with default configuration
        let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default())
            .expect("Failed to create resolver");

        self.seed_nodes.iter().flat_map(move |seed_node| {
            // Resolve the IP addresses for the seed node
            let response = resolver
                .lookup_ip(*seed_node)
                .expect("Failed to resolve IP address");

            // Print the resolved IP addresses
            response.into_iter().map(move |ip_address| {
                println!("Resolved IP address for {}: {}", seed_node, ip_address);
                ip_address
            })
        })
    }
}

// pub fn resolve_bitcoin_address<'a>(seed_nodes: &'a [&'a str]) -> impl Iterator<Item = IpAddr> {
//     // Create a new resolver with default configuration
//     let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default())
//         .expect("Failed to create resolver");

//     // Perform a DNS lookup for Bitcoin seed nodes
//     let seed_nodes = seed_nodes.is_empty().then(|| DEFAULT_SEED_NODES).unwrap(); // won't panic because of the previous `then`

//     seed_nodes.into_iter().flat_map(move |seed_node| {
//         // Resolve the IP addresses for the seed node
//         let response = resolver
//             .lookup_ip(seed_node)
//             .expect("Failed to resolve IP address");

//         // Print the resolved IP addresses
//         response.into_iter().map(move |ip_address| {
//             println!("Resolved IP address for {}: {}", seed_node, ip_address);
//             ip_address
//         })
//     })
// }
