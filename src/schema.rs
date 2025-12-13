// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        name -> Nullable<Varchar>,
        email -> Varchar,
        password -> Varchar,
        role -> Nullable<Varchar>,
        photo -> Nullable<Varchar>,
        verified -> Nullable<Bool>,
        favourite -> Array<Nullable<Text>>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    avito_accounts (account_id) {
        account_id -> Uuid,
        user_id -> Uuid,
        client_id -> Varchar,
        avito_client_secret -> Text,
        avito_client_id -> Text,
        is_connected -> Nullable<Bool>,
        created_ts -> Timestamp,
        updated_ts -> Timestamp,
    }
}

diesel::joinable!(avito_accounts -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    users,
    avito_accounts,
);