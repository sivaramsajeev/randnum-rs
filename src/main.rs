use warp::Filter;
use rand::Rng;
use serde::Serialize;

#[derive(Serialize)]
struct CountResponse {
    count: u32,
}

#[tokio::main]
async fn main() {
    let count_route = warp::path!("count")
        .map(|| {
            let mut rng = rand::thread_rng();
            let count = rng.gen_range(1..=50);
            warp::reply::json(&CountResponse { count: count })
        });

    warp::serve(count_route)
        .run(([0, 0, 0, 0], 8080))
        .await;
}

