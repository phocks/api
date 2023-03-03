use futures::executor::block_on;

use crate::entities::{ prelude::*, * };
use sea_orm::*;

const DATABASE_URL: &str = "sqlite:./sqlite.db?mode=rwc";

async fn insert_new_user(username: &str, password: &str) -> Result<(), DbErr> {
  let db = Database::connect(DATABASE_URL).await?;

  println!("Attempting to insert new user: {}", username);

  let new_user = users::ActiveModel {
    username: ActiveValue::Set(username.to_string()),
    password_hash: ActiveValue::Set(password.to_string()),
    ..Default::default()
  };
  let _res = Users::insert(new_user).exec(&db).await?;

  Ok(())
}

// Login function
pub async fn find_user(username: &str, password: &str) -> Result<(), DbErr> {
  println!("find_user running {}", username);
  let db = Database::connect(DATABASE_URL).await?;

  println!("db {:?}", db);

  let user: Option<users::Model> = match
    Users::find().filter(users::Column::Username.eq(username)).one(&db).await
  {
    Ok(result) => result,
    Err(err) => {
      println!("{:?}", err);
      return Err(DbErr::from(err));
    } // Convert the error to the DbErr enum
  };

  println!("User found {:?}", user);

  if let Some(user) = user {
    println!("User found");
    let hash = user.password_hash;
    let is_valid = bcrypt::verify(password, &hash).unwrap();

    if is_valid {
      return Ok(());
    } else {
      return Err(DbErr::Custom("User is not valid".to_string()));
    }
  } else {
    println!("nothing");
    return Err(DbErr::Custom("User is not valid".to_string()));
  }
}

pub fn insert_blocking(username: &str, password: &str) -> Option<()> {
  if let Err(err) = block_on(insert_new_user(username, password)) {
    // panic!("{}", err);
    println!("{}", err);
    return None;
  }

  Some(())
}