// @generated automatically by Diesel CLI.

diesel::table! {
    contributors (id) {
        id -> Int4,
        amount_contributed -> Float8,
        groups_users_id -> Int4,
        expenses_id -> Int4,
    }
}

diesel::table! {
    expenses (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        montant -> Float8,
        stock_parts -> Int4,
    }
}

diesel::table! {
    friend_requests (id) {
        id -> Int4,
        from_user_id -> Int4,
        to_user_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
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
        pict_ref -> Varchar,
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
        status -> Int4,
    }
}

diesel::table! {
    participants (id) {
        id -> Int4,
        amount_participated -> Float8,
        part_number -> Nullable<Int4>,
        expenses_id -> Int4,
        groups_users_id -> Int4,
    }
}

diesel::table! {
    refunds (id) {
        id -> Int4,
        amount -> Float8,
        status -> Varchar,
        contributors_id -> Int4,
        participants_id -> Int4,
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

diesel::joinable!(contributors -> expenses (expenses_id));
diesel::joinable!(contributors -> groups_users (groups_users_id));
diesel::joinable!(groups_users -> groups (id_group));
diesel::joinable!(groups_users -> users (id_user));
diesel::joinable!(participants -> expenses (expenses_id));
diesel::joinable!(participants -> groups_users (groups_users_id));
diesel::joinable!(refunds -> contributors (contributors_id));
diesel::joinable!(refunds -> participants (participants_id));

diesel::allow_tables_to_appear_in_same_query!(
    contributors,
    expenses,
    friend_requests,
    friends,
    groups,
    groups_users,
    participants,
    refunds,
    users,
);
