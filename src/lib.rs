#![feature(trait_alias)]

mod args;
mod resolver;
mod version_message;

pub use self::args::{Args, Network};
pub use self::resolver::DnsResolver;
pub use self::version_message::build_version_message;
