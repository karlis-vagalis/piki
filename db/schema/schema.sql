-- https://www.ackee.agency/blog/hierarchical-models-in-postgresql

CREATE TYPE "account_type" AS ENUM (
	'service',
	'person'
);

CREATE TABLE "accounts" (
	"id" BIGSERIAL NOT NULL UNIQUE,
	"name" TEXT NOT NULL UNIQUE,
	"type" ACCOUNT_TYPE NOT NULL,
	"created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	"updated_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	"deleted_at" TIMESTAMPTZ,
	PRIMARY KEY("id")
);




CREATE TABLE "groups" (
	"id" BIGSERIAL NOT NULL UNIQUE,
	"managed_by" BIGINT NOT NULL,
	"name" TEXT NOT NULL UNIQUE,
	"display_name" TEXT NOT NULL,
	"description" TEXT,
	"created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	"updated_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	"parent_id" BIGINT,
	"left" BIGINT NOT NULL,
	"right" BIGINT NOT NULL,
	PRIMARY KEY("id")
);



ALTER TABLE "groups"
ADD FOREIGN KEY("managed_by") REFERENCES "accounts"("id")
ON UPDATE NO ACTION ON DELETE NO ACTION;
ALTER TABLE "groups"
ADD FOREIGN KEY("parent_id") REFERENCES "groups"("id")
ON UPDATE NO ACTION ON DELETE NO ACTION;