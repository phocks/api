use actix_web::{ post, web, HttpResponse, Responder };
use serde_json::json;
use serde::Deserialize;

use crate::database;

// Reduce CPU time it takes to generate a hash
const REDUCED_COST: u32 = 8;

#[derive(Deserialize, Debug)]
struct RegisterFields {
  username: String,
  password: String,
}

#[post("/user/register")]
async fn register(req_body: web::Json<RegisterFields>) -> impl Responder {
  println!("data: {:?}", req_body);

  let username: String = req_body.username.to_string();
  let password: String = req_body.password.to_string();
  let hashed = bcrypt::hash(&password, REDUCED_COST).expect("Hashing error!");

  database::insert_blocking(&username, &hashed);

  let return_data = json!({
    "username": username,
    "hash": hashed,
    });

  HttpResponse::Ok().json(return_data)
}