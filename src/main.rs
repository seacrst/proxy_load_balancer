use std::{convert::Infallible, net::SocketAddr, sync::Arc};

use hyper::{service::{make_service_fn, service_fn}, Server};

use proxy_load_balancer::{handle_req, LoadBalancer};
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
  let workers = vec![
    "http://localhost:3001".to_string(),
    "http://localhost:3002".to_string(),
    "http://localhost:3003".to_string(),
  ];

  let balancer = Arc::new(RwLock::new(LoadBalancer::new(workers)));
  let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 1337));
  let server = Server::bind(&addr)
    .serve(
      make_service_fn(move |_conn| {
        let ld_bal = balancer.clone();
        async move { Ok::<_, Infallible>(service_fn(move |req| handle_req(req, ld_bal.clone()))) }
      })
    );

  if let Err(e) = server.await {
    eprintln!("error: {}", e);
  }
}