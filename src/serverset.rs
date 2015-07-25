#![feature(duration)]
extern crate zookeeper;
extern crate log;

use config::*;

use std::time::Duration;
use std::collections::HashMap;
use zookeeper::{Acl, CreateMode, Watcher, WatchedEvent, ZooKeeper};
use zookeeper::perms;


pub struct ServiceEndpoint {
  host: String,
  port: u16,
}

pub struct ServersetMember {
  serviceEndpoint: ServiceEndpoint,
  additionalEndpoints: HashMap<String, ServiceEndpoint>,
  status: String,
}

struct Serverset {
  members: HashMap<String, ServersetMember>,
}

impl Watcher for Serverset {
  fn handle(&self, e: &WatchedEvent) {
    info!("{:?}", e)
  }
}

const STATUS_DEAD: &'static str = "DEAD";
const STATUS_STARTING: &'static str = "STARTING";
const STATUS_ALIVE: &'static str = "ALIVE";
const STATUS_STOPPING: &'static str = "STOPPING";
const STATUS_STOPPED: &'static str = "STOPPED";
const STATUS_WARNING: &'static str = "WARNING";
const STATUS_UNKNOWN: &'static str = "UNKNOWN";


fn get_serverset(config: DiscoConfig) {
  let serverset = Serverset {
    members: HashMap::new(),
  };
  let zk = match ZooKeeper::connect(format!("{}:{}/", config.zookeeper_host,
      config.zookeeper_port).as_str(), Duration::from_secs(config.zookeeper_timeout_secs),
      serverset) {
    Err(reason) => panic!("Unable to connect to ZooKeeper: {}", reason),
    Ok(client) => client,
  };
}
