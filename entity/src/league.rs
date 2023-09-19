use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize, FromForm)]
#[serde(crate = "rocket::serde")]
#[sea_orm(table_name = "leagues")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub league_name: String,
    pub paid_for: i32,
    pub money: bool,
    pub free: bool,
    pub active: bool,
    pub password: String,
    pub spreads: bool,
    pub double_enabled: bool,
    pub entry_fee: i32,
    pub weekly_fee: i32,
    pub first_place_percentage: i32,
    pub second_place_percentage: i32,
    pub third_place_percentage: i32,
    pub fourth_place_percentage: i32,
    pub fifth_place_percentage: i32,
    pub double_type: i32,
    pub banker: bool,
    pub season_id: i32,
    pub admin_id: i32

    // #[sea_orm(column_type = "Text")]
    // pub text: username,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
