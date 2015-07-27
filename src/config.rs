extern crate rustc_serialize;

use std::io::prelude::*;
use std::io;
use std::fs::File;

use rustc_serialize::json;


#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct DiscoConfig {
  pub zookeeper_host: String,
  pub zookeeper_port: u16,
  pub zookeeper_poll_secs: u64,
  pub zookeeper_timeout_secs: u64,
  pub serverset_znode: String,
}


pub fn read_config(config_file_loc: String) -> io::Result<DiscoConfig> {
  let mut config_file = try!(File::open(config_file_loc));
  let mut config_file_contents = String::new();
  try!(config_file.read_to_string(&mut config_file_contents));
  let config: DiscoConfig = json::decode(&config_file_contents).unwrap();
  Ok(config)
}
