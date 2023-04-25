use futures::executor::block_on;
use chrono::Utc;

use crate::entities::{ prelude::*, * };
use sea_orm::*;
use dotenv_codegen::dotenv;

const DATABASE_URL: &str = dotenv!("DB_URL");

async fn insert_new_user(
  username: &str,
  password: &str,
  nano_id: &str
) -> Result<(), DbErr> {
  let db = Database::connect(DATABASE_URL).await?;

  println!("Attempting to insert new user: {}", username);

  let new_user = users::ActiveModel {
    username: ActiveValue::Set(username.to_string()),
    password_hash: ActiveValue::Set(password.to_string()),
    nano_id: ActiveValue::Set(nano_id.to_string()),
    created_at: ActiveValue::Set(Utc::now()),
    ..Default::default()
  };
  let _response = Users::insert(new_user).exec(&db).await?;

  Ok(())
}

// Login function
pub async fn find_user(username: &str, password: &str) -> Result<(), DbErr> {
  println!("{}", DATABASE_URL);
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

pub fn insert_blocking(
  username: &str,
  password: &str,
  nano_id: &str
) -> Option<()> {
  if let Err(err) = block_on(insert_new_user(username, password, nano_id)) {
    // panic!("{}", err);
    println!("{}", err);
    return None;
  }

  Some(())
}