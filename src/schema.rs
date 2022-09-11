// @generated automatically by Diesel CLI.

diesel::table! {
    content_links (id) {
        id -> Int4,
        todo_id -> Nullable<Int4>,
        link -> Nullable<Varchar>,
    }
}

diesel::table! {
    todos (id) {
        id -> Int4,
        timestamp -> Timestamp,
        end_at -> Timestamp,
        importance -> Int2,
        status -> Int2,
        text -> Varchar,
    }
}

diesel::joinable!(content_links -> todos (todo_id));

diesel::allow_tables_to_appear_in_same_query!(
    content_links,
    todos,
);
