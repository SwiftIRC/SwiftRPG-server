use diesel::{allow_tables_to_appear_in_same_query, joinable, prelude::*};

diesel::table! {
    users (id) {
        id -> Unsigned<Int4>,
        name -> Varchar,
        is_admin -> Bool,
        hitpoints -> Unsigned<Tinyint>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
        tile_id -> Unsigned<Int4>,
        building_id -> Nullable<Unsigned<Int4>>,
    }
}

diesel::table! {
    skills (id) {
        id -> Unsigned<Int4>,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    skill_user (id) {
        id -> Unsigned<Int4>,
        user_id -> Unsigned<Int4>,
        skill_id -> Unsigned<Int4>,
        quantity -> Unsigned<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(skills -> skill_user (id));
allow_tables_to_appear_in_same_query!(skills, skill_user);

diesel::table! {
    items (id) {
        id-> Unsigned<Int4>,
        name-> Varchar,
        description-> Varchar,
        weight-> Unsigned<Int4>,
        ticks-> Float,
        created_at-> Timestamp,
        updated_at-> Timestamp,
        deleted_at-> Nullable<Timestamp>,
    }
}

diesel::table! {
    item_property_user (id) {
        id-> Unsigned<Int4>,
        user_id-> Unsigned<Int4>,
        item_id-> Unsigned<Int4>,
        property_id -> Unsigned<Int4>,
        quantity-> Unsigned<Int4>,
        created_at-> Timestamp,
        updated_at-> Timestamp,
        deleted_at-> Nullable<Timestamp>,
    }
}

diesel::table! {
    item_properties (id) {
        id-> Unsigned<Int4>,
        name-> Varchar,
        description-> Varchar,
        created_at-> Timestamp,
        updated_at-> Timestamp,
    }
}
