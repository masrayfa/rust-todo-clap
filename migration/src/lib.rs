pub use sea_orm_migration::prelude::*;

mod m20231102_030846_create_user;
mod m20231102_031615_create_todo;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20231102_030846_create_user::Migration),
            Box::new(m20231102_031615_create_todo::Migration),
        ]
    }
}
