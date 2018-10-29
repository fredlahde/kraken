extern crate digitalocean;

use digitalocean::prelude::*;
use std::env;
use std::net::Ipv4Addr;
use std::process;

const API_KEY_VAR: &'static str = "API_KEY";

fn main() {
    let api_key = match env::var("API_KEY") {
        Ok(k) => k,
        Err(e) => {
            eprintln!("Could not find Key in {}: {}", API_KEY_VAR, e);
            process::exit(1);
        }
    };

    let client = DigitalOcean::new(api_key).unwrap();

    match Droplet::create(
        "foo-1",
        "ams3",
        "s-1vcpu-1gb",
        "centos-7-x64",
        vec!["61:30:b5:e0:f0:35:aa:a2:9a:9e:d4:9a:23:a7:e3:e4"],
    )
    .execute(&client)
    {
        Err(e) => {
            eprintln!("{:?}", e);
            process::exit(1);
        }
        _ => {}
    }

    let droplets = match Droplet::list().execute(&client) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("{:?}", e);
            process::exit(1);
        }
    };

    let addrs: Vec<Ipv4Addr> = droplets
        .iter()
        .filter(|d| d.name().contains("foo"))
        .flat_map(|d| d.networks().v4.iter().map(|n| n.ip_address))
        .collect();

    droplets
        .iter()
        .filter(|d| d.name().contains("foo"))
        .map(|d| {
            (
                d.id(),
                d.name(),
                d.networks().v4.iter().map(|n| n.ip_address),
            )
        })
        .for_each(|d| println!("id: {}\nname: {}\nip: {:?}", d.0, d.1, addrs));

    //delete_test_droplets(&client);
}

fn delete_test_droplets(client: &DigitalOcean) {
    let droplets = match Droplet::list().execute(&client) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("{:?}", e);
            process::exit(1);
        }
    };
    droplets
        .iter()
        .filter(|d| d.name().contains("foo"))
        .for_each(|d| match Droplet::delete(*d.id()).execute(&client) {
            Err(e) => {
                eprintln!("{:?}", e);
                process::exit(1);
            }
            _ => {}
        });
}
