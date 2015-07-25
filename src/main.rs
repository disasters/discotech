extern crate rustc_serialize;

use rustc_serialize::json;
use std::env;

use std::io::prelude::*;
use std::io;
use std::fs::File;


#[derive(RustcDecodable, RustcEncodable)]
pub struct Config {
  zookeeper_host: String,
  zookeeper_port: u16,
  serverset: String,
}


fn read_config(config_file_loc: String) -> io::Result<Config> {
  let mut config_file = try!(File::open(config_file_loc));
  let mut config_file_contents = String::new();
  try!(config_file.read_to_string(&mut config_file_contents));
  let config: Config = json::decode(&config_file_contents).unwrap();
  Ok(config)
}

fn initialize(config: Config) {
  println!("Config read");
}


fn main() {
  let config_file_loc = match env::var("DISCO_CONF") {
    Err(_) => panic!("Please set the DISCO_CONF environment variable"),
    Ok(location) => location,
  };
  match read_config(config_file_loc) {
    Err(_) => panic!("Unable to read configuration; bailing"),
    Ok(config) => initialize(config),
  }
}
