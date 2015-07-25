extern crate rustc_serialize;

use rustc_serialize::json;
use std::env;

use std::io::prelude::*;
use std::fs::File;


#[derive(RustcDecodable, RustcEncodable)]
pub struct Config {
  zookeeper_host: String,
  zookeeper_port: u8,
  serverset: String,
}


fn read_config(config_file_loc: String) -> Config {
  let mut config_file = try!(File::open(config_file_loc));
  let mut config_file_contents = String::new();
  try!(config_file.read_to_string(&mut config_file_contents));
  let config: Config = json::decode(&config_file_contents).unwrap();
  return config;
}


fn main() {
  let config_file_loc = env::var("DISCO_CONF");
  let config = read_config(config_file_loc);

  println!("welcome to the discotech");
}
