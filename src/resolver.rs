use std::net::IpAddr;

use futures::stream::LocalBoxStream;
use futures::{future, StreamExt};
use trust_dns_resolver::config::{ResolverConfig, ResolverOpts};
use trust_dns_resolver::{AsyncResolver, TokioAsyncResolver};

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
    resolver: TokioAsyncResolver,
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
        let resolver = AsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default())
            .expect("Failed to create resolver");

        Self {
            seed_nodes,
            resolver,
        }
    }

    pub async fn resolve_bitcoin_addresses(&'a self) -> LocalBoxStream<'a, IpAddr> {
        futures::stream::iter(self.seed_nodes.iter())
            .then(move |seed_node| self.resolver.lookup_ip(*seed_node))
            .filter(|response| future::ready(response.is_ok()))
            .flat_map(|response| futures::stream::iter(response.unwrap().into_iter()))
            .boxed_local()
    }
}
