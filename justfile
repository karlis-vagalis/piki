set dotenv-load := true

schema-inspect:
    atlas schema inspect --url $DATABASE_URL

schema-apply:
    atlas schema apply --url $DATABASE_URL --to file://db/schema/schema.sql --dev-url $DATABASE_URL

migrate-diff:
    atlas migrate diff --dir file://db/migrations --to file://db/schema/schema.sql --dev-url $DATABASE_URL

