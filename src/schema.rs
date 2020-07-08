table! {
    presencies (id) {
        id -> Integer,
        from -> Text,
        to -> Text,
        user -> Text,
        room -> Text,
    }
}

table! {
    rooms (id) {
        id -> Text,
        max_occupancy -> Integer,
    }
}

table! {
    users (id) {
        id -> Text,
        password_hash -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    presencies,
    rooms,
    users,
);
