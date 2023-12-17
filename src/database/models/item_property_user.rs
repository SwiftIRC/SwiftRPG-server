use diesel::{associations::*, expression::*, mysql::*, prelude::*};

#[derive(Queryable, Selectable, Clone, Debug, Identifiable, PartialEq)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(table_name = crate::schema::item_property_user)]
pub struct ItemPropertyUser {
    pub id: u32,
    pub user_id: u32,
    pub item_id: u32,
    pub property_id: u32,
    pub quantity: u32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable, Clone, Debug, PartialEq)]
#[diesel(table_name = crate::schema::item_property_user)]
pub struct NewItemPropertyUser {
    pub user_id: u32,
    pub item_id: u32,
    pub property_id: u32,
    pub quantity: u32,
}
