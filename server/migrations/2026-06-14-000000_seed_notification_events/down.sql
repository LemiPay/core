-- Remove seeded events (only if no preferences reference them; safe for dev)
DELETE FROM notification_event
WHERE name IN (
    'proposal_created',
    'proposal_approved',
    'proposal_rejected',
    'proposal_executed',
    'investment_created',
    'fund_round_created',
    'new_member_added',
    'expense_created'
);
