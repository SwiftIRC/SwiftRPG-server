// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Mediumint"))]
    pub struct NpcSkillQuantityMediumint;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Enum"))]
    pub struct NpcsGenderEnum;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Enum"))]
    pub struct NpcsSpeciesEnum;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Mediumint"))]
    pub struct SkillUserQuantityMediumint;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Mediumint"))]
    pub struct UsersHitpointsMediumint;
}

diesel::table! {
    buildings (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 1000]
        description -> Nullable<Varchar>,
        zone_id -> Unsigned<Bigint>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    item_properties (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 255]
        description -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    item_property_user (id) {
        id -> Unsigned<Bigint>,
        user_id -> Unsigned<Bigint>,
        item_id -> Unsigned<Bigint>,
        property_id -> Unsigned<Bigint>,
        quantity -> Unsigned<Bigint>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    items (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 255]
        description -> Nullable<Varchar>,
        weight -> Unsigned<Bigint>,
        ticks -> Unsigned<Double>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    names (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        species -> Varchar,
        #[max_length = 255]
        gender -> Nullable<Varchar>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::NpcSkillQuantityMediumint;

    npc_skill (id) {
        id -> Unsigned<Bigint>,
        skill_id -> Unsigned<Bigint>,
        npc_id -> Unsigned<Bigint>,
        quantity -> Unsigned<NpcSkillQuantityMediumint>,
    }
}

diesel::table! {
    npc_tile (id) {
        id -> Unsigned<Bigint>,
        tile_id -> Unsigned<Bigint>,
        npc_id -> Unsigned<Bigint>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::NpcsSpeciesEnum;
    use super::sql_types::NpcsGenderEnum;

    npcs (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 50]
        first_name -> Varchar,
        #[max_length = 50]
        last_name -> Varchar,
        #[max_length = 5]
        species -> NpcsSpeciesEnum,
        #[max_length = 10]
        gender -> NpcsGenderEnum,
        occupation_id -> Nullable<Unsigned<Bigint>>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    occupations (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 1000]
        description -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::SkillUserQuantityMediumint;

    skill_user (id) {
        id -> Unsigned<Bigint>,
        skill_id -> Unsigned<Bigint>,
        user_id -> Unsigned<Bigint>,
        quantity -> Unsigned<SkillUserQuantityMediumint>,
    }
}

diesel::table! {
    skills (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    terrains (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 1000]
        description -> Nullable<Varchar>,
        movement_cost -> Smallint,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    tiles (id) {
        id -> Unsigned<Bigint>,
        discovered_by -> Unsigned<Bigint>,
        discovered_at -> Nullable<Timestamp>,
        terrain_id -> Unsigned<Bigint>,
        x -> Bigint,
        y -> Bigint,
        max_trees -> Unsigned<Smallint>,
        available_trees -> Unsigned<Smallint>,
        max_ore -> Unsigned<Smallint>,
        available_ore -> Unsigned<Smallint>,
        last_disturbed -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UsersHitpointsMediumint;

    users (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        is_admin -> Bool,
        #[max_length = 100]
        remember_token -> Nullable<Varchar>,
        hitpoints -> Unsigned<UsersHitpointsMediumint>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        tile_id -> Unsigned<Bigint>,
        building_id -> Nullable<Unsigned<Bigint>>,
    }
}

diesel::table! {
    zone_properties (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 1000]
        description -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    zones (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 1000]
        description -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(buildings -> zones (zone_id));
diesel::joinable!(item_property_user -> item_properties (property_id));
diesel::joinable!(item_property_user -> items (item_id));
diesel::joinable!(item_property_user -> users (user_id));
diesel::joinable!(npc_skill -> npcs (npc_id));
diesel::joinable!(npc_skill -> skills (skill_id));
diesel::joinable!(npc_tile -> npcs (npc_id));
diesel::joinable!(npc_tile -> tiles (tile_id));
diesel::joinable!(npcs -> occupations (occupation_id));
diesel::joinable!(skill_user -> skills (skill_id));
diesel::joinable!(skill_user -> users (user_id));
diesel::joinable!(tiles -> terrains (terrain_id));
diesel::joinable!(users -> buildings (building_id));

diesel::allow_tables_to_appear_in_same_query!(
    buildings,
    item_properties,
    item_property_user,
    items,
    names,
    npc_skill,
    npc_tile,
    npcs,
    occupations,
    skill_user,
    skills,
    terrains,
    tiles,
    users,
    zone_properties,
    zones,
);
