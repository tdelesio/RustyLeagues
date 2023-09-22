use sea_orm::Schema;
use sea_orm_migration::prelude::*;
use entity::League;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        println!("Inside Migration up method");

        let db_postgres = manager.get_database_backend();
        let schema = Schema::new(db_postgres);

        manager.create_table(
            schema.create_table_from_entity(League)
        ).await

    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(LeagueE::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum LeagueE {
    Table,
}