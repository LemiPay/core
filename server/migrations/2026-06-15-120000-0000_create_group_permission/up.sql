CREATE TABLE group_permission (
    group_id UUID NOT NULL REFERENCES "group"(id) ON DELETE CASCADE,
    role group_role NOT NULL,
    action VARCHAR(50) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (group_id, role, action)
);
