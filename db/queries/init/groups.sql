-- Get admin ID
WITH admin_id AS (
    SELECT id FROM accounts WHERE name = 'admin'
)
-- Add top level group containing all accounts
INSERT INTO "groups" ("managed_by", "name", "display_name", "description", "parent_id", "left", "right")
SELECT
    admin_id.id,
    'all_accounts',
    'All accounts',
    'Top-level group containing all accounts',
    NULL,
    1,
    2
FROM admin_id
ON CONFLICT ("name") DO NOTHING;