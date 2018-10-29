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

    let droplet = match Droplet::create(
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
        Ok(d) => d,
    };

    println!("{:?}", droplet);

    let addrs = &droplet.networks().v4[0];
    println!(
        "id: {}\nname: {}\nip: {:?}",
        droplet.id(),
        droplet.name(),
        addrs
    );

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
