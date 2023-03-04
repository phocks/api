pub use sea_orm_migration::prelude::*;

// Add each migration file as a module
mod m20220101_000003_create_users_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
  fn migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![
      // Define the order of migrations.
      Box::new(m20220101_000003_create_users_table::Migration)
    ]
  }
}