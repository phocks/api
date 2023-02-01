use actix_web::{ get, HttpResponse, Responder };
use serde_json::json;
use crate::alphabet::ALPHABET as alphabet;

use nanoid::nanoid;

#[get("/")]
pub async fn home() -> impl Responder {
  let id = nanoid!(10, &alphabet); //=> "4f90d13a42"

  let return_data =
    json!({
        "01": "Hello World!",
        "02": "If you're seeing this message it means the API is working.",
        "03": "Use some of the other endpoints to get some different functionality.",
        "random_id": id,
    });

  HttpResponse::Ok().json(return_data)
}