use std::error::Error;
use warp::{ws::Message, Filter, Rejection};
use std::{collections::HashMap, convert::Infallible, sync::Arc};
use tokio::sync::{mpsc, Mutex};
mod handlers;
mod ws;

pub struct Config {
  pub port: u8,
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
    if args.len() < 1 {
      return Err("Not enough arguments");
    }
    let port = args[1].parse().unwrap();

    Ok(Config {port})
  }
}

#[tokio::main]
pub async fn run(config: Config){
  
  let clients: Clients = Arc::new(Mutex::new(HashMap::new()));
  println!("Configuring websocket route");
  let ws_route = warp::path("ws")
    .and(warp::ws())
    .and(with_clients(clients.clone()))
    .and_then(handlers::ws_handler);
  
  let routes = ws_route.with(warp::cors().allow_any_origin());
  println!("Starting server");
  warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}
