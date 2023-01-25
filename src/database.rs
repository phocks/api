use futures::executor::block_on;

use crate::entities::{ prelude::*, * };
use sea_orm::*;

// Change this according to your database implementation,
// or supply it as an environment variable.
// the whole database URL string follows the following format:
// "protocol://username:password@host:port/database"
// We put the database name (that last bit) in a separate variable simply for convenience.
const DATABASE_URL: &str = "sqlite:./db/sqlite.db?mode=rwc";
const DB_NAME: &str = "users";

async fn run() -> Result<(), DbErr> {
  let db = Database::connect(DATABASE_URL).await?;

  // -- Sample code to create an entity --
  // let happy_bakery = bakery::ActiveModel {
  //   name: ActiveValue::Set("Happy Bakery".to_owned()),
  //   profit_margin: ActiveValue::Set(0.0),
  //   ..Default::default()
  // };
  // let res = Bakery::insert(happy_bakery).exec(&db).await?;

  Ok(())
}

async fn insert_new_user(username: &str, password: &str) -> Result<(), DbErr> {
  let db = Database::connect(DATABASE_URL).await?;

  println!("Attempting to insert new user: {}", username);

  // -- Sample code to create an entity --
  // TODO: fix make no null
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