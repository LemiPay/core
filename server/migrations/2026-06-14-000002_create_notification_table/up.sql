CREATE TABLE notification (
    id         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id    UUID NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    group_id   UUID REFERENCES "group"(id) ON DELETE CASCADE,
    event_name TEXT NOT NULL,
    group_name TEXT,
    read       BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_notification_user_created ON notification(user_id, created_at DESC);
