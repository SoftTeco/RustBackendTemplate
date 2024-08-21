// @generated automatically by Diesel CLI.

diesel::table! {
    companies (id) {
        id -> Int4,
        #[max_length = 64]
        name -> Varchar,
        #[max_length = 64]
        email -> Nullable<Varchar>,
        #[max_length = 64]
        website -> Nullable<Varchar>,
        #[max_length = 255]
        address -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

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
    user_company_roles (id) {
        id -> Int4,
        user_id -> Int4,
        company_id -> Int4,
        role_id -> Int4,
        created_at -> Nullable<Timestamp>,
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
        updated_at -> Timestamp,
        #[max_length = 24]
        user_type -> Varchar,
    }
}

diesel::joinable!(user_company_roles -> companies (company_id));
diesel::joinable!(user_company_roles -> roles (role_id));
diesel::joinable!(user_company_roles -> users (user_id));
diesel::joinable!(user_roles -> roles (role_id));
diesel::joinable!(user_roles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    companies,
    roles,
    user_company_roles,
    user_roles,
    users,
);
