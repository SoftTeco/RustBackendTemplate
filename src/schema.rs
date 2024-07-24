// @generated automatically by Diesel CLI.

diesel::table! {
    roles (id) {
        id -> Int4,
        #[max_length = 64]
        code -> Varchar,
        #[max_length = 128]
        name -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    user_roles (id) {
        id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 64]
        username -> Varchar,
        #[max_length = 64]
        email -> Varchar,
        #[max_length = 128]
        password -> Varchar,
        #[max_length = 64]
        first_name -> Nullable<Varchar>,
        #[max_length = 64]
        last_name -> Nullable<Varchar>,
        #[max_length = 64]
        country -> Nullable<Varchar>,
        birth_date -> Nullable<Date>,
        created_at -> Timestamp,
        confirmed -> Bool,
    }
}

diesel::joinable!(user_roles -> roles (role_id));
diesel::joinable!(user_roles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    roles,
    user_roles,
    users,
);
