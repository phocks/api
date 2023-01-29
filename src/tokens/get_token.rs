use chrono::{ Duration, Utc };
use jsonwebtoken::{
  decode,
  encode,
  DecodingKey,
  EncodingKey,
  Header,
  Validation,
};
use serde::{ Deserialize, Serialize };
use dotenv_codegen::dotenv as dotenv_codegen;

#[derive(Deserialize, Serialize, Debug)]
struct Claims {
  sub: String,
  iat: usize,
  exp: usize,
  username: String,
}

pub fn get_token_for_username(username: &String) -> std::string::String {
  let bytes_secret = dotenv_codegen!("SECRET").as_bytes();

  let key = bytes_secret;
  let my_iat = Utc::now().timestamp();
  let my_exp = Utc::now()
    .checked_add_signed(Duration::days(365))
    .expect("invalid timestamp")
    .timestamp();

  let my_claims = Claims {
    sub: "auth".to_string(),
    iat: my_iat as usize,
    exp: my_exp as usize,
    username: username.to_string(),
  };

  let token = match
    encode(&Header::default(), &my_claims, &EncodingKey::from_secret(key))
  {
    Ok(t) => t,
    Err(_) => panic!(),
  };

  return token;
}