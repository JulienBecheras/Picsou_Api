// @generated automatically by Diesel CLI.

diesel::table! {
    contributors (id) {
        id -> Int4,
        id_user -> Nullable<Int4>,
        id_payment -> Nullable<Int4>,
        amount -> Float8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    expenses (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    expenses_evo (id) {
        id -> Int4,
        name -> Varchar,
        share_number -> Int4,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    friends (id) {
        id -> Int4,
        user1_id -> Int4,
        user2_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    groups (id) {
        id -> Int4,
        name -> Varchar,
        pict_ref -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    groups_users (id) {
        id -> Int4,
        id_user -> Int4,
        id_group -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    participants (id) {
        id -> Int4,
        id_user -> Nullable<Int4>,
        id_payment -> Nullable<Int4>,
        amount -> Float8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    payments (id) {
        id -> Int4,
        amount -> Float8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    refunds (id) {
        id -> Int4,
        status -> Text,
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

diesel::joinable!(contributors -> payments (id_payment));
diesel::joinable!(contributors -> users (id_user));
diesel::joinable!(expenses -> payments (id));
diesel::joinable!(expenses_evo -> payments (id));
diesel::joinable!(participants -> payments (id_payment));
diesel::joinable!(participants -> users (id_user));
diesel::joinable!(refunds -> payments (id));

diesel::allow_tables_to_appear_in_same_query!(
    contributors,
    expenses,
    expenses_evo,
    friends,
    groups,
    groups_users,
    participants,
    payments,
    refunds,
    users,
);
