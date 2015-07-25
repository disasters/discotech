extern crate discotech;

use discotech::config::*;

use std::env;


fn initialize(config: DiscoConfig) {
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
