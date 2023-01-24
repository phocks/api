use futures::executor::block_on;

use crate::entities::{ prelude::*, * };
use sea_orm::*;

// Change this according to your database implementation,
// or supply it as an environment variable.
// the whole database URL string follows the following format:
// "protocol://username:password@host:port/database"
// We put the database name (that last bit) in a separate variable simply for convenience.
const DATABASE_URL: &str = "sqlite:./db/sqlite.db?mode=rwc";
const DB_NAME: &str = "bakeries_db";

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

pub fn connect() {
  if let Err(err) = block_on(run()) {
    panic!("{}", err);
  }
}