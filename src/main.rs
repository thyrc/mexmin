use maxminddb;

use std::net::IpAddr;
use std::str::FromStr;

use maxminddb::geoip2;

extern crate clap;
use clap::{App, Arg};

fn main() {
    let cli_arguments = App::new("mexmin")
                            .version("0.1.1")
                            .about("Dump MaxMind GeoIP DB info")
                            .arg(Arg::with_name("database")
                                .short("d")
                                .long("database")
                                .value_name("DATABASE")
                                .default_value("/tmp/GeoLite2-City.mmdb")
                                .help("Sets GeoIP / GeoLite DB")
                                .takes_value(true))
                            .arg(Arg::with_name("dump")
                                .short("a")
                                .long("dump")
                                .help("Dump all available fields"))
                            .arg(Arg::with_name("ip")
                                .help("Sets the IP(s) to look up")
                                .value_name("IP")
                                .index(1)
                                .multiple(true))
                            .get_matches();

    let reader = maxminddb::Reader::open_readfile(cli_arguments.value_of("DATABASE").unwrap_or("/tmp/GeoLite2-City.mmdb")).unwrap();

    if let Some(ip) = cli_arguments.values_of("ip") {
        for arg in ip {
            let ip: IpAddr = FromStr::from_str(&arg).unwrap();
            let city: geoip2::City = reader.lookup(ip).unwrap();
            if cli_arguments.occurrences_of("dump") > 0 {
                println!("{:#?}", city);
            } else {
                if let Some(city) = city.city {
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
    }
}
