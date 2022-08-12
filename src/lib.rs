// use std::error::Error;
use warp::{ws::Message, Filter, Rejection};
use std::{collections::HashMap, convert::Infallible, sync::Arc};
use tokio::sync::{mpsc, Mutex};
use ipaddress::IPAddress;
mod handlers;
mod ws;

pub struct Config {
  pub port: u16,
  pub ip: IPAddress,
  pub ws_path: String,
  //Insert necessary arguments
}

#[derive(Debug, Clone)]
pub struct Client {
    pub client_id: String,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

// Create an alias for a group of clients
type Clients = Arc<Mutex<HashMap<String, Client>>>;

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
    if args.is_empty(){
      let port: u16 = 8000;
      let ip = IPAddress::parse("192.168.10.100/24").unwrap();
      let ws_path = String::from("ws");
      return Ok(Config{port, ip, ws_path});
      //return Err("Not enough arguments");
    }
    //Deal with unwrap
    let port = args[1].parse().unwrap(); 
    let ip = IPAddress::parse(args[2].to_string()).unwrap();
    let ws_path = args[3].to_string();
    
    Ok(Config {port, ip, ws_path})
  }
}

#[tokio::main]
pub async fn run(config: Config){
  
  let clients: Clients = Arc::new(Mutex::new(HashMap::new()));
  println!("Configuring websocket route");
  let ws_route = warp::path(config.ws_path)
    .and(warp::ws())
    .and(with_clients(clients.clone()))
    .and_then(handlers::ws_handler);
  
  let routes = ws_route.with(warp::cors().allow_any_origin());
  println!("Starting server");
  warp::serve(routes).run(([192,168,0,205], config.port)).await;
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}
