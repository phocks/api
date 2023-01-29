use actix_web::{ get, HttpResponse, Responder };
use serde_json::json;

#[get("/")]
pub async fn home() -> impl Responder {
  let return_data =
    json!({
        "01": "Hello World!",
        "02": "If you're seeing this message it means the API is working.",
        "03": "Use some of the other endpoints to get some different functionality.",
    });

  HttpResponse::Ok().json(return_data)
}