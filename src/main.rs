use std::{convert::Infallible, net::SocketAddr};

use hyper::service::service_fn;

use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
  let worker_hosts = vec![
        "http://localhost:3000".to_string(),
        "http://localhost:3001".to_string(),
    ];

    let addr = SocketAddr::from(([127, 0, 0, 1], 1337));

    let server = Server::bind(&addr).serve(make_service_fn(move |_conn| {
      async move { Ok::<_, Infallible>(service_fn(move |req| handle(req))) }
  }));
}

async fn handle(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
  Ok(())
}
