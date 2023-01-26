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
  let db = match Database::connect(DATABASE_URL).await {
    Ok(db) => db,
    Err(err) => {
      println!("$$$$$$$$$$$$$$$$$$$$$$$$$$ {}", err);
      return Err(err);
    }
  };

  // let db = &(match db.get_database_backend() {
  //   DbBackend::MySql => {
  //     db.execute(
  //       Statement::from_string(
  //         db.get_database_backend(),
  //         format!("CREATE DATABASE IF NOT EXISTS `{}`;", DB_NAME)
  //       )
  //     ).await?;

  //     let url = format!("{}/{}", DATABASE_URL, DB_NAME);
  //     Database::connect(&url).await?
  //   }
  //   DbBackend::Postgres => {
  //     db.execute(
  //       Statement::from_string(
  //         db.get_database_backend(),
  //         format!("DROP DATABASE IF EXISTS \"{}\";", DB_NAME)
  //       )
  //     ).await?;
  //     db.execute(
  //       Statement::from_string(
  //         db.get_database_backend(),
  //         format!("CREATE DATABASE \"{}\";", DB_NAME)
  //       )
  //     ).await?;

  //     let url = format!("{}/{}", DATABASE_URL, DB_NAME);
  //     Database::connect(&url).await?
  //   }
  //   DbBackend::Sqlite => db,
  // });

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