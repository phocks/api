use actix_web::{ get, post, web, App, HttpResponse, HttpServer, Responder };
use serde_json::json;
use serde::Deserialize;

use crate::tokens;

#[derive(Deserialize, Debug)]
struct MyQuery {
  username: String,
}

#[get("/get-jwt")]
async fn get_jwt(query: web::Query<MyQuery>) -> impl Responder {
  // let email: String = String::from("no-reply@gmail.com");
  let username: String = query.username.to_string();
  let token: String = tokens::get_token_for_username(&username);

  let return_data = json!({
    "username": username,
    "jwt": token,
    });

  HttpResponse::Ok().json(return_data)
}