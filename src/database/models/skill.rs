use diesel::{associations::*, expression::*, mysql::*, prelude::*};

use crate::schema::skills::dsl::*;

#[derive(Queryable, Selectable, Clone, Debug, Identifiable, PartialEq)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(table_name = crate::schema::skills)]
pub struct Skill {
    pub id: u32,
    pub name: String,
}

impl Skill {
    pub fn all(connection: &mut MysqlConnection) -> Result<Vec<Skill>, diesel::result::Error> {
        skills.select((id, name)).load::<Skill>(connection)
    }
}
