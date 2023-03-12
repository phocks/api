use actix_web::{ get, HttpResponse, Responder };
use serde_json::json;
use names::Generator;

use nanoid::nanoid;

#[get("/")]
pub async fn home() -> impl Responder {
  let id = nanoid!();
  let mut generator = Generator::default();

  let return_data =
    json!({
        "01": "Hello World!",
        "02": "If you're seeing this message it means the API is working.",
        "03": "Use some of the other endpoints to get some different functionality.",
        "04": "Below are just a few test strings",
        "05": "So why is it only executing the first one?",
        "random_id": id,
        "random_name": generator.next().unwrap(),
    });

  HttpResponse::Ok().json(return_data)
}