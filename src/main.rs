// #![allow(unused)]

// Local modules
mod entities;
mod tokens;
mod database;
mod schema;

// Actix web imports
use actix_cors::Cors;
use actix_web::{ guard, web, web::Data, App, HttpResponse, HttpServer, Result };
use async_graphql::{
  http::GraphiQLSource,
  EmptyMutation,
  EmptySubscription,
  Schema,
};
use async_graphql_actix_web::*;
use schema::*;

type SchemaType = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

use juniper::http::{ graphiql::graphiql_source, GraphQLRequest };

// Load our API routes
mod routes;

use dotenv::dotenv;

#[macro_use]
extern crate dotenv_codegen;

async fn index_graphiql() -> Result<HttpResponse> {
  Ok(
    HttpResponse::Ok()
      .content_type("text/html; charset=utf-8")
      .body(GraphiQLSource::build().endpoint("/").finish())
  )
}

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
          .service(web::resource("/").guard(guard::Get()).to(index_graphiql))
      )
  })
    .bind(("0.0.0.0", 3000))?
    .run().await
}