extern crate discotech;
#[macro_use]
extern crate log;
extern crate log4rs;

use discotech::{Serverset, DiscoConfig, read_config};

use std::env;

fn initialize_logging() {
  let root = log4rs::config::Root::builder(log::LogLevelFilter::Debug)
    .appender("stderr".to_string());
  let console = Box::new(log4rs::appender::ConsoleAppender::builder().build());
  let config = log4rs::config::Config::builder(root.build())
    .appender(log4rs::config::Appender::builder("stderr".to_string(), console).build());
  log4rs::init_config(config.build().unwrap()).unwrap();
}

fn initialize(config: DiscoConfig) {
  initialize_logging();
  let serverset = *Box::new(Serverset::new(config));
  debug!("THINGS");
  serverset.update_members();
  for member in serverset.members.read().unwrap().iter() {
    debug!("Member: {:?}", member);
  }
}

#[test]
fn integration() {
  let config_file_loc = match env::var("DISCO_CONF") {
    Err(_) => panic!("Please set the DISCO_CONF environment variable"),
    Ok(location) => location,
  };
  match read_config(config_file_loc) {
    Err(reason) => panic!("Unable to read configuration; bailing: {}", reason),
    Ok(config) => initialize(config),
  }
}
