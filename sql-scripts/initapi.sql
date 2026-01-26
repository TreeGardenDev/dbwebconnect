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

