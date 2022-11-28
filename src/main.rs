use std::env;
use std::io;
use std::io::Write;
use std::net::{IpAddr, TcpStream};
use std::process;
use std::sync::mpsc;
use std::thread;

use port_sniffer::Config;

const MAX: u16 = 65535;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        if err.contains("help") {
            println!(
                "Usage: -t to select how many threads you want\r\n-h or -help to show this message"
            );
        } else {
            eprintln!("Problem parsing arguments: {err}");
        }
        process::exit(0);
    });

    let ipaddr = config.ipaddr;
    let num_threads = config.threads;
    let (tx, rx) = mpsc::channel();

    for i in 0..num_threads {
        let tx = tx.clone();

        thread::spawn(move || {
            scan(tx, i, ipaddr, num_threads);
        });
    }

    drop(tx);

    let mut out = vec![];

    for p in rx {
        out.push(p);
    }

    println!("");
    out.sort();
    for v in out {
        println!("{} is open", v);
    }
}

fn scan(tx: mpsc::Sender<u16>, start_port: u16, ipaddr: IpAddr, num_threads: u16) {
    let mut port = start_port + 1;
    loop {
        match TcpStream::connect((ipaddr, port)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }

        if MAX - port <= num_threads {
            break;
        }
        port += num_threads;
    }
}
