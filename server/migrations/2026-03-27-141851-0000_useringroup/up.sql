-- Your SQL goes here
CREATE TYPE group_role AS ENUM ('admin', 'member');
CREATE TYPE group_member_status AS ENUM ('active', 'banned', 'left');

CREATE TABLE user_in_group (
    user_id UUID NOT NULL,
    group_id UUID NOT NULL,

    role group_role NOT NULL DEFAULT 'member',
    status group_member_status NOT NULL DEFAULT 'active',

    joined_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    PRIMARY KEY (user_id, group_id),

    CONSTRAINT fk_user
       FOREIGN KEY (user_id)
           REFERENCES "user"(id)
           ON DELETE CASCADE,

    CONSTRAINT fk_group
       FOREIGN KEY (group_id)
           REFERENCES "group"(id)
           ON DELETE CASCADE
);