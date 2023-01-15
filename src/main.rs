#![allow(unused)]

mod get_token;
use get_token::{ get_token };

use actix_web::{ get, post, web, App, HttpResponse, HttpServer, Responder };
use serde_json::json;

#[get("/")]
async fn hello() -> impl Responder {
  // let token = get_token();
  let token: String = get_token();

  let data = json!({
        "jwt": token,
    });

  HttpResponse::Ok().json(data)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
  HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
  HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new().service(hello).service(echo).route("/hey", web::get().to(manual_hello))
  })
    .bind(("0.0.0.0", 3000))?
    .run().await
}