use super::item::Item;
use super::item_property_user::{ItemPropertyUser, NewItemPropertyUser};
use super::skill::Skill;
use super::skill_user::{NewSkillUser, SkillUser};

use crate::schema::item_property_user::dsl::{
    id as item_property_user_id, item_id as item_property_user_item_id,
    property_id as item_property_user_property_id, quantity as item_property_user_quantity,
    user_id as item_property_user_user_id, *,
};
use crate::schema::items::dsl::{
    description as items_description, id as items_id, name as items_name, ticks as items_ticks,
    weight as items_weight, *,
};
use crate::schema::skill_user::dsl::{
    id as skill_user_id, quantity as skill_user_quantity, skill_id as skill_user_skill_id,
    user_id as skill_user_user_id, *,
};
use crate::schema::skills::dsl::{id as skills_id, name as skills_name, *};
use crate::schema::users::dsl::{
    created_at as users_created_at, id as users_id, name as users_name,
    updated_at as users_updated_at, *,
};
use diesel::{expression::Selectable, mysql::*, prelude::*};

#[derive(Clone, Debug, PartialEq, Identifiable, Selectable, Queryable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct User {
    pub id: u32,
    pub name: String,
    pub is_admin: bool,
    pub hitpoints: u8,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
    pub tile_id: u32,
    pub building_id: Option<u32>,
}

impl User {
    pub fn star(connection: &mut MysqlConnection) -> Result<Vec<User>, diesel::result::Error> {
        users
            .select(User::as_select())
            .for_update()
            .skip_locked()
            .load::<User>(connection)
    }

    pub fn admins(connection: &mut MysqlConnection) -> Result<Vec<User>, diesel::result::Error> {
        users
            .select(User::as_select())
            .filter(is_admin.eq(true))
            .for_update()
            .skip_locked()
            .load::<User>(connection)
    }

    pub fn add_xp(
        self,
        skill: u32,
        xp: u32,
        connection: &mut MysqlConnection,
    ) -> Result<usize, diesel::result::Error> {
        diesel::insert_into(skill_user)
            .values(NewSkillUser {
                user_id: self.id,
                skill_id: skill,
                quantity: xp,
            })
            .on_conflict(diesel::dsl::DuplicatedKeys)
            .do_update()
            .set(skill_user_quantity.eq(skill_user_quantity + xp))
            .execute(connection)
    }

    pub fn xp(
        self,
        skill: u32,
        connection: &mut MysqlConnection,
    ) -> Result<u32, diesel::result::Error> {
        skill_user
            .select(skill_user_quantity)
            .filter(
                skill_user_skill_id
                    .eq(skill)
                    .and(skill_user_user_id.eq(self.id)),
            )
            .first(connection)
    }

    pub fn skills(
        self,
        connection: &mut MysqlConnection,
    ) -> Result<Vec<(SkillUser, Option<Skill>)>, diesel::result::Error> {
        SkillUser::belonging_to(&self)
            .inner_join(skills.on(skill_user_skill_id.eq(skills_id)))
            .select((SkillUser::as_select(), Option::<Skill>::as_select()))
            .order_by(skill_user_id.asc())
            .load::<(SkillUser, Option<Skill>)>(connection)
    }

    pub fn add_item(
        self,
        item: Item,
        amount: u32,
        connection: &mut MysqlConnection,
    ) -> Result<usize, diesel::result::Error> {
        diesel::insert_into(item_property_user)
            .values(NewItemPropertyUser {
                user_id: self.id,
                item_id: item.id,
                property_id: 1, // TODO: Make this dynamic
                quantity: amount,
            })
            .on_conflict(diesel::dsl::DuplicatedKeys)
            .do_update()
            .set(item_property_user_quantity.eq(item_property_user_quantity + amount))
            .execute(connection)
    }
}
