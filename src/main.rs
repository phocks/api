// #![allow(unused)]

// Local modules
mod entities;
mod tokens;
mod database;
// mod schema;
mod star_wars;

// Load our API routes
mod routes;

// Actix web imports
use actix_cors::Cors;
use actix_web::{ guard, web, web::Data, App, HttpResponse, HttpServer, Result };
use async_graphql::{
  http::GraphiQLSource,
  EmptyMutation,
  EmptySubscription,
  Schema,
};
use async_graphql_actix_web::{ GraphQLRequest, GraphQLResponse };
// use schema::*;
use star_wars::{ QueryRoot, StarWars, StarWarsSchema };

// type SchemaType = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

use dotenv::dotenv;

#[macro_use]
extern crate dotenv_codegen;

async fn index(
  schema: web::Data<StarWarsSchema>,
  req: GraphQLRequest
) -> GraphQLResponse {
  schema.execute(req.into_inner()).await.into()
}

async fn index_graphiql() -> Result<HttpResponse> {
  Ok(
    HttpResponse::Ok()
      .content_type("text/html; charset=utf-8")
      .body(GraphiQLSource::build().endpoint("/graphql").finish())
  )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  // Load local environment variables from .env file
  dotenv().ok();

  let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
    .data(StarWars::new())
    .finish();

  println!("Attempting to listen on http://localhost:3000");

  HttpServer::new(move || {
    App::new()
      .wrap(Cors::permissive())
      .app_data(Data::new(schema.clone()))
      .service(routes::home)
      .service(web::resource("/graphql").guard(guard::Post()).to(index))
      .service(web::resource("/graphql").guard(guard::Get()).to(index_graphiql))
      .service(
        web
          ::scope("/v1")
          .service(routes::user::login)
          .service(routes::user::register)
      )
  })
    // App::new()
    //   .app_data(Data::new(schema.clone()))
    //   .wrap(Cors::permissive())
    //   .service(routes::home)
    //   .service(
    //     web
    //       ::scope("/v1")
    //       .service(routes::user::login)
    //       .service(routes::user::register)
    //       .service(web::resource("/").guard(guard::Post()).to(index))
    //       .service(web::resource("/").guard(guard::Get()).to(index_graphiql))
    //   )
    // })
    .bind(("0.0.0.0", 3000))?
    .run().await
}

// mod star_wars;

// use actix_web::{guard, web, web::Data, App, HttpResponse, HttpServer, Result};
// use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
// use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
// use star_wars::{QueryRoot, StarWars, StarWarsSchema};

// async fn index(schema: web::Data<StarWarsSchema>, req: GraphQLRequest) -> GraphQLResponse {
//     schema.execute(req.into_inner()).await.into()
// }

// async fn index_graphiql() -> Result<HttpResponse> {
//     Ok(HttpResponse::Ok()
//         .content_type("text/html; charset=utf-8")
//         .body(GraphiQLSource::build().endpoint("/").finish()))
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
//         .data(StarWars::new())
//         .finish();

//     println!("GraphiQL IDE: http://localhost:8000");

//     HttpServer::new(move || {
//         App::new()
//             .app_data(Data::new(schema.clone()))
//             .service(web::resource("/").guard(guard::Post()).to(index))
//             .service(web::resource("/").guard(guard::Get()).to(index_graphiql))
//     })
//     .bind("127.0.0.1:8000")?
//     .run()
//     .await
// }