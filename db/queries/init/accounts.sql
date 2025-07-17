-- Add admin account if it does exist yet
INSERT INTO "accounts" ("name", "type")
VALUES ('admin', 'service')
ON CONFLICT ("name") DO NOTHING;