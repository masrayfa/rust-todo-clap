use sea_orm_migration::prelude::*;
use super::m20231102_030846_create_user::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(Todo::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Todo::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Todo::Title).string().not_null())
                    .col(ColumnDef::new(Todo::Task).string().not_null())
                    .col(ColumnDef::new(Todo::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Todo::DueDate).date_time().not_null())
                    .col(ColumnDef::new(Todo::UserId).string().not_null())
                    .foreign_key(ForeignKey::create()
                        .name("fk_todo_user_id")
                        .from_col(Todo::UserId)
                        .to_tbl(User::Table)
                        .to_col(User::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(Todo::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Todo {
    Table,
    Id,
    Title,
    Task,
    CreatedAt,
    DueDate,
    UserId
}
