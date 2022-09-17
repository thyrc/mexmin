use std::net::IpAddr;
use std::str::FromStr;

use clap::{Arg, Command};
use maxminddb::{geoip2, Reader};

fn main() {
    let cli_arguments = Command::new("mexmin")
        .version("0.1.2")
        .about("Dump MaxMind GeoIP DB info")
        .arg(
            Arg::new("database")
                .short('d')
                .long("database")
                .value_name("DATABASE")
                .default_value("/tmp/GeoLite2-City.mmdb")
                .help("Sets GeoIP / GeoLite DB")
                .takes_value(true),
        )
        .arg(
            Arg::new("dump")
                .short('a')
                .long("dump")
                .help("Dump all available fields"),
        )
        .arg(
            Arg::new("ip")
                .help("Sets the IP(s) to look up")
                .value_name("IP")
                .index(1)
                .multiple_values(true),
        )
        .get_matches();

    let reader = Reader::open_readfile(
        cli_arguments
            .get_one("database")
            .map_or("/tmp/GeoLite2-City.mmdb", std::string::String::as_str),
    )
    .unwrap();

    for arg in cli_arguments.get_many::<String>("ip").unwrap() {
        let ip: IpAddr = FromStr::from_str(arg).unwrap();
        let city: geoip2::City = reader.lookup(ip).unwrap();
        if cli_arguments.contains_id("dump") {
            println!("{:#?}", city);
        } else if let Some(city) = city.city {
            if let Some(name) = city.names {
                // println!("{:#?}", name.en);
                match name.get("en") {
                    Some(name) => println!("{}", name),
                    _ => continue,
                }
            }
        }
    }
}
