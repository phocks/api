#![allow(unused)]

mod get_token;
use get_token::{ get_token };

use actix_web::{ get, post, web, App, HttpResponse, HttpServer, Responder };
use serde_json::json;

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

  HttpResponse::Ok().json(return_data)
}

#[get("/get-jwt")]
async fn get_jwt() -> impl Responder {
  let email: String = String::from("no-reply@gmail.com");
  let token: String = get_token(&email);

  let return_data = json!({
    "email" : email,
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

  HttpServer::new(|| {
    App::new().service(root).service(echo).service(get_jwt)
  })
    .bind(("0.0.0.0", 3000))?
    .run().await
}