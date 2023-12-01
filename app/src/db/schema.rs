// @generated automatically by Diesel CLI.

diesel::table! {
    puzzle_set (id) {
        id -> Integer,
        #[max_length = 255]
        name -> Varchar,
        description -> Text,
        created -> Datetime,
        modified -> Datetime,
    }
}

diesel::table! {
    puzzle_set_refs (id) {
        id -> Integer,
        puzzle_set_id -> Integer,
        puzzle_id -> Integer,
    }
}

diesel::table! {
    puzzles (id) {
        id -> Integer,
        #[max_length = 255]
        name -> Varchar,
        description -> Text,
        created -> Datetime,
        modified -> Datetime,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 255]
        salt -> Varchar,
        admin -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    puzzle_set,
    puzzle_set_refs,
    puzzles,
    users,
);
