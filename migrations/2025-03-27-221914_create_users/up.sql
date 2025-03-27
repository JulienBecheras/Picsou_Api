-- Your SQL goes here
CREATE TABLE users
(
    id              SERIAL PRIMARY KEY,
    first_name      VARCHAR   NOT NULL,
    last_name       VARCHAR   NOT NULL,
    email           VARCHAR   NOT NULL UNIQUE,
    tel             VARCHAR   NOT NULL,
    rib             VARCHAR   NOT NULL,
    email_paypal    VARCHAR   NOT NULL,
    tel_wero        VARCHAR   NOT NULL,
    profil_pict_ref VARCHAR   NOT NULL,
    created_at      TIMESTAMP NOT NULL,
    updated_at      TIMESTAMP NOT NULL,
    password        VARCHAR   NOT NULL
);
