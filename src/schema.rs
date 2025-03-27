// @generated automatically by Diesel CLI.

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
        created_at -> Timestamp,
        updated_at -> Timestamp,
        password -> Varchar,
    }
}
