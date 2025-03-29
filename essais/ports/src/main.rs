use warp::Filter;
use serde::{Deserialize, Serialize};
use warp::http::Method;
use warp::cors::Cors;

#[derive(Deserialize)]
struct Request {
    name: String,
}

#[derive(Serialize)]
struct Response {
    message: String,
}

#[tokio::main]
async fn main() {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(&[Method::POST])
        .allow_headers(vec!["Content-Type"]);

    let hello = warp::path("hello")
        .and(warp::post())
        .and(warp::body::json())
        .map(|req: Request| {
            let response = Response {
                message: format!("Hello, {}!", req.name),
            };
            warp::reply::json(&response)
        });

    let routes = hello.with(cors);

    warp::serve(routes).run(([0, 0, 0, 0], 4174)).await;
}
