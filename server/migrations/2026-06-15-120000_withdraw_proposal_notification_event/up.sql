INSERT INTO notification_event (name)
VALUES ('withdraw_proposal_created')
ON CONFLICT (name) DO NOTHING;

-- Migrate user preferences from proposal_created (keep enabled state).
INSERT INTO user_notification_preference (user_id, event_id, channel_id, enabled)
SELECT unp.user_id, new_e.id, unp.channel_id, unp.enabled
FROM user_notification_preference unp
         JOIN notification_event old_e ON old_e.id = unp.event_id AND old_e.name = 'proposal_created'
         JOIN notification_event new_e ON new_e.name = 'withdraw_proposal_created'
ON CONFLICT (user_id, event_id, channel_id) DO NOTHING;

-- Migrate group preferences from proposal_created.
INSERT INTO group_notification_preference (user_id, group_id, event_id, channel_id, enabled)
SELECT gnp.user_id, gnp.group_id, new_e.id, gnp.channel_id, gnp.enabled
FROM group_notification_preference gnp
         JOIN notification_event old_e ON old_e.id = gnp.event_id AND old_e.name = 'proposal_created'
         JOIN notification_event new_e ON new_e.name = 'withdraw_proposal_created'
ON CONFLICT (user_id, group_id, event_id, channel_id) DO NOTHING;

-- Backfill for users/groups that never had proposal_created prefs.
INSERT INTO user_notification_preference (user_id, event_id, channel_id, enabled)
SELECT u.id, e.id, c.id, TRUE
FROM "user" u
         CROSS JOIN notification_event e
         CROSS JOIN notification_channel c
WHERE e.name = 'withdraw_proposal_created'
ON CONFLICT (user_id, event_id, channel_id) DO NOTHING;

INSERT INTO group_notification_preference (user_id, group_id, event_id, channel_id, enabled)
SELECT uig.user_id, uig.group_id, e.id, c.id, TRUE
FROM user_in_group uig
         CROSS JOIN notification_event e
         CROSS JOIN notification_channel c
WHERE e.name = 'withdraw_proposal_created'
  AND uig.status = 'active'
ON CONFLICT (user_id, group_id, event_id, channel_id) DO NOTHING;

UPDATE notification
SET event_name = 'withdraw_proposal_created'
WHERE event_name = 'proposal_created';

DELETE
FROM group_notification_preference
WHERE event_id IN (SELECT id FROM notification_event WHERE name = 'proposal_created');

DELETE
FROM user_notification_preference
WHERE event_id IN (SELECT id FROM notification_event WHERE name = 'proposal_created');

DELETE FROM notification_event WHERE name = 'proposal_created';