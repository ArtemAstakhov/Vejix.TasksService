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
        task_list_id -> Nullable<Int4>,
    }
}

table! {
    tasks_lists (id) {
        id -> Int4,
        name -> Text,
        user_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        experience -> Int4,
        challenge_active -> Nullable<Bool>,
        challenge_fails -> Nullable<Int4>,
        reset_confirm -> Nullable<Text>,
        role -> Nullable<Text>,
    }
}

joinable!(checklist_items -> checklists (checklist_id));
joinable!(checklists -> users (user_id));
joinable!(tasks -> tasks_lists (task_list_id));
joinable!(tasks -> users (user_id));
joinable!(tasks_lists -> users (user_id));

allow_tables_to_appear_in_same_query!(
    checklist_items,
    checklists,
    tasks,
    tasks_lists,
    users,
);
