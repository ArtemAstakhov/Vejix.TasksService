table! {
    tasks (id) {
        id -> Int4,
        date -> Timestamp,
        title -> Text,
        completed -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deadline -> Nullable<Timestamp>,
        tag -> Nullable<Text>,
        user_id -> Int4,
        order -> Int4,
    }
}
