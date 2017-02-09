extern crate iron;
extern crate logger;
extern crate router;
extern crate hyper;
extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate log;
extern crate env_logger;

use std::str::FromStr;
use std::io::Read;
use iron::prelude::*;
use router::Router;
use hyper::client::{Client, Body};
use logger::Logger;

struct Forwarder {
    client: Client,
    protocol: String,
    domain: String,
}

impl Forwarder {
    fn forward(&self, req: &mut Request) {
        let mut url_base = String::new();
        url_base.push_str(&self.protocol);
        url_base.push_str("://");
        url_base.push_str(&self.domain);
        url_base.push_str("/");
        url_base.push_str(&req.url.path().join("/"));

        if let Some(ref q) = req.url.query() {
            url_base.push_str("?");
            url_base.push_str(q);
        }
        
        let mut body = Vec::new();
        req.body.read_to_end(&mut body);

        self.client.request(req.method.clone(), &url_base)
            .headers(req.headers.clone())
            .body(Body::BufBody(body.as_slice(), body.len()))
            .send()
            .unwrap();
    }

    fn new(protocol: &str, domain: &str) -> Forwarder {
        Forwarder { client: Client::new(),
                    protocol: String::from_str(protocol).unwrap(),
                    domain: String::from_str(domain).unwrap()}
    }
}


fn forward(req: &mut Request) -> IronResult<Response> {
   let forwarder = Forwarder::new("http", "localhost:6668");
   forwarder.forward(req);

   Ok(Response::with((iron::status::Ok, "Hello world"))) 
}

#[derive(Serialize)]
struct Stats {
    requests_forwarded: u64,
    target_requests_per_second: f64,
    average_requests_per_second: f64,
    max_requests_per_second: f64,
    buffer_size_in_bytes: usize,
}

fn stat_handler(req: &mut Request) -> IronResult<Response> {
    let stats = Stats {
        requests_forwarded: 345242,
        target_requests_per_second: 250.,
        average_requests_per_second: 261.,
        max_requests_per_second: 342.,
        buffer_size_in_bytes: 5098231,
    };

    Ok(Response::with((iron::status::Ok, 
        serde_json::to_string(&stats).unwrap())))
}

fn rate_handler(req: &mut Request) -> IronResult<Response> {
   Ok(Response::with((iron::status::Ok, "Hello admin"))) 
}

fn buffer_handler(req: &mut Request) -> IronResult<Response> {
   Ok(Response::with((iron::status::Ok, "Hello admin"))) 
}

fn get_target(req: &mut Request) -> IronResult<Response> {
   Ok(Response::with((iron::status::Ok, "Hello admin"))) 
}

fn set_target(req: &mut Request) -> IronResult<Response> {
   Ok(Response::with((iron::status::Ok, "Hello admin"))) 
}

fn main() {
    env_logger::init().unwrap();

    let (logger_before, logger_after) = Logger::new(None);
    let mut forward_chain = Chain::new(forward);
    forward_chain.link_before(logger_before);
    forward_chain.link_after(logger_after);
    let forward_server = Iron::new(forward_chain).http("localhost:6666");

    let mut router = Router::new();
    router.get("/stat", stat_handler, "stat");
    router.put("/rate", rate_handler, "rate");
    router.delete("/buffer", buffer_handler, "buffer");
    router.get("/target", get_target, "get_target");
    router.put("/target", set_target, "set_target");

    let admin_server = Iron::new(router).http("localhost:6667");

    debug!("debug logging on");
    println!("Ready");
    forward_server.unwrap();
}
