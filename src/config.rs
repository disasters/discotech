extern crate rustc_serialize;

use std::io::prelude::*;
use std::io;
use std::fs::File;

use rustc_serialize::json;


#[derive(RustcDecodable, RustcEncodable)]
pub struct DiscoConfig {
  zookeeper_host: String,
  zookeeper_port: u16,
  serverset: String,
}


pub fn read_config(config_file_loc: String) -> io::Result<DiscoConfig> {
  let mut config_file = try!(File::open(config_file_loc));
  let mut config_file_contents = String::new();
  try!(config_file.read_to_string(&mut config_file_contents));
  let config: DiscoConfig = json::decode(&config_file_contents).unwrap();
  Ok(config)
}
