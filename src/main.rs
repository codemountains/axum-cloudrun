use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use serde::Serialize;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/mountains", get(find_sacred_mountains));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap_or_else(|_| panic!("Server cannot launch."));
}

async fn find_sacred_mountains() -> (StatusCode, Json<JsonResponse>) {
    let response: JsonResponse = Default::default();
    (StatusCode::OK, Json(response))
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonResponse {
    mountains: Vec<SacredMountain>,
    total: usize,
}

impl Default for JsonResponse {
    fn default() -> Self {
        let mountains = vec![
            SacredMountain::new(1, "恐山".to_string()),
            SacredMountain::new(2, "比叡山".to_string()),
            SacredMountain::new(3, "高野山".to_string()),
        ];
        let total = mountains.len();

        Self { mountains, total }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SacredMountain {
    id: i32,
    name: String,
}

impl SacredMountain {
    fn new(id: i32, name: String) -> Self {
        Self { id, name }
    }
}
