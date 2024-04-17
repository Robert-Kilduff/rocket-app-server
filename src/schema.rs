// @generated automatically by Diesel CLI.

diesel::table! {
    habits (id) {
        id -> Integer,
        user_id -> Integer,
        name -> Text,
        created_at -> Timestamp,
        max_progress_per_period -> Nullable<Integer>,
    }
}

diesel::table! {
    task_habit (task_id, habit_id) {
        task_id -> Integer,
        habit_id -> Integer,
        contribution -> Nullable<Integer>,
    }
}

diesel::table! {
    tasks (id) {
        id -> Integer,
        name -> Text,
        created_at -> Timestamp,
        completed_at -> Nullable<Timestamp>,
        is_completed -> Bool,
        complexity -> Integer,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
        role -> Nullable<Integer>,
        created_at -> Timestamp,
    }
}

diesel::joinable!(task_habit -> habits (habit_id));
diesel::joinable!(task_habit -> tasks (task_id));

diesel::allow_tables_to_appear_in_same_query!(
    habits,
    task_habit,
    tasks,
    users,
);
