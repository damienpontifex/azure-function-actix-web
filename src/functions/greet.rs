use serde::Deserialize;
use actix_web::{get, web::Query};

#[derive(Deserialize)]
pub(crate) struct Info {
    name: Option<String>,
}

#[get("/api/HttpTrigger")]
pub(crate) async fn greet(info: Query<Info>) -> String {
    match info.name {
        Some(ref name) => format!("hello {}!", name),
        None => "hello world!".to_string(),
    }
}