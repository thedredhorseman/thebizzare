use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::net::SocketAddr;
use crate::config::Config;
use std::fs;
use std::path::Path;
use tokio::time::{sleep, Duration};

pub async fn start_proxy(config: &Config) {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let make_svc = make_service_fn(move |_conn| {
        let config = config.clone();
        async move {
            Ok::<_, hyper::Error>(service_fn(move |req| {
                let config = config.clone();
                handle_request(req, config)
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("HTTP Proxy listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn handle_request(req: Request<Body>, config: Config) -> Result<Response<Body>, hyper::Error> {
    let path = match req.uri().path_and_query() {
        Some(p) => p.path(),
        None => req.uri().path(),
    };

    let file_path = Path::new(&config.html_dir).join(&path[1..]);

    if !file_path.exists() {
        return Ok(Response::builder()
            .status(404)
            .body(Body::from("Not Found"))
            .unwrap());
    }

    let contents = fs::read_to_string(file_path).unwrap();
    let chunk_size = 1024 * 16;

    let mut body = Body::empty();
    for chunk in contents.as_bytes().chunks(chunk_size) {
        body = body.concat(Body::from(chunk.to_vec())).await.unwrap();
    }

    Ok(Response::new(body))
}
