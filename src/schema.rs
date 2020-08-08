table! {
    checklist_items (id) {
        id -> Int4,
        name -> Text,
        completed -> Bool,
        checklist_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    checklists (id) {
        id -> Int4,
        name -> Text,
        user_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

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

allow_tables_to_appear_in_same_query!(
    checklist_items,
    checklists,
    tasks,
);
