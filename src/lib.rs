use std::{net::IpAddr, str::FromStr};

pub struct Config {
    pub flag: String,
    pub ipaddr: IpAddr,
    pub threads: u16,
}

impl Config {
    pub fn build<T: Iterator<Item = String>>(args: T) -> Result<Config, &'static str> {
        let args: Vec<String> = args.collect();

        if args.len() < 2 {
            return Err("Not enough arguments");
        } else if args.len() > 4 {
            return Err("Too mant arguments");
        }

        let address = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&address) {
            return Ok(Config {
                flag: String::from(""),
                ipaddr,
                threads: 4,
            });
        }

        let flag = address;
        if flag.contains("-h") || flag.contains("-help") {
            if args.len() == 2 {
                return Err("help");
            } else {
                return Err("To many arguments");
            }
        }

        if flag.contains("-t") {
            let ipaddr = match IpAddr::from_str(&args[3]) {
                Ok(x) => x,
                Err(_) => return Err("Not valid IP address, my be IPV4 or IPV6"),
            };

            let threads = match args[2].parse::<u16>() {
                Ok(num) => num,
                Err(_) => return Err("Failed to parse thread number"),
            };

            return Ok(Config {
                flag,
                ipaddr,
                threads,
            });
        }

        return Err("Invalid Syntax");
    }
}
