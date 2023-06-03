//  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#![allow(unused_imports)]

use {
    crate::Environment,
    dns_lookup::lookup_host,
    fastping_rs::{
        PingResult::{Idle, Receive},
        Pinger,
    },
    serde::{Deserialize, Serialize},
    serde_json::{Result as SerdeResult, Value},
    std::{error::Error, fs::File, io::BufReader, path::Path, sync::RwLock},
};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Host {
    pub group: String,
    pub host: String,
    pub label: String,
}

impl Host {
    pub fn is_up(host: &Host) -> bool {
        match lookup_host(&host.host) {
            Ok(ips) => {
                let ip_addr = ips[0];

                let (pinger, results) = match Pinger::new(Some(500_u64), None) {
                    Ok((pinger, results)) => (pinger, results),
                    Err(e) => panic!("Error creating pinger: {}", e),
                };

                pinger.add_ipaddr(&ip_addr.to_string());
                pinger.run_pinger();

                match results.recv() {
                    Ok(result) => match result {
                        Idle { addr: _ } => false,
                        Receive { addr: _, rtt: _ } => true,
                    },
                    Err(_) => false, // panic!("Worker threads disconnected before the solution was found!"),
                }
            }
            Err(_) => false, // panic!("hostname: {} DNS lookup failure", host.host),
        }
    }

    pub fn info(host: &Host) -> String {
        match lookup_host(&host.host) {
            Ok(ips) => {
                let ip_addr = ips[0];

                format!(
                    "{:?} {}",
                    ip_addr,
                    if Self::is_up(host) { "up" } else { "down" },
                )
            }
            Err(_) => format!("hostname: {} DNS lookup failure", host.host),
        }
    }

    pub fn load(env: &Environment) -> Option<Vec<Host>> {
        let dot_path = &std::path::Path::join(&env.dot_path, "hosts.json");
        let path = match &env.hosts_path {
            Some(path) => path,
            None => dot_path,
        };

        match File::open(path) {
            Ok(file) => {
                let reader = BufReader::new(file);

                let hosts_vec: Vec<Host> = serde_json::from_reader(reader).unwrap();
                Some(hosts_vec)
            }
            Err(_) => None,
        }
    }
}
