// @generated automatically by Diesel CLI.

diesel::table! {
    friends (id) {
        id -> Int4,
        user1_id -> Int4,
        user2_id -> Int4,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        tel -> Varchar,
        rib -> Varchar,
        email_paypal -> Varchar,
        tel_wero -> Varchar,
        profil_pict_ref -> Varchar,
        password -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    friends,
    users,
);
