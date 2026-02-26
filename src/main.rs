use std::convert::Infallible;
use std::env;
use warp::{http::Method, Filter};

#[tokio::main]
async fn main() {
    // Azure sets PORT automatically
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "3030".to_string())
        .parse()
        .expect("PORT must be a number");

    // CORS (allow browser calls from store-front)
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(&[Method::GET])
        .allow_headers(vec!["content-type"]);

    // GET /
    let root = warp::path::end().map(|| "product-service is running");

    // GET /health
    let health = warp::path("health").and(warp::path::end()).map(|| {
        warp::reply::json(&serde_json::json!({
            "status": "ok",
            "service": "product-service"
        }))
    });

    // GET /products
    let products = warp::path("products").and(warp::path::end()).map(|| {
        warp::reply::json(&vec![
            serde_json::json!({ "id": 1, "name": "Dog Food", "price": 19.99 }),
            serde_json::json!({ "id": 2, "name": "Cat Food", "price": 34.99 }),
            serde_json::json!({ "id": 3, "name": "Bird Seeds", "price": 10.99 }),
        ])
    });

    let routes = root.or(health).or(products).with(cors);

    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}
