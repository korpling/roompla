table! {
    presency (id) {
        id -> Nullable<Integer>,
        datetime -> Text,
        user -> Nullable<Text>,
    }
}

table! {
    room (id) {
        id -> Text,
        max_occupancy -> Integer,
    }
}

table! {
    user (id) {
        id -> Text,
        password_hash -> Nullable<Text>,
    }
}

joinable!(presency -> user (user));

allow_tables_to_appear_in_same_query!(
    presency,
    room,
    user,
);
