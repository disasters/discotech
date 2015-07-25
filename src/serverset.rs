extern crate zookeeper;

use config::*;

use std::collections::HashMap;


pub struct ServiceEndpoint {
  host: String,
  port: u16,
}

pub struct ServersetMember {
  serviceEndpoint: ServiceEndpoint,
  additionalEndpoints: HashMap<String, ServiceEndpoint>,
  status: String,
}

const STATUS_DEAD: &'static str = "DEAD";
const STATUS_STARTING: &'static str = "STARTING";
const STATUS_ALIVE: &'static str = "ALIVE";
const STATUS_STOPPING: &'static str = "STOPPING";
const STATUS_STOPPED: &'static str = "STOPPED";
const STATUS_WARNING: &'static str = "WARNING";
const STATUS_UNKNOWN: &'static str = "UNKNOWN";


fn monitor_serverset(config: DiscoConfig) {
  println!("test");
}
