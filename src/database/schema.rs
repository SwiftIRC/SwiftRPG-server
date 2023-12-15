diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        is_admin -> Bool,
        hitpoints -> Unsigned<Tinyint>,
        created_at -> Timestamp,
    }
}
