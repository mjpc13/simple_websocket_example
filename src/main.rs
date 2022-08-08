use std::env;
use std::process;
use simple_websocket_example::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|_err| {
        process::exit(1);
    });
    simple_websocket_example::run(config);
    
}
