//! SeaORM Entity. Generated by sea-orm-codegen 0.4.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "xf_hb_chat_message")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub message_id: u32,
    #[sea_orm(column_type = "Text")]
    pub message_text: String,
    pub message_action: bool,
    pub message_discouraged: bool,
    #[sea_orm(column_type = "Decimal(Some((16, 6)))")]
    pub message_date: Decimal,
    #[sea_orm(column_type = "Decimal(Some((16, 6)))")]
    pub message_update: Decimal,
    pub room_id: u32,
    pub warning_id: Option<u32>,
    pub ip_id: Option<u32>,
    pub user_id: Option<u32>,
    pub username: String,
    pub recipient_id: Option<u32>,
    pub recipient_username: Option<String>,
    #[sea_orm(column_type = "Decimal(Some((16, 6)))", nullable)]
    pub last_edit_date: Option<Decimal>,
    pub last_edit_user_id: Option<u32>,
    pub last_edit_username: Option<String>,
    #[sea_orm(column_type = "Decimal(Some((16, 6)))", nullable)]
    pub deleted_date: Option<Decimal>,
    pub deleted_user_id: Option<u32>,
    pub deleted_username: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
