ALTER TABLE groups_users
    ADD COLUMN status INT CHECK (status IN (0, 1, 2, 3, 4, 5)) NOT NULL ,
    ADD CONSTRAINT fk_user FOREIGN KEY (id_user) REFERENCES users(id),
    ADD CONSTRAINT fk_group FOREIGN KEY (id_group) REFERENCES groups(id);