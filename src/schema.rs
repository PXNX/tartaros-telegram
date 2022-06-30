table! {
    reports (id) {
        id -> Int4,
        author -> Int4,
        date -> Timestamp,
        user_id -> Int4,
        user_msg -> Text,
    }
}

table! {
    users (id) {
        id -> Int4,
        msg -> Text,
        date -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    reports,
    users,
);
