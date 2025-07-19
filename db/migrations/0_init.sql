-- Add admin account if it does exist yet
INSERT INTO "accounts" ("id", "name", "type")
VALUES ('00000000-0000-7000-8000-830000000000', 'admin', 'service')
ON CONFLICT ("id") DO NOTHING;