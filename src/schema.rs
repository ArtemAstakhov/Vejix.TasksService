table! {
    achivement_categories (id) {
        id -> Int4,
        name -> Text,
        parent_category -> Nullable<Int4>,
        level -> Nullable<Int4>,
        user_id -> Int4,
    }
}

table! {
    achivements (id) {
        id -> Int4,
        name -> Text,
        points -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        category_id -> Int4,
        completed -> Bool,
        completed_at -> Nullable<Timestamp>,
        user_id -> Int4,
    }
}

table! {
    budget_accounts (id) {
        id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        user_id -> Int4,
        currency -> Nullable<Text>,
        icon -> Nullable<Text>,
        name -> Nullable<Text>,
        value -> Nullable<Float8>,
        active -> Bool,
        account_type -> Text,
        limit -> Nullable<Int4>,
        overdraft -> Bool,
        connected_debt_transaction_id -> Nullable<Int4>,
        debt_type -> Nullable<Text>,
    }
}

table! {
    budget_cost_categories (id) {
        id -> Int4,
        parent_category -> Nullable<Int4>,
        level -> Int4,
        name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        user_id -> Int4,
        active -> Bool,
        icon -> Nullable<Text>,
    }
}

table! {
    budget_debt_accounts (id) {
        id -> Int4,
        user_id -> Int4,
        name -> Text,
        value -> Float8,
        currency -> Text,
        connected_transaction_id -> Nullable<Int4>,
        debt_type -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    budget_income_categories (id) {
        id -> Int4,
        level -> Int4,
        name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        user_id -> Int4,
        parent_category -> Nullable<Int4>,
        active -> Bool,
        icon -> Nullable<Text>,
    }
}

table! {
    budget_transactions (id) {
        id -> Int4,
        date -> Timestamp,
        user_id -> Int4,
        account_id -> Int4,
        value -> Float8,
        transfer_account_id -> Nullable<Int4>,
        transfer_value -> Nullable<Float8>,
        category_id -> Nullable<Int4>,
        transaction_type -> Varchar,
        comment -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        debt_contragent -> Nullable<Varchar>,
        debt_type -> Nullable<Varchar>,
        debt_closed -> Nullable<Bool>,
        is_update -> Nullable<Bool>,
    }
}

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
    }
}

joinable!(achivement_categories -> users (user_id));
joinable!(achivements -> achivement_categories (category_id));
joinable!(achivements -> users (user_id));
joinable!(budget_accounts -> users (user_id));
joinable!(budget_cost_categories -> users (user_id));
joinable!(budget_debt_accounts -> users (user_id));
joinable!(budget_income_categories -> users (user_id));
joinable!(budget_transactions -> users (user_id));
joinable!(checklist_items -> checklists (checklist_id));
joinable!(checklists -> users (user_id));
joinable!(tasks -> users (user_id));

allow_tables_to_appear_in_same_query!(
    achivement_categories,
    achivements,
    budget_accounts,
    budget_cost_categories,
    budget_debt_accounts,
    budget_income_categories,
    budget_transactions,
    checklist_items,
    checklists,
    tasks,
    users,
);
