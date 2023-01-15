use chrono::{ Duration, Utc };
use jsonwebtoken::{ decode, encode, DecodingKey, EncodingKey, Header, Validation };
use serde::{ Deserialize, Serialize };

#[derive(Deserialize, Serialize, Debug)]
struct Claims {
  sub: String,
  iat: usize,
  exp: usize,
  email: String,
}

pub fn get_token() -> std::string::String {
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
    email: "phocks@gmail.com".to_owned(),
  };

  let token = match encode(&Header::default(), &my_claims, &EncodingKey::from_secret(key)) {
    Ok(t) => t,
    Err(_) => panic!(),
  };

  // println!("token: {:?}", token);
  println!("{}", dotenv!("SECRET"));

  return token;
}