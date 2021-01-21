#![allow(dead_code)]
use chrono::{DateTime, Utc};
use serde::Deserialize;
use actix_web::{HttpResponse, Responder, post, web::Json};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FunctionHandler {
    data: Data,
    metadata: Metadata,
}

#[derive(Debug, Deserialize)]
pub struct Data {
    timer: Timer,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Timer {
    schedule: Schedule,
    schedule_status: Option<serde_json::Value>,
    is_past_due: bool,
}

#[derive(Debug, Deserialize)]
pub struct Schedule {
    #[serde(rename = "AdjustForDST")]
    adjust_for_dst: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Metadata {
    #[serde(rename = "sys")]
    sys: Sys,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Sys {
    method_name: String,
    utc_now: DateTime<Utc>,
    rand_guid: String,
}

#[post("/MyTimer")]
pub(crate) async fn my_timer(trigger: Json<FunctionHandler>) -> impl Responder {
    println!("Timer triggered at {:?}", trigger);
    HttpResponse::Ok()
}