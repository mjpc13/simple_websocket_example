use warp::{Reply, Rejection};
use crate::{ws, Clients};

pub async fn ws_handler(ws: warp::ws::Ws, clients: Clients) -> Result<impl Reply, Rejection > {
    println!("ws_handler");
    
    Ok(ws.on_upgrade(move |socket| ws::client_connection(socket, clients)))
}
