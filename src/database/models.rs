use crate::schema::users::dsl::*;
use diesel::{expression::Selectable, mysql::*, prelude::*};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub is_admin: bool,
    pub hitpoints: u8,
    pub created_at: chrono::NaiveDateTime,
}
impl User {
    pub fn star(connection: &mut MysqlConnection) -> Result<Vec<User>, diesel::result::Error> {
        users
            .select((id, name, is_admin, hitpoints, created_at))
            .for_update()
            .skip_locked()
            .load::<User>(connection)
    }

    pub fn admins(connection: &mut MysqlConnection) -> Result<Vec<User>, diesel::result::Error> {
        users
            .select((id, name, is_admin, hitpoints, created_at))
            .filter(is_admin.eq(true))
            .for_update()
            .skip_locked()
            .load::<User>(connection)
    }
}
