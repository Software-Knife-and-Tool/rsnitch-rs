//  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#![allow(unused_imports)]

use {
    crate::Environment,
    dns_lookup::lookup_host,
    fastping_rs::{
        NewPingerResult,
        PingResult::{self, Idle, Receive},
        Pinger,
    },
    serde::{Deserialize, Serialize},
    serde_json::{Result as SerdeResult, Value},
    std::{
        error::Error,
        fs::File,
        io::BufReader,
        path::Path,
        sync::{mpsc::Receiver, RwLock},
    },
};

pub struct Poll {
    pinger: Pinger,
    results: Receiver<PingResult>,
}

impl Poll {
    pub fn new(_env: &Environment) -> Self {
        let (pinger, results) = match Pinger::new(Some(1000_u64), None) {
            Ok((pinger, results)) => (pinger, results),
            Err(e) => panic!("Error creating pinger: {}", e),
        };

        Self { pinger, results }
    }

    pub fn poll(&self, host: &Host) -> bool {
        match lookup_host(&host.host) {
            Ok(ips) => {
                let ip_addr = ips[0];
                self.pinger.add_ipaddr(&ip_addr.to_string());
                self.pinger.run_pinger();

                let state = match self.results.recv() {
                    Ok(result) => match result {
                        Idle { addr: _ } => false,
                        Receive { addr: _, rtt: _ } => true,
                    },
                    Err(_) => false, // panic!("Worker threads disconnected before the solution was found!"),
                };

                self.pinger.remove_ipaddr(&ip_addr.to_string());

                state
            }
            Err(_) => false, // panic!("hostname: {} DNS lookup failure", host.host),
        }
    }

    pub fn poll_all(&self, hosts: &[Host]) -> Vec<bool> {
        let mut states = vec![false; hosts.len()];

        let ipaddrs: Vec<String> = hosts
            .iter()
            .map(|host| match lookup_host(&host.host) {
                Ok(ips) => {
                    let ip_addr = ips[0].to_string();
                    self.pinger.add_ipaddr(&ip_addr);
                    ip_addr
                }
                Err(_) => "".to_string(),
            })
            .collect();

        self.pinger.run_pinger();

        for _ in 0..hosts.len() {
            let (addr, state) = match self.results.recv() {
                Ok(result) => match result {
                    Idle { addr } => (addr, false),
                    Receive { addr, rtt: _ } => (addr, true),
                },
                Err(_) => panic!("Worker threads disconnected before the solution was found!"),
            };

            states[ipaddrs
                .iter()
                .position(|ipaddr| &addr.to_string() == ipaddr)
                .unwrap()] = state;
        }

        states
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Host {
    pub group: String,
    pub host: String,
    pub label: String,
}

impl Host {
    pub fn info(poll: &Poll, host: &Host) -> String {
        match lookup_host(&host.host) {
            Ok(ips) => {
                let ip_addr = ips[0];

                format!(
                    "{:?} {}",
                    ip_addr,
                    if poll.poll(host) { "up" } else { "down" },
                )
            }
            Err(_) => format!("hostname: {} DNS lookup failure", host.host),
        }
    }

    pub fn load(env: &Environment) -> Option<Vec<Host>> {
        let dot_path = &std::path::Path::join(&env.config_path, "hosts.json");
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
