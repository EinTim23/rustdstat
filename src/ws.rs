use crate::{Client, Clientlist};
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;
use warp::ws::{WebSocket};

pub async fn client_connection(ws: WebSocket, id: String, clients: Clientlist, mut client: Client) {
    let (client_ws_sender, mut client_ws_rcv) = ws.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel();

    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
        if let Err(_e) = result {
            eprintln!("unbekannter fehler :KEKW:");
        }
    }));
    client.sender = Some(client_sender);
    clients.write().await.insert(id.clone(), client.clone());

    while let Some(result) = client_ws_rcv.next().await {
        let _msg = match result {
            Ok(_msg) => _msg,
            Err(_e) => {
                eprintln!("unbekannter fehler :KEKW:");
                break;
            }
        };
    }

    clients.write().await.remove(&id);
    unsafe{
        super::id -= 1;
    }
}
