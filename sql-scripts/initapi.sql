-- Initialize core schemas and tables for Postgres

-- In Postgres we treat logical databases as schemas. The main
-- database is created by POSTGRES_DB, so we only need schemas
-- and tables here.

-- Schema and table for API keys
CREATE SCHEMA IF NOT EXISTS "ApiKey";

CREATE TABLE IF NOT EXISTS "ApiKey".apikeys (
	"INTERNAL_PRIMARY_KEY" SERIAL PRIMARY KEY,
	databaseuser        VARCHAR(100),
	databasepasshash    VARCHAR(100),
	apikey              VARCHAR(255)
);

-- Schema and table for relationships metadata
CREATE SCHEMA IF NOT EXISTS "Relationships";

CREATE TABLE IF NOT EXISTS "Relationships".relationships (
	"INTERNAL_PRIMARY_KEY" SERIAL PRIMARY KEY,
	targeted_database VARCHAR(100),
	parent_table      VARCHAR(100),
	child_table       VARCHAR(100),
	where_clause      VARCHAR(255),
	relationship      VARCHAR(100) UNIQUE
);

-- Schema and table for user accounts (JWT auth)
CREATE SCHEMA IF NOT EXISTS "Auth";

CREATE TABLE IF NOT EXISTS "Auth".users (
	id              SERIAL PRIMARY KEY,
	email           VARCHAR(255) UNIQUE NOT NULL,
	password_hash   VARCHAR(255)        NOT NULL,
	role            VARCHAR(50)         NOT NULL DEFAULT 'user',
	allowed_schemas TEXT                NOT NULL DEFAULT ''
);

