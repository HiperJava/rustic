use actix_web::{delete, get, post, put, web, HttpResponse};
use actix_web::client::SendRequestError::Http;
use serde_json::json;
use crate::config::error_handler::CustomError;
use crate::sensor::model::Sensor;
use crate::sensor::Sensors;

#[get("/sensors")]// -> <HttpResponse, CustomError>
async fn find_all() -> Result<HttpResponse, CustomError> {
    let sensors = web::block(|| Sensors::find_all()).await.unwrap();
    Ok(HttpResponse::Ok().json(sensors))
}

#[get("/sensors/{id}")]// -> <HttpResponse, CustomError>
async fn find(id: web::Path<String>) -> Result<HttpResponse, CustomError> {
    let sensor = Sensors::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(sensor))
}

#[post("/sensors")]// -> <HttpResponse, CustomError>
async fn create(sensor: web::Json<Sensor>) -> Result<HttpResponse, CustomError> {
    let sensor = Sensors::create(sensor.into_inner())?;
    Ok(HttpResponse::Ok().json(sensor))
}

#[put("/sensors/{id}")]// -> <HttpResponse, CustomError>
async fn update(id: web::Path<String>, sensor: web::Json<Sensor>) -> Result<HttpResponse, CustomError> {
    let sensor = Sensors::update(id.into_inner(), sensor.into_inner())?;
    Ok(HttpResponse::Ok().json(sensor))
}

#[delete("/sensors/{id}")]// -> <HttpResponse, CustomError>
async fn delete(id: web::Path<String>) -> Result<HttpResponse, CustomError> {
    let delete_sensor = Sensors::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({"deleted":delete_sensor})))
}
pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(find);
    config.service(create);
    config.service(update);
    config.service(delete);
}
