use std::env;
use warp::Filter;

#[tokio::main]
async fn main() {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET"]);

    let products = warp::path("products")
        .map(|| {
            warp::reply::json(&vec![
                serde_json::json!({ "id": 1, "name": "Dog Food", "price": 19.99 }),
                serde_json::json!({ "id": 2, "name": "Cat Food", "price": 34.99 }),
                serde_json::json!({ "id": 3, "name": "Bird Seeds", "price": 10.99 }),
            ])
        });

    let health = warp::path("health")
        .map(|| "ok");

    // Combine routes + apply CORS to everything
    let routes = products
        .or(health)
        .with(cors);

    // Azure sets PORT automatically
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "3030".to_string())
        .parse()
        .expect("PORT must be a number");

    warp::serve(routes)
        .run(([0, 0, 0, 0], port))
        .await;
}