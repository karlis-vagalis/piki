CREATE TYPE "account_type" AS ENUM (
	'service',
	'person'
);

CREATE TABLE "accounts" (
	"id" BIGSERIAL NOT NULL UNIQUE,
	"type" ACCOUNT_TYPE NOT NULL,
	"name" TEXT NOT NULL,
	"created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	"updated_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	"deleted_at" TIMESTAMPTZ,
	PRIMARY KEY("id")
);