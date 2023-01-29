// #![allow(unused)]

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



#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();

  println!("Attempting to listen on http://localhost:3000");

  HttpServer::new(|| {
    App::new()
      .wrap(Cors::permissive())
      .service(routes::home)
      .service(routes::get_jwt)
      .service(routes::user_register)
  })
    .bind(("0.0.0.0", 3000))?
    .run().await
}

// let valid = match verify(&password, &hashed) {
//   Ok(v) => v,
//   Err(e) => panic!("Error verifying password: {}", e),
// };
// println!("Is password valid? {}", valid);