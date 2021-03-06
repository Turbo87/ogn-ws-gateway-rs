use actix::prelude::*;
use actix_web::{error::ErrorInternalServerError, web, Responder};
use anyhow::Result;

use crate::redis;

pub async fn get(redis: web::Data<Addr<redis::RedisExecutor>>) -> impl Responder {
    let devices: Result<String> = redis
        .send(redis::ReadOGNDDB)
        .await
        .map_err(ErrorInternalServerError)?;

    let response = devices
        .map_err(ErrorInternalServerError)?
        .with_header("Access-Control-Allow-Origin", "*")
        .with_header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
        .with_header("Content-Type", "application/json");

    Ok::<_, actix_web::Error>(response)
}
