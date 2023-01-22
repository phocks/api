#![allow(unused)]

mod get_token;
use get_token::{ get_token };

mod database;
use database::{ connect };

use actix_cors::Cors;
use actix_web::{ get, post, web, App, HttpResponse, HttpServer, Responder };
use serde_json::json;
use serde::{ Deserialize };

use dotenv::dotenv;
use std::env;

#[get("/")]
async fn root() -> impl Responder {
  let email: String = String::from("no-reply@gmail.com");
  let token: String = get_token(&email);

  let return_data =
    json!({
        "01": "Hello World!",
        "02": "If you're seeing this message it means the API is working.",
        "03": "Use some of the other endpoints to get some different functionality.",
    });

  connect();

  HttpResponse::Ok().json(return_data)
}

#[derive(Deserialize, Debug)]
struct MyQuery {
  username: String,
}

#[get("/get-jwt")]
async fn get_jwt(query: web::Query<MyQuery>) -> impl Responder {
  // let email: String = String::from("no-reply@gmail.com");
  let username: String = query.username.to_string();
  let token: String = get_token(&username);

  let return_data = json!({
    "username": username,
    "jwt": token,
    });

  HttpResponse::Ok().json(return_data)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
  HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();

  println!("Attempting to listen on http://localhost:3000");

  HttpServer::new(|| {
    App::new()
      .wrap(Cors::permissive())
      .service(root)
      .service(echo)
      .service(get_jwt)
  })
    .bind(("0.0.0.0", 3000))?
    .run().await
}