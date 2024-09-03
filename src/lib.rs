use std::{str::FromStr, sync::Arc};

use hyper::{client::ResponseFuture, Body, Client, Request, Response, Uri};
use tokio::sync::RwLock;

pub struct LoadBalancer {
  client: Client<hyper::client::HttpConnector>,
  workers: Vec<String>,
  curr_worker: usize,
}

impl LoadBalancer {
    pub fn new(workers: Vec<String>) -> Self {
      Self {
        workers,
        curr_worker: 0,
        client: Client::new()
      }
    }

    pub fn forward_request(&mut self, req: Request<Body>) -> ResponseFuture {
      println!("worker {}", self.curr_worker);
      let mut worker_uri = self.get_worker().to_owned();
      if let Some(path_and_query) = req.uri().path_and_query() {
          worker_uri.push_str(path_and_query.as_str());
      }
      let new_uri = Uri::from_str(&worker_uri).unwrap();
      let headers = req.headers().clone();
      let mut new_req = Request::builder()
          .method(req.method())
          .uri(new_uri)
          .body(req.into_body())
          .expect("request builder");

      for (key, value) in headers.iter() {
          new_req.headers_mut().insert(key, value.clone());
      }

      self.client.request(new_req)
  }

  // Round-robin strategy
  fn get_worker(&mut self) -> &str {
    let worker = self.workers.get(self.curr_worker).unwrap();
    self.curr_worker = (self.curr_worker + 1) % self.workers.len();
    worker
  }
}


pub async fn handle_req(req: Request<Body>, ld_bal: Arc<RwLock<LoadBalancer>>) -> Result<Response<Body>, hyper::Error> {
  ld_bal.write().await.forward_request(req).await
}