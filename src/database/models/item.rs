use diesel::{associations::*, expression::*, mysql::*, prelude::*};

#[derive(Queryable, Selectable, Clone, Debug, Identifiable, PartialEq)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(table_name = crate::schema::items)]
pub struct Item {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub weight: u32,
    pub ticks: f32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable, Clone, Debug, PartialEq)]
#[diesel(table_name = crate::schema::items)]
pub struct NewItem {
    pub name: String,
    pub description: String,
    pub weight: u32,
    pub ticks: f32,
}
