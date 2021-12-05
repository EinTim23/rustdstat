use crate::{ws, Client, Clientlist, Result};
use warp::{http::StatusCode, ws::Message, Reply};

pub async fn reset() {
    unsafe {
        super::CLIENTS
        .read()
        .await
        .iter()
        .for_each(|(_, client)| {
            if let Some(sender) = &client.sender {
                let _ = sender.send(Ok(Message::text(super::requests.to_string())));
            }
        });
        super::requests = 0;
    }
}
pub async fn ws_handler(ws: warp::ws::Ws, clients: Clientlist) -> Result<impl Reply> {
    unsafe{
        super::id += 1;
        clients.write().await.insert(super::id.to_string(), Client{topics: vec![String::from("dstatlistener")],sender: None,});
        Ok(ws.on_upgrade(move |socket| ws::client_connection(socket, super::id.to_string(), clients,Client {topics: vec![String::from("dstatlistener")],sender: None,})))
    }
}

pub async fn index_handler() -> Result<impl Reply> {
    Ok(warp::reply::html(super::HTMLF.clone()).into_response())
}
pub async fn dstat_handler() -> Result<impl Reply> {
    unsafe {
        super::requests += 1;
    }
    Ok(StatusCode::OK)
}