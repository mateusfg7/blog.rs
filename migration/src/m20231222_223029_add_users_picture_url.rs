use std::borrow::BorrowMut;

use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum Users {
    Table,
    PictureUrl,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _ = manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .add_column_if_not_exists(string(Users::PictureUrl).borrow_mut())
                    .to_owned(),
            )
            .await;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _ = manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .drop_column(Users::PictureUrl)
                    .to_owned(),
            )
            .await;

        Ok(())
    }
}
