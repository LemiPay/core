-- Seed notification events (the ones previously commented + a couple practical ones)
INSERT INTO notification_event (name)
VALUES ('proposal_created'),
--     ('proposal_approved'),
--     ('proposal_rejected'),
       ('proposal_executed'),
       ('investment_created'),
       ('investment_matured'),
       ('fund_round_created'),
       ('new_member_added'),
       ('expense_created')
ON CONFLICT (name) DO NOTHING;
