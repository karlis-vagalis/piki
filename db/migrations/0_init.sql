-- Add admin account if it does exist yet
INSERT INTO "accounts" ("id", "name", "type")
VALUES ('00000000-0000-7000-8000-000000000000', 'admin', 'service')
ON CONFLICT ("id") DO NOTHING;

INSERT INTO "groups" ("managed_by", "id", "name", "description", "parent_id", "left", "right")
VALUES (
    '00000000-0000-7000-8000-000000000000',
    '00000000-0000-7000-8000-000000000001',
    'All accounts',
    'Top-level group containing all accounts',
    NULL,
    1,
    2
)
ON CONFLICT ("id") DO NOTHING;