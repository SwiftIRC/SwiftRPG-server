// @generated automatically by Diesel CLI.

pub mod sql_types {
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
    tiles (id) {
        id -> Unsigned<Bigint>,
        discovered_by -> Nullable<Unsigned<Bigint>>,
        discovered_at -> Nullable<Timestamp>,
        terrain_id -> Unsigned<Bigint>,
        #[max_length = 100]
        psuedo_id -> Varchar,
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
diesel::joinable!(users -> buildings (building_id));
diesel::joinable!(users -> tiles (tile_id));

diesel::allow_tables_to_appear_in_same_query!(
    buildings,
    item_properties,
    item_property_user,
    items,
    tiles,
    users,
    zones,
);
