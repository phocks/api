// #![allow(unused)]

// Local modules
mod entities;
mod tokens;
mod database;

// Actix web imports
use actix_cors::Cors;
use actix_web::{ get, post, web, App, HttpResponse, HttpServer, Responder };

// Load our API routes
mod routes;

use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  // Load local environment variables from .env file
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