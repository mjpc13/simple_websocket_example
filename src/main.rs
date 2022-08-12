use std::env;
use std::process;
use simple_websocket_example::Config;
use clap::Parser;


#[derive(Parser,Default,Debug)]
#[clap(author="Mário Cristóvão", version, about="A Very simple example to create a WebSocket Server")]
struct Arguments {
  pub port: String,
  pub ip_address: String,
  pub ws_path: String,
}

impl Arguments {
    pub fn as_array(&self) -> [String; 3] {
        [self.port.clone(), self.ip_address.clone(), self.ws_path.clone()]
    }
}

fn main() {
    // let args: Vec<String> = env::args().collect();
    let args = Arguments::parse();
    let config = Config::new(&args.as_array()).unwrap_or_else(|_err| {
        process::exit(1);
    });
    simple_websocket_example::run(config);
    
}
