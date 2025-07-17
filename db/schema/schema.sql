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




CREATE TABLE "groups" (
	"id" BIGSERIAL NOT NULL UNIQUE,
	"name" TEXT NOT NULL,
	"description" TEXT,
	"created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	"updated_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	"deleted_at" TIMESTAMPTZ,
	"parent_id" BIGINT,
	"lft" INTEGER NOT NULL,
	"rgt" INTEGER NOT NULL,
	PRIMARY KEY("id")
);




CREATE TABLE "group_memberships" (
	"id" BIGSERIAL NOT NULL UNIQUE,
	"group_id" BIGINT NOT NULL,
	"account_id" BIGINT NOT NULL,
	"joined_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	PRIMARY KEY("id")
);




CREATE TABLE "permissions" (
	"id" BIGSERIAL NOT NULL UNIQUE,
	"name" TEXT NOT NULL,
	PRIMARY KEY("id")
);




CREATE TABLE "group_permissions" (
	"id" BIGSERIAL NOT NULL UNIQUE,
	"group_id" BIGINT NOT NULL,
	"permission_id" BIGINT NOT NULL,
	PRIMARY KEY("id")
);



ALTER TABLE "group_memberships"
ADD FOREIGN KEY("group_id") REFERENCES "groups"("id")
ON UPDATE NO ACTION ON DELETE CASCADE;
ALTER TABLE "group_memberships"
ADD FOREIGN KEY("account_id") REFERENCES "accounts"("id")
ON UPDATE NO ACTION ON DELETE CASCADE;
ALTER TABLE "groups"
ADD FOREIGN KEY("parent_id") REFERENCES "groups"("id")
ON UPDATE NO ACTION ON DELETE CASCADE;
ALTER TABLE "group_permissions"
ADD FOREIGN KEY("group_id") REFERENCES "groups"("id")
ON UPDATE NO ACTION ON DELETE NO ACTION;
ALTER TABLE "group_permissions"
ADD FOREIGN KEY("permission_id") REFERENCES "permissions"("id")
ON UPDATE NO ACTION ON DELETE NO ACTION;