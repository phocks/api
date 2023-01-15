use chrono::{ Duration, Utc };
use jsonwebtoken::{ decode, encode, DecodingKey, EncodingKey, Header, Validation };
use serde::{ Deserialize, Serialize };

#[macro_use]
use dotenv_codegen::dotenv as dotenv_codegen;

#[derive(Deserialize, Serialize, Debug)]
struct Claims {
  sub: String,
  iat: usize,
  exp: usize,
  email: String,
}

pub fn get_token(email: &String) -> std::string::String {
  let bytes_secret = dotenv_codegen!("SECRET").as_bytes();

  let key = bytes_secret;
  let my_iat = Utc::now().timestamp();
  let my_exp = Utc::now()
    .checked_add_signed(Duration::days(365))
    .expect("invalid timestamp")
    .timestamp();

  let my_claims = Claims {
    sub: "h@d.com".to_owned(),
    iat: my_iat as usize,
    exp: my_exp as usize,
    email: email.to_owned(),
  };

  let token = match encode(&Header::default(), &my_claims, &EncodingKey::from_secret(key)) {
    Ok(t) => t,
    Err(_) => panic!(),
  };

  return token;
}