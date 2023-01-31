use actix_web::{ post, web, HttpResponse, Responder };
use serde_json::json;
use serde::Deserialize;

use crate::tokens;
use crate::database;

#[derive(Deserialize, Debug)]
struct LoginFields {
  username: String,
  password: String,
}

#[post("/user/login")]
async fn login(req_body: web::Json<LoginFields>) -> impl Responder {
  println!("Attempting to login user: {}", req_body.username);
  // let email: String = String::from("no-reply@gmail.com");
  let username: String = req_body.username.to_string();
  let password: String = req_body.password.to_string();

  // Find user in database
  let is_user_valid = match database::find_user(&username, &password).await {
    Ok(_) => true,
    Err(_) => false,
  };

  if is_user_valid {
    println!("User {} is valid!", username);

    let token: Option<String> = tokens::get_token_for_username(&username);

    let return_data = json!({
    "username": username,
    "jwt": token,
    });

    HttpResponse::Ok().json(return_data)
  } else {
    // Send an error

    let return_data =
      json!({
    "error": {"code": "auth/invalid-credentials",
    "message": "The username or password you entered is incorrect."},
    });

    HttpResponse::Ok().json(return_data)
  }
}