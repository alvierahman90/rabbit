// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Int4,
        name -> Varchar,
        user_id -> Int4,
    }
}

diesel::table! {
    series (id) {
        id -> Int4,
        name -> Varchar,
        repeat -> Int4,
        good -> Bool,
        category_id -> Int4,
    }
}

diesel::table! {
    series_points (id) {
        id -> Int4,
        timestamp -> Timestamp,
        value -> Int4,
        series_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
    }
}

diesel::joinable!(categories -> users (user_id));
diesel::joinable!(series -> categories (category_id));
diesel::joinable!(series_points -> series (series_id));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    series,
    series_points,
    users,
);
