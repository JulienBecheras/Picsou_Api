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
    password        VARCHAR   NOT NULL,
    created_at      TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at      TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE OR REPLACE FUNCTION update_timestamp_user()
    RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_timestamp
    BEFORE UPDATE ON USERS
    FOR EACH ROW
EXECUTE FUNCTION update_timestamp_user();