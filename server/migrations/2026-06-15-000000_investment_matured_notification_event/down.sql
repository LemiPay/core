DELETE
FROM group_notification_preference
WHERE event_id IN (SELECT id FROM notification_event WHERE name = 'investment_matured');

DELETE
FROM user_notification_preference
WHERE event_id IN (SELECT id FROM notification_event WHERE name = 'investment_matured');

DELETE FROM notification_event WHERE name = 'investment_matured';