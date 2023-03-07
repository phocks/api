// #![allow(unused)]

// Local modules
mod entities;
mod tokens;
mod database;

// Actix web imports
use actix_cors::Cors;
use actix_web::{ web, App, HttpServer };

// Load our API routes
mod routes;

use dotenv::dotenv;

#[macro_use]
extern crate dotenv_codegen;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  // Load local environment variables from .env file
  dotenv().ok();

  println!("Attempting to listen on http://localhost:3000");

  HttpServer::new(|| {
    App::new()
      .wrap(Cors::permissive())
      .service(routes::home)
      .service(
        web
          ::scope("/v1")
          .service(routes::user::login)
          .service(routes::user::register)
      )
  })
    .bind(("0.0.0.0", 3000))?
    .run().await
}