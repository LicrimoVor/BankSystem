use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("auth")
                    .if_not_exists()
                    .col(string("refresh_token").primary_key().unique_key())
                    .col(uuid("user_id"))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-auth-users")
                            .from("auth", "user_id")
                            .to("users", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("auth").to_owned())
            .await
    }
}
