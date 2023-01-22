use futures::executor::block_on;
use sea_orm::{ Database, DbErr };

// Change this according to your database implementation,
// or supply it as an environment variable.
// the whole database URL string follows the following format:
// "protocol://username:password@host:port/database"
// We put the database name (that last bit) in a separate variable simply for convenience.
const DATABASE_URL: &str = "sqlite:./sqlite.db?mode=rwc";
const DB_NAME: &str = "bakeries_db";

async fn run() -> Result<(), DbErr> {
  let db = Database::connect(DATABASE_URL).await?;

  Ok(())
}

pub fn connect() {
  if let Err(err) = block_on(run()) {
    panic!("{}", err);
  }
}