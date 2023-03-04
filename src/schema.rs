// @generated automatically by Diesel CLI.

diesel::table! {
    app_user (id) {
        id -> Int4,
        email -> Varchar,
        username -> Varchar,
        secret -> Text,
        created -> Timestamp,
    }
}

diesel::table! {
    article (id) {
        id -> Int4,
        title -> Varchar,
        content -> Varchar,
        created -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    app_user,
    article,
);
