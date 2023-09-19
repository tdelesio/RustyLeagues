use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize, FromForm)]
#[serde(crate = "rocket::serde")]
#[sea_orm(table_name = "seasons")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub start_year: i32,
    pub end_year: i32,
    pub league_type: String

    // #[sea_orm(column_type = "Text")]
    // pub text: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
