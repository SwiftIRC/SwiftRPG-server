use super::skill::Skill;
use super::user::User;

use diesel::{expression::*, mysql::*, prelude::*};

#[derive(Identifiable, Associations, Queryable, Selectable, Clone, Debug, PartialEq)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(belongs_to(Skill, foreign_key = skill_id))]
#[diesel(table_name = crate::schema::skill_user)]
pub struct SkillUser {
    pub id: u32,
    pub user_id: u32,
    pub skill_id: u32,
    pub quantity: u32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl SkillUser {
    pub fn all(connection: &mut MysqlConnection) -> Result<Vec<SkillUser>, diesel::result::Error> {
        crate::schema::skill_user::table.load::<SkillUser>(connection)
    }
}

#[derive(Insertable, Clone, Debug, PartialEq)]
#[diesel(table_name = crate::schema::skill_user)]
pub struct NewSkillUser {
    pub user_id: u32,
    pub skill_id: u32,
    pub quantity: u32,
}
