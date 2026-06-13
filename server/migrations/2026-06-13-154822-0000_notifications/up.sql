-- Your SQL goes here
-- =========================
-- NOTIFICATION EVENTS
-- =========================

CREATE TABLE notification_event (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL UNIQUE
);

-- =========================
-- NOTIFICATION CHANNELS
-- =========================

CREATE TABLE notification_channel (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL UNIQUE
);

-- =========================
-- USER NOTIFICATION PREFERENCES
-- =========================

CREATE TABLE user_notification_preference (
    user_id UUID NOT NULL,
    event_id UUID NOT NULL,
    channel_id UUID NOT NULL,

    enabled BOOLEAN NOT NULL DEFAULT TRUE,

    PRIMARY KEY (user_id, event_id, channel_id),

    FOREIGN KEY (user_id)
        REFERENCES "user"(id)
        ON DELETE CASCADE,

    FOREIGN KEY (event_id)
        REFERENCES notification_event(id)
        ON DELETE CASCADE,

    FOREIGN KEY (channel_id)
        REFERENCES notification_channel(id)
        ON DELETE CASCADE
);

-- =========================
-- GROUP NOTIFICATION PREFERENCES
-- =========================

CREATE TABLE group_notification_preference (
    user_id UUID NOT NULL,
    group_id UUID NOT NULL,
    event_id UUID NOT NULL,
    channel_id UUID NOT NULL,

    enabled BOOLEAN NOT NULL DEFAULT TRUE,

    PRIMARY KEY (user_id, group_id, event_id, channel_id),

    FOREIGN KEY (user_id)
        REFERENCES "user"(id)
        ON DELETE CASCADE,

    FOREIGN KEY (group_id)
        REFERENCES "group"(id)
        ON DELETE CASCADE,

    FOREIGN KEY (event_id)
        REFERENCES notification_event(id)
        ON DELETE CASCADE,

    FOREIGN KEY (channel_id)
        REFERENCES notification_channel(id)
        ON DELETE CASCADE
);

-- Channels
INSERT INTO notification_channel (name)
VALUES
    ('email'),
    ('web');

-- Events
-- INSERT INTO notification_event (name)
-- VALUES
--     ('proposal_created'),
--     ('proposal_approved'),
--     ('proposal_rejected'),
--     ('proposal_executed'),
--     ('investment_created'),
--     ('fund_round_created'),
--     ('new_member_added');