## Disclaimer

This project was created as an exercise for interview. It does not have any real life usage as it only showcase high level production code example with usage libraries such as thiserror, tracing, structopt etc. Project was designed to have in mind:
* to be idiomatic,
* to be self explanatory as in Clean Code paradigm,
* to be extentandable,
* to be fast enough,
* to ilustrate good SOLID practicies (as far as it applies to rust and this particular task)

I failed to use tokio::TcpStream because it brought unknown overhead. I spend most of implementation time on this issue and decided to move on with std::net::TcpStream instead so i can actually finish this assignment.

It includes only one set of unit tests not to waste each others time.

## Overview

Program perform asynchronous handshake with all found peers for Bitcoin network. It performs search for peers in dns seeds and perform handshake (Version, Vrack).

It accepts following parameters:

```
USAGE:
    peer-to-peer-handshake [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --concurrent <concurrent>            [default: 0]
        --dns-address <dns-address>...
        --network <network>                [default: bitcoin]
```

* concurrent flags accept u8 and it limits execution to given number of not ready futures polled at a time (it does not involve tasks). `0` means no limits.
* dns-address accept vector of strings which are dns seeds for given network
* network flag accepts for now only bitcoin value.

Since all options have proper defaults, prefered way of running project is as simple as
```
cargo run --release
```

Default search is done on given dns seeds:
```
"seed.bitcoin.sipa.be",
"dnsseed.bluematt.me",
"seed.bitcoinstats.com",
"seed.bitcoin.jonasschnelli.ch",
"seed.btc.petertodd.org",
```

## Improvements

I can see that the project could be improved by adding possibility to connect to other networks such as BitcoinTest or even other p2p networks.

It also could have more tests, especially acceptance but to be frank i dont think it is good to include network tests without secure host.

Probably could be faster or could use tasks.