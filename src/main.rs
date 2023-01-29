#![allow(unused)]

// Local modules
mod entities;
mod tokens;
mod database;
mod routes;

use actix_cors::Cors;
use actix_web::{ get, post, web, App, HttpResponse, HttpServer, Responder };
use serde_json::json;
use serde::Deserialize;
use dotenv::dotenv;

use bcrypt;
const REDUCED_COST: u32 = 8;

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

  let hashed = bcrypt::hash(&password, REDUCED_COST).expect("Hashing error!");

  // let valid = match verify(&password, &hashed) {
  //   Ok(v) => v,
  //   Err(e) => panic!("Error verifying password: {}", e),
  // };
  // println!("Is password valid? {}", valid);

  println!("Hashed password: {}", hashed);

  database::insert_blocking(&username, &hashed);

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
      .service(routes::home)
      .service(echo)
      .service(get_jwt)
      .service(user_register)
  })
    .bind(("0.0.0.0", 3000))?
    .run().await
}