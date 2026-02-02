# API Endpoints Overview

This document summarizes the HTTP endpoints exposed by `dbwebconnect`, their purpose, authentication and expected behavior.

All examples assume the server is running on `http://localhost:8080`.

- Authentication: JWT via `Authorization: Bearer <token>` for most routes
- Logical databases: implemented as Postgres schemas (e.g. `my_schema`)
- Legacy API-key path parameters exist for backward compatibility but are ignored

---

## Auth & Administration

### `POST /auth/register`
- **Auth**: none
- **Body (JSON)**:
  - `email: string`
  - `password: string`
- **Behavior**:
  - Creates a new user in `"Auth".users` with an Argon2-hashed password.
  - First user becomes `admin`, subsequent users default to `user`.
- **Response**:
  - `200 OK` with JSON containing a message / basic user info.

**Example**

```bash
curl -X POST \
  http://localhost:8080/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email":"admin@test.local","password":"Password123!"}'
```

### `POST /auth/login`
- **Auth**: none
- **Body (JSON)**:
  - `email: string`
  - `password: string`
- **Behavior**:
  - Verifies credentials and issues a JWT (HS256) if valid.
- **Response**:
  - `200 OK` with `{ "token": "<jwt>" }`.

**Example**

```bash
curl -X POST \
  http://localhost:8080/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"admin@test.local","password":"Password123!"}'
```

### `GET /admin/users`
- **Auth**: `admin` role required
- **Behavior**:
  - Lists registered users and their roles/allowed_schemas.
- **Response**:
  - `200 OK` JSON array of user objects.

**Example**

```bash
curl -X GET \
  http://localhost:8080/admin/users \
  -H "Authorization: Bearer $TOKEN"
```

### `POST /admin/users/schemas`
- **Auth**: `admin` role required
- **Body (JSON)**:
  - typically `{ "email": "...", "allowed_schemas": "schema1,schema2" }`
- **Behavior**:
  - Updates which logical schemas a user may access.
- **Response**:
  - `200 OK` on success.

**Example**

```bash
curl -X POST \
  http://localhost:8080/admin/users/schemas \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"email":"user@test.local","allowed_schemas":"my_schema,other_schema"}'
```

### `GET /health`
- **Auth**: none
- **Behavior**:
  - Checks `DATABASE_URL` and attempts to open a Postgres connection.
- **Response**:
  - `200 OK` with `{ "status": "ok", ... }` on success.
  - `500` JSON with error info if DB or env is misconfigured.

**Example**

```bash
curl -X GET http://localhost:8080/health
```

---

## Initialization & Legacy

### `GET /`
- **Auth**: none
- **Behavior**:
  - Returns an HTML page from `initconnect::getpagehtml()` (legacy UI entrypoint).
- **Response**:
  - `200 OK` HTML.

**Example**

```bash
curl -X GET http://localhost:8080/
```

### `POST /`
- **Auth**: none
- **Behavior**:
  - Legacy API-key based initializer, now deprecated.
- **Response**:
  - `410 Gone` with `{ "error": "deprecated_endpoint_use_jwt_auth" }`.

**Example**

```bash
curl -X POST http://localhost:8080/ -d 'deprecated'
```

### `GET /getkey/{database}&apikey={apikey}`
- **Auth**: none
- **Behavior**:
  - Legacy API-key issuance, now deprecated in favor of JWT.
- **Response**:
  - `410 Gone` with `{ "error": "deprecated_endpoint_use_jwt_auth" }`.

**Example**

```bash
curl -X GET "http://localhost:8080/getkey/my_schema&apikey=ignored"
```

---

## Logical Database (Schema) Management

### `POST /createdatabase/{database}&apikey={apikey}`
- **Auth**: `admin` role required
- **Path Params**:
  - `database`: logical database name (Postgres schema)
  - `apikey`: ignored
- **Behavior**:
  - Executes `CREATE SCHEMA IF NOT EXISTS <database>`.
- **Response**:
  - `200 OK` with a base64-encoded string of `"ok"` (for legacy compatibility).

**Example**

```bash
curl -X POST \
  "http://localhost:8080/createdatabase/my_schema&apikey=ignored" \
  -H "Authorization: Bearer $TOKEN"
```

---

## Table DDL

### `POST /createtable/{database}&table={table}&gps={gps}&apikey={apikey}`
- **Auth**: `admin` role required
- **Path Params**:
  - `database`: schema name
  - `table`: table name
  - `gps`: `true`/`false` – whether to also create a `{table}_GPS` attachment table
  - `apikey`: ignored
- **Body (JSON)**:
  - `columns`: stringified JSON list of `{ "name:<colname>" }`
  - `types`: stringified JSON list of `{ "type:<SQL_TYPE>" }`
- **Behavior**:
  - Creates `database.table` with an `INTERNAL_PRIMARY_KEY SERIAL PRIMARY KEY` and given columns.
  - Adds defaults for certain types (e.g. `VARCHAR` → `DEFAULT ''`, `INT` → `DEFAULT 0`).
  - If `gps=true`, also creates `database.table_GPS` with GPS/attachment columns.
- **Response**:
  - `200 OK` with `"Table Created"` on success.
  - `200 OK` with `"Invalid column name: ..."` if validation fails.

**Example**

```bash
curl -X POST \
  "http://localhost:8080/createtable/my_schema&table=my_table&gps=false&apikey=ignored" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "columns": "[{\"name:id\"},{\"name:name\"}]",
    "types":   "[{\"type:INT\"},{\"type:VARCHAR(255)\"}]"
  }'
```

### `POST /droptable/{database}&table={table}&apikey={apikey}`
- **Auth**: `admin` role required
- **Path Params**:
  - `database`: schema name
  - `table`: table name
  - `apikey`: ignored
- **Body (JSON)**:
  - A single boolean field, e.g. `{ "backup": true }`.
- **Behavior**:
  - If `backup=true`, generates and executes a backup statement.
  - Drops `database.table` via `DROP TABLE`.
- **Response**:
  - `200 OK` with `"Table Dropped"`.

**Example**

```bash
curl -X POST \
  "http://localhost:8080/droptable/my_schema&table=my_table&apikey=ignored" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"backup": false}'
```

---

## Insert / Update / Delete

### `POST /insert/{database}&table={table}&apikey={api}`
- **Auth**: user must be allowed to access `database`
- **Path Params**:
  - `database`: schema name
  - `table`: table name
  - `api`: ignored
- **Body (JSON)**:
  - Array of row objects, e.g. `[ { "col1": 1, "col2": "x" }, ... ]`
- **Behavior**:
  - Validates that keys match table columns (excluding internal columns).
  - Builds a multi-row `INSERT` with all values sent as quoted string literals (Postgres casts them).
- **Response**:
  - `200 OK` with `"Insert Successful"` on success.
  - `200 OK` with `"Invalid Data"` if column names/lengths dont match.

**Example**

```bash
curl -X POST \
  "http://localhost:8080/insert/my_schema&table=my_table&apikey=ignored" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '[{"id":1,"name":"Alice"},{"id":2,"name":"Bob"}]'
```

### `POST /updaterecord/{database}&table={table}&apikey={api}`
- **Auth**: user must be allowed to access `database`
- **Path Params**:
  - `database`: schema name
  - `table`: table name
  - `api`: ignored
- **Body (JSON)**:
  - Array of objects including `INTERNAL_PRIMARY_KEY` and any fields to change, e.g.:
    - `[ { "INTERNAL_PRIMARY_KEY": 1, "name": "new" }, ... ]`
- **Behavior**:
  - For each object, builds an `UPDATE` for that primary key.
  - All values are stringified and quoted; Postgres casts appropriately.
- **Response**:
  - `200 OK` with `"Update Successful"`.

**Example**

```bash
curl -X POST \
  "http://localhost:8080/updaterecord/my_schema&table=my_table&apikey=ignored" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '[{"INTERNAL_PRIMARY_KEY":1,"name":"Alice Updated"}]'
```

### `POST /deleterecord/{database}&table={table}&apikey={api}`
- **Auth**: user must be allowed to access `database`
- **Path Params**:
  - `database`: schema name
  - `table`: table name
  - `api`: ignored
- **Body (JSON)**:
  - Object whose values are the `INTERNAL_PRIMARY_KEY` values to delete, e.g. `{ "id1": 1, "id2": 2 }`.
- **Behavior**:
  - Builds `DELETE FROM database.table WHERE INTERNAL_PRIMARY_KEY IN (...)`.
- **Response**:
  - `200 OK` with `"Status: 200 Record Deleted"`.

**Example**

```bash
curl -X POST \
  "http://localhost:8080/deleterecord/my_schema&table=my_table&apikey=ignored" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"id1":1,"id2":2}'
```

---

## Attachments (GPS side table)

### `POST /insertattachment/{database}&table={table}&apikey={api}`
- **Auth**: user must be allowed to access `database`
- **Path Params**:
  - `database`: schema name
  - `table`: base table name; side table is `{table}_GPS`
  - `api`: ignored
- **Body (JSON)**:
  - Expected order: first key is filename, second key is attachment data.
  - Example: `{ "filename": "file1.txt", "attachment": "<base64>" }`.
- **Behavior**:
  - Decodes base64 data, hex-encodes bytes, inserts into `database.table_GPS (X_COORD, ATTACHMENT)`.
- **Response**:
  - `200 OK` with `"Status: 200 Record Inserted"`.

**Example**

```bash
ATTACH_BASE64=$(printf 'hello world' | base64)

curl -X POST \
  "http://localhost:8080/insertattachment/my_schema&table=my_table&apikey=ignored" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d "{\"filename\":\"file1.txt\",\"attachment\":\"${ATTACH_BASE64}\"}"
```

### `GET /retrieveattachment/{database}&table={table}&id={id}&apikey={api}`
- **Auth**: user must be allowed to access `database`
- **Path Params**:
  - `database`: schema name
  - `table`: base table name
  - `id`: `INTERNAL_PRIMARY_KEY` in `{table}_GPS`
  - `api`: ignored
- **Behavior**:
  - Selects the `Attachment` column from `database.table_GPS` for the given id.
  - Returns the raw bytes base64-encoded as `result`.
- **Response**:
  - `200 OK` with `{ "result": "<base64>" }`.

**Example**

```bash
curl -X GET \
  "http://localhost:8080/retrieveattachment/my_schema&table=my_table&id=1&apikey=ignored" \
  -H "Authorization: Bearer $TOKEN"
```

---

## Querying Data

### `GET /query/{database}&table={table}&select={select}&where={where}&expand={expand}&apikey={api}`
- **Auth**: user must be allowed to access `database`
- **Path Params**:
  - `database`: schema name
  - `table`: table name
  - `select`: comma-separated list of columns or `*`
  - `where`: SQL where clause (URL-encoded), e.g. `INTERNAL_PRIMARY_KEY=1`
  - `expand`: `true` or `false` – whether to expand all columns
  - `api`: ignored
- **Behavior**:
  - Runs per-column SELECTs (casting to text) and builds a row-wise JSON object.
- **Response**:
  - `200 OK` with JSON object keyed by row index (`"0"`, `"1"`, ...).

**Example**

```bash
curl -X GET \
  "http://localhost:8080/query/my_schema&table=my_table&select=*&where=INTERNAL_PRIMARY_KEY%3D1&expand=false&apikey=ignored" \
  -H "Authorization: Bearer $TOKEN"
```

### `GET /queryall/{database}&table={table}&depth={depth}&apikey={api}`
- **Auth**: user must be allowed to access `database`
- **Path Params**:
  - `database`: schema name
  - `table`: root table name
  - `depth`: recursion depth (integer)
  - `api`: ignored
- **Behavior**:
  - Reads relationships from `"Relationships".relationships` where `table` is a parent.
  - Recursively walks child tables up to `depth`, nesting related records.
- **Response**:
  - `200 OK` with nested JSON of parents and children (and deeper descendants if `depth > 1`).

**Example**

```bash
curl -X GET \
  "http://localhost:8080/queryall/my_schema&table=parents&depth=1&apikey=ignored" \
  -H "Authorization: Bearer $TOKEN"
```

### `GET /queryrelationship/{database}&relationship={relationship}&apikey={api}`
- **Auth**: user must be allowed to access `database`
- **Path Params**:
  - `database`: target schema
  - `relationship`: logical relationship name
  - `api`: ignored
- **Behavior**:
  - Loads a single relationship definition from `"Relationships".relationships`.
  - Queries the parent table, then nests matching child rows using the stored `where_clause`.
- **Response**:
  - `200 OK` with JSON of parent rows including nested child data under the child column name.

**Example**

```bash
curl -X GET \
  "http://localhost:8080/queryrelationship/my_schema&relationship=parents_children&apikey=ignored" \
  -H "Authorization: Bearer $TOKEN"
```

### `GET /querytableschema/{database}&table={table}&apikey={api}`
- **Auth**: user must be allowed to access `database`
- **Path Params**:
  - `database`: schema name
  - `table`: table name
  - `api`: ignored
- **Behavior**:
  - Queries `information_schema.columns` and `information_schema.key_column_usage`.
  - Builds JSON for each column with type and constraint info.
- **Response**:
  - `200 OK` with objects like `{ "column_name", "column_type", "constraint_name" }` keyed by index.

**Example**

```bash
curl -X GET \
  "http://localhost:8080/querytableschema/my_schema&table=my_table&apikey=ignored" \
  -H "Authorization: Bearer $TOKEN"
```

### `GET /querydatabase/{database}&expand={expand}&apikey={api}`
- **Auth**: `admin` role required
- **Path Params**:
  - `database`: schema name
  - `expand`: `true` or `false`
  - `api`: ignored
- **Behavior**:
  - If `expand=false`, returns table names and schema.
  - If `expand=true`, for each table returns column names, types, and constraints.
- **Response**:
  - `200 OK` with JSON description of tables and, optionally, their schemas.

**Example**

```bash
curl -X GET \
  "http://localhost:8080/querydatabase/my_schema&expand=false&apikey=ignored" \
  -H "Authorization: Bearer $TOKEN"
```

---

## Relationships Metadata

### `POST /relationship/{database}&apikey={api}`
- **Auth**: `admin` role required
- **Path Params**:
  - `database`: schema whose tables are being related
  - `api`: ignored
- **Body (JSON)**:
  - Keys:
    - `table1`, `table1col`, `table2`, `table2col`, `ondelete`, `onupdate`
- **Behavior**:
  - Builds an `ALTER TABLE <database>.<table1> ADD FOREIGN KEY ...` statement.
  - Executes it via Diesel.
- **Response**:
  - `200 OK` with `"Status: 200 Relationship Created"` on success.

**Example**

```bash
curl -X POST \
  "http://localhost:8080/relationship/my_schema&apikey=ignored" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "table1":   "parents",
    "table1col":"id",
    "table2":   "children",
    "table2col":"parent_id",
    "ondelete": "CASCADE",
    "onupdate": "CASCADE"
  }'
```

### `POST /relateparent/{database}&parent_table={parent_table}&child_table={child_table}&relationship_name={relationship_name}&apikey={api}`
- **Auth**: `admin` role required
- **Path Params**:
  - `database`: schema name
  - `parent_table`: parent table
  - `child_table`: child table
  - `relationship_name`: unique logical name
  - `api`: ignored
- **Body (JSON)**:
  - Typically `{ "where": "<parent_col>=<child_col>" }`, after parsing.
- **Behavior**:
  - Ensures `relationship_name` is unique in `"Relationships".relationships`.
  - Inserts a row describing the relationship (target DB, tables, where clause, name).
- **Response**:
  - `200 OK` with `"Status: 200 Relationship Created"` on success.
  - `200 OK` with `"Status: 400 Relationship Name Already Exists"` if duplicate.

**Example**

```bash
curl -X POST \
  "http://localhost:8080/relateparent/my_schema&parent_table=parents&child_table=children&relationship_name=parents_children&apikey=ignored" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"where":"parents.INTERNAL_PRIMARY_KEY=children.parent_id"}'
```
