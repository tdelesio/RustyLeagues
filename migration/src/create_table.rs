use sea_orm_migration::prelude::*;
use sea_orm::{DbBackend, Schema, Statement};
use entity::League;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db_postgres = manager.get_database_backend();
        let schema = Schema::new(db_postgres);

        let statement = db_postgres.build(&schema.create_table_from_entity(League));
        manager.create_table(Table::create().

        ).await


        // manager
        //     .create_table(
        //         Table::create()
        //             .table(Posts::Table)
        //             .if_not_exists()
        //             .col(
        //                 ColumnDef::new(Posts::Id)
        //                     .integer()
        //                     .not_null()
        //                     .auto_increment()
        //                     .primary_key(),
        //             )
        //             .col(ColumnDef::new(Posts::Title).string().not_null())
        //             .col(ColumnDef::new(Posts::Text).string().not_null())
        //             .to_owned(),
        //     )
        //     .await
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