use std::{thread,time::{Duration, Instant}};
use futures::executor::block_on;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use warp::{ws::Message, Filter, Rejection};
use lazy_static::lazy_static;

mod handler;

type Result<T> = std::result::Result<T, Rejection>;
type Clientlist = Arc<RwLock<HashMap<String, Client>>>;

#[allow(non_upper_case_globals)]
pub static mut requests: i32 = 0;

#[allow(non_upper_case_globals)]
pub static mut id: i32 = 0;

lazy_static! {
    pub static ref HTMLF: String = String::from(std::fs::read_to_string("./index.html").expect("Unable to read file"));
    pub static ref CLIENTS: Clientlist = Arc::new(RwLock::new(HashMap::new()));
}
#[derive(Debug, Clone)]
pub struct Client {
    pub topics: Vec<String>,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

#[tokio::main]
async fn main() {
    let argv: Vec<_> = std::env::args().collect();
    let argc: usize = argv.len();
    if argc == 1 {
        println!("EinTim DSTAT v0.1.0");
        println!("Usage: program <port>");
        std::process::exit(1);
    }
    let _scheduler = thread::spawn(|| {
        let wait_time = Duration::from_millis(1000);
        loop {
            let start = Instant::now();
            let _thread_a = thread::spawn(move || block_on(handler::reset()));
            let runtime = start.elapsed();
            if let Some(remaining) = wait_time.checked_sub(runtime) {
                thread::sleep(remaining);
            }
        }
    });
    
    let index_route = warp::path::end().and_then(handler::index_handler);
    let dstat_route = warp::path!("dstat").and_then(handler::dstat_handler);
    let getc_route = warp::path!("getc").and_then(handler::getc_handler);
    let routes = index_route
        .or(dstat_route)
        .or(getc_route)
        .with(warp::cors().allow_any_origin());
    println!("EinTim Layer 7 DSTAT coded by Da Mivolis#1337 Port: {}", argv[1].parse::<i32>().unwrap());
    let addr: String = "0.0.0.0:".to_string() + &argv[1];
    let server: std::net::SocketAddr = addr
        .parse()
        .expect("Unable to parse socket address");
    warp::serve(routes).run(server).await;
}
