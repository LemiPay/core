INSERT INTO notification_event (name)
VALUES ('investment_matured')
ON CONFLICT (name) DO NOTHING;

-- Backfill preferences so existing users can configure the new event like the others.
INSERT INTO user_notification_preference (user_id, event_id, channel_id, enabled)
SELECT u.id, e.id, c.id, TRUE
FROM "user" u
         CROSS JOIN notification_event e
         CROSS JOIN notification_channel c
WHERE e.name = 'investment_matured'
ON CONFLICT (user_id, event_id, channel_id) DO NOTHING;

INSERT INTO group_notification_preference (user_id, group_id, event_id, channel_id, enabled)
SELECT uig.user_id, uig.group_id, e.id, c.id, TRUE
FROM user_in_group uig
         CROSS JOIN notification_event e
         CROSS JOIN notification_channel c
WHERE e.name = 'investment_matured'
  AND uig.status = 'active'
ON CONFLICT (user_id, group_id, event_id, channel_id) DO NOTHING;