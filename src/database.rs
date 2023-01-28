use futures::executor::block_on;

use crate::entities::{ prelude::*, * };
use sea_orm::*;

const DATABASE_URL: &str = "sqlite:./sqlite.db?mode=rwc";
const DB_NAME: &str = "users";

async fn run() -> Result<(), DbErr> {
  let db = Database::connect(DATABASE_URL).await?;

  Ok(())
}

async fn insert_new_user(username: &str, password: &str) -> Result<(), DbErr> {
  let db = Database::connect(DATABASE_URL).await?;

  println!("Attempting to insert new user: {}", username);

  let new_user = users::ActiveModel {
    username: ActiveValue::Set(username.to_string()),
    password_hash: ActiveValue::Set(password.to_string()),
    ..Default::default()
  };
  let res = Users::insert(new_user).exec(&db).await?;

  Ok(())
}

pub fn insert_blocking(username: &str, password: &str) {
  if let Err(err) = block_on(insert_new_user(username, password)) {
    panic!("{}", err);
  }
}

pub fn connect() {
  if let Err(err) = block_on(run()) {
    panic!("{}", err);
  }
}