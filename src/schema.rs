table! {
    occupancies (id) {
        id -> Integer,
        start -> Timestamp,
        end -> Timestamp,
        user_id -> Text,
        user_name -> Text,
        user_contact -> Text,
        room -> Text,
    }
}

table! {
    rooms (id) {
        id -> Text,
        max_occupancy -> Integer,
        timezone -> Nullable<Text>,
    }
}

table! {
    users (id) {
        id -> Text,
        display_name -> Text,
        contact_info -> Text,
        password_hash -> Nullable<Text>,
    }
}

joinable!(occupancies -> rooms (room));

allow_tables_to_appear_in_same_query!(occupancies, rooms, users,);
