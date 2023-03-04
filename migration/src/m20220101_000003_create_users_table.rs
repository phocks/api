use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
  fn name(&self) -> &str {
    "m20220101_000003_create_users_table" // Make sure this matches with the file name
  }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  // Define how to apply this migration: Create the Users table.
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager.create_table(
      Table::create()
        .table(Users::Table)
        .col(
          ColumnDef::new(Users::Id)
            .integer()
            .not_null()
            .auto_increment()
            .primary_key()
        )
        .col(ColumnDef::new(Users::NanoId).string().not_null())
        .col(ColumnDef::new(Users::Username).string().unique_key().not_null())
        .col(ColumnDef::new(Users::PasswordHash).string().not_null())
        .col(ColumnDef::new(Users::Email).string())
        .col(ColumnDef::new(Users::CreatedAt).string())
        .to_owned()
    ).await
  }

  // Define how to rollback this migration: Drop the Users table.
  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager.drop_table(Table::drop().table(Users::Table).to_owned()).await
  }
}

#[derive(Iden)]
pub enum Users {
  Table,
  Id,
  NanoId,
  PasswordHash,
  Username,
  Email,
  CreatedAt
}