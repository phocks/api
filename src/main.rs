#![allow(unused)]

mod entities;

mod get_token;
use get_token::{ get_token };

mod database;
use database::{ connect, insert_blocking };

use actix_cors::Cors;
use actix_web::{ get, post, web, App, HttpResponse, HttpServer, Responder };
use serde_json::json;
use serde::{ Deserialize };

use dotenv::dotenv;
use std::env;

mod routes;
use routes::home;


use bcrypt::{ DEFAULT_COST, hash, verify };
const REDUCED_COST: u32 = 8;

// #[get("/")]
// async fn root() -> impl Responder {
//   let email: String = String::from("no-reply@gmail.com");
//   let token: String = get_token(&email);

//   let return_data =
//     json!({
//         "01": "Hello World!",
//         "02": "If you're seeing this message it means the API is working.",
//         "03": "Use some of the other endpoints to get some different functionality.",
//     });

//   connect();

//   HttpResponse::Ok().json(return_data)
// }

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

#[derive(Deserialize, Debug)]
struct LoginData {
  // JSON fields go here
  username: String,
  password: String,
}

#[post("/user/register")]
async fn user_register(req_body: web::Json<LoginData>) -> impl Responder {
  println!("data: {:?}", req_body);
  // let username: String = query.username.to_string();
  // let token: String = get_token(&username);

  let username: String = req_body.username.to_string();
  let password: String = req_body.password.to_string();

  let hashed = hash(&password, REDUCED_COST).expect("Hashing error!");

  // let valid = match verify(&password, &hashed) {
  //   Ok(v) => v,
  //   Err(e) => panic!("Error verifying password: {}", e),
  // };
  // println!("Is password valid? {}", valid);

  println!("Hashed password: {}", hashed);

  insert_blocking(&username, &hashed);

  let return_data = json!({
    "username": username,
    "hash": hashed,
    });

  HttpResponse::Ok().json(return_data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();

  println!("Attempting to listen on http://localhost:3000");

  HttpServer::new(|| {
    App::new()
      .wrap(Cors::permissive())
      .service(home)
      .service(echo)
      .service(get_jwt)
      .service(user_register)
  })
    .bind(("0.0.0.0", 3000))?
    .run().await
}