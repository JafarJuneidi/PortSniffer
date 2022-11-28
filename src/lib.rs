use std::io::{self, Write};
use std::net::{IpAddr, Ipv4Addr};
use std::sync::mpsc;
use tokio::net::TcpStream;
use tokio::task;

use bpaf::Bpaf;

const IPFALLBACK: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
const MAX: u16 = 65535;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
pub struct Config {
    #[bpaf(long, short, fallback(IPFALLBACK))]
    /// The address that you want to sniff. Must be a valid Ipv4 address. Falls back to 127.0.0.1
    pub address: IpAddr,

    #[bpaf(
        long("start"),
        short('s'),
        fallback(1u16),
        guard(start_port_guard, "Must be greater than 0")
    )]
    /// The start port for the sniffer. (must be greater than 0)
    pub start_port: u16,

    #[bpaf(
        long("end"),
        short('e'),
        fallback(MAX),
        guard(end_port_guard, "Must be less than or equal to 65535")
    )]
    /// The end port for the sniffer. (must be less than or equal to 65535)
    pub end_port: u16,
}

fn start_port_guard(input: &u16) -> bool {
    *input > 0
}

fn end_port_guard(input: &u16) -> bool {
    *input <= MAX
}

async fn scan(tx: mpsc::Sender<u16>, port: u16, addr: IpAddr) {
    match TcpStream::connect(format!("{}:{}", addr, port)).await {
        Ok(_) => {
            print!(".");
            io::stdout().flush().unwrap();
            tx.send(port).unwrap();
        }
        Err(_) => {}
    }
}

#[tokio::main]
pub async fn run() {
    let opts: Config = config().run();

    let (tx, rx) = mpsc::channel();
    for i in opts.start_port..opts.end_port {
        let tx = tx.clone();

        task::spawn(async move {
            scan(tx, i, opts.address).await;
        });
    }

    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p);
    }

    println!("");
    out.sort();
    for v in out {
        println!("{} is open", v);
    }
}
