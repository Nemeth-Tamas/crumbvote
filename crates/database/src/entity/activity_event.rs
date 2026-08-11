use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "activity_events")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub event_id: i32,
    pub entry_id: i32,
    pub voter_hash: String,
    pub kind: String,
    pub created_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
