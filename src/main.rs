// mod email;
// use email::email;
use chrono::{ Duration, Utc };
use jsonwebtoken::{ decode, encode, DecodingKey, EncodingKey, Header, Validation };
use serde::{ Deserialize, Serialize };
use serde_json::json;
use actix_web::{ get, post, web, App, HttpResponse, HttpServer, Responder };

#[derive(Deserialize, Serialize, Debug)]
struct Claims {
  sub: String,
  iat: usize,
  exp: usize,
  test: String,
}

#[get("/")]
async fn hello() -> impl Responder {
  let key = b"a secret code";
  let my_iat = Utc::now().timestamp();
  let my_exp = Utc::now()
    .checked_add_signed(Duration::days(365))
    .expect("invalid timestamp")
    .timestamp();

  let my_claims = Claims {
    sub: "h@d.com".to_owned(),
    iat: my_iat as usize,
    exp: my_exp as usize,
    test: "hello world".to_owned(),
  };

  let token = match encode(&Header::default(), &my_claims, &EncodingKey::from_secret(key)) {
    Ok(t) => t,
    Err(_) => panic!(),
  };

  // println!("token: {:?}", token);

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