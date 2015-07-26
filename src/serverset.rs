#![feature(duration)]
extern crate zookeeper;
extern crate log;

use config::*;

use std::time::Duration;
use std::collections::HashMap;
use zookeeper::{Acl, CreateMode, Watcher, WatchedEvent, ZkError, ZooKeeper};
use zookeeper::perms;


pub struct ServiceEndpoint {
  pub host: String,
  pub port: u16,
}


pub struct ServersetMember {
  pub serviceEndpoint: ServiceEndpoint,
  pub additionalEndpoints: HashMap<String, ServiceEndpoint>,
  pub status: String,
}


struct NullWatcher;
impl Watcher for NullWatcher {
  fn handle(&self, e: &WatchedEvent) {
    return
  }
}


pub struct Serverset {
  zk_client: ZooKeeper,
  pub members: HashMap<String, ServersetMember>,
}
impl Serverset {
  pub fn new(config: DiscoConfig) -> Serverset {
    match ZooKeeper::connect(format!("{}:{}/",
        config.zookeeper_host, config.zookeeper_port).as_str(),
        Duration::from_secs(config.zookeeper_timeout_secs), NullWatcher) {
      Err(reason) => panic!("Unable to connect to ZooKeeper: {}", reason),
      Ok(client) => Serverset{
        zk_client: client,
        members: HashMap::new(),
      },
    }
  }
}


const STATUS_DEAD: &'static str = "DEAD";
const STATUS_STARTING: &'static str = "STARTING";
const STATUS_ALIVE: &'static str = "ALIVE";
const STATUS_STOPPING: &'static str = "STOPPING";
const STATUS_STOPPED: &'static str = "STOPPED";
const STATUS_WARNING: &'static str = "WARNING";
const STATUS_UNKNOWN: &'static str = "UNKNOWN";
