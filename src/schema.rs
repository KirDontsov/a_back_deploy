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