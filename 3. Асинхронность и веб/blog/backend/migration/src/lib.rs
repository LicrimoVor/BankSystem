pub use sea_orm_migration::prelude::*;

mod m20260208_094916_user;
mod m20260208_175902_post;
mod m20260210_034214_auth;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260208_094916_user::Migration),
            Box::new(m20260208_175902_post::Migration),
            Box::new(m20260210_034214_auth::Migration),
        ]
    }
}
