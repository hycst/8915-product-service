use std::env;
use warp::{http::Method, Filter};

#[tokio::main]
async fn main() {
    // CORS for browser calls (store-front -> API)
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec![Method::GET])
        .allow_headers(vec!["content-type"]);

    // GET /
    let root = warp::path::end().map(|| "product-service is running. Try /health or /products");

    // GET /health
    let health = warp::path("health").and(warp::path::end()).map(|| "ok");

    // GET /products
    let products = warp::path("products").and(warp::path::end()).map(|| {
        warp::reply::json(&vec![
            serde_json::json!({ "id": 1, "name": "Dog Food", "price": 19.99 }),
            serde_json::json!({ "id": 2, "name": "Cat Food", "price": 34.99 }),
            serde_json::json!({ "id": 3, "name": "Bird Seeds", "price": 10.99 }),
        ])
    });

    // Combine routes + apply CORS + request logging
    let routes = root
        .or(health)
        .or(products)
        .with(cors)
        .with(warp::log("product-service"));

    // Azure sets PORT automatically
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "3030".to_string())
        .parse()
        .expect("PORT must be a number");

    println!("Listening on 0.0.0.0:{port}");

    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}
