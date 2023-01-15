#![allow(unused)]

mod get_token;
use get_token::{ get_token };

use actix_web::{ get, post, web, App, HttpResponse, HttpServer, Responder };
use serde_json::json;

use dotenv::dotenv;
use std::env;


#[get("/")]
async fn hello() -> impl Responder {
  let email = String::from("no-reply@gmail.com");
  let token: String = get_token(&email);

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
  dotenv().ok();

  // for (key, value) in env::vars() {
  //     println!("{}: {}", key, value);
  // }

  // println!("{}", dotenv!("SECRET"));
  
  HttpServer::new(|| {
    App::new().service(hello).service(echo).route("/hey", web::get().to(manual_hello))
  })
    .bind(("0.0.0.0", 3000))?
    .run().await
}