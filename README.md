# dbwebconnect

This project provides a web interface and tooling for working with databases. It is now configured to use Diesel with PostgreSQL.

## Requirements

- Rust toolchain (e.g. via `rustup`)
- Docker and Docker Compose

## First-Time Setup

1. **Build the Rust application (optional sanity check)**

   ```bash
   cargo build
   ```

2. **Start PostgreSQL and initialize schemas/tables**

   From the project root:

   ```bash
   docker compose up -d db
   ```

   On first startup, the `db` service will:

   - Create the `api_main` database (via `POSTGRES_DB`).
   - Run all SQL files in `sql-scripts` inside `/docker-entrypoint-initdb.d`.
   - Execute `sql-scripts/initapi.sql` to create the following in `api_main`:
     - Schema `"ApiKey"` with table `apikeys`.
     - Schema `"Relationships"` with table `relationships`.

3. **Verify the database (optional)**

   ```bash
   docker logs hosted_database
   ```

   If you have `psql` available:

   ```bash
   docker exec -it hosted_database psql -U api_user -d api_main
   ```

   Inside `psql` you can run, for example:

   ```sql
   \dn
   \dt "ApiKey".*
   \dt "Relationships".*
   ```

4. **Start the application container**

   From the project root:

   ```bash
   docker compose up -d app
   ```

   The `app` service is configured with:

   - `DATABASE_URL=postgres://api_user:secret@db:5432/api_main`

   and will connect to the `db` service using Diesel.

5. **Access the application**

   Once the `app` container is running, the HTTP server listens on port `8080`:

   - http://localhost:8080/

   (See `src/main.rs` for the available routes.)

## Subsequent Runs

For later sessions (after the first initialization):

- Start both services:

  ```bash
  docker compose up -d
  ```

- Stop everything:

  ```bash
  docker compose down
  ```

Because the Postgres data directory is mounted from `./data`, your schemas, tables, and data are preserved across container restarts.

## Local Development (without Docker app container)

If you prefer to run the Rust binary directly on your host while still using the Dockerized Postgres:

1. Ensure the `db` container is running:

   ```bash
   docker compose up -d db
   ```

2. Export a matching `DATABASE_URL` that points to `localhost` instead of `db`:

   ```bash
   export DATABASE_URL=postgres://api_user:secret@localhost:5432/api_main
   ```

3. Run the application:

   ```bash
   cargo run
   ```

The server will again be available on http://localhost:8080/.

## JWT Authentication & Authorization

The application now uses JSON Web Tokens (JWT) and a Postgres-backed users table instead of API keys for most database actions.

### Secrets

- The signing key is read from the `JWT_SECRET` environment variable.
- In Docker, this is set for the `app` service in [docker-compose.yml](docker-compose.yml).
- For local development, export it before running:

   ```bash
   export JWT_SECRET=change-me-in-prod
   ```

### User Registration

- Endpoint: `POST /auth/register`
- Body (JSON):

   ```json
   {
      "email": "user@example.com",
      "password": "at_least_8_chars",
      "allowed_schemas": ["my_database_schema"]
   }
   ```

- Behavior:
   - The **first** user ever registered becomes an `admin`.
   - All subsequent users are regular `user` accounts.
   - `allowed_schemas` is optional; if omitted or empty, the user is allowed to access **all** schemas.

### Login and Tokens

- Endpoint: `POST /auth/login`
- Body (JSON):

   ```json
   {
      "email": "user@example.com",
      "password": "at_least_8_chars"
   }
   ```

- On success, you receive:

   ```json
   {
      "status": "ok",
      "token": "<JWT_TOKEN>"
   }
   ```

- Use this token in the `Authorization` header for all protected endpoints:

   ```http
   Authorization: Bearer <JWT_TOKEN>
   ```

### Admin Endpoints

The first registered user (role `admin`) can manage other users:

- List users:
   - `GET /admin/users`
   - Requires an `Authorization: Bearer <admin token>` header.
- Update which schemas a user can access:
   - `POST /admin/users/schemas`
   - Body (JSON):

      ```json
      {
         "email": "user@example.com",
         "allowed_schemas": ["schema1", "schema2"]
      }
      ```

## Auth Behavior for Database Actions

Most existing routes still keep their original URL shape (including `&apikey=...` in the path), but **the API key is now ignored**. Authorization is based solely on the JWT token and the user’s role/schemas.

### Per-schema user actions

These endpoints require a valid JWT and that the user is allowed to access the `{database}` schema (either explicitly via `allowed_schemas` or implicitly when the list is empty):

- Insert records:
   - `POST /insert/{database}&table={table}&apikey={ignored}`
- Insert attachments:
   - `POST /insertattachment/{database}&table={table}&apikey={ignored}`
- Update records:
   - `POST /updaterecord/{database}&table={table}&apikey={ignored}`
- Delete records:
   - `POST /deleterecord/{database}&table={table}&apikey={ignored}`
- Query table data:
   - `GET /query/{database}&table={table}&select={select}&where={where}&expand={expand}&apikey={ignored}`
- Query table schema:
   - `GET /querytableschema/{database}&table={table}&apikey={ignored}`
- Query relationships and nested data:
   - `GET /queryrelationship/{database}&relationship={relationship}&apikey={ignored}`
   - `GET /queryall/{database}&table={table}&depth={depth}&apikey={ignored}`
- Retrieve attachments:
   - `GET /retrieveattachment/{database}&table={table}&id={id}&apikey={ignored}`

All of the above must be called with:

```http
Authorization: Bearer <JWT_TOKEN>
```

If the token is missing/invalid, or the user is not allowed to access `{database}`, the server responds with HTTP 403.

### Admin-only actions

These operations require a JWT for a user whose role is `admin`:

- Create tables:
   - `POST /createtable/{database}&table={table}&gps={gps}&apikey={ignored}`
- Drop tables:
   - `POST /droptable/{database}&table={table}&apikey={ignored}`
- Create relationships:
   - `POST /relationship/{database}&apikey={ignored}`
   - `POST /relateparent/{database}&parent_table={parent_table}&child_table={child_table}&relationship_name={relationship_name}&apikey={ignored}`
- Introspect database schema:
   - `GET /querydatabase/{database}&expand={expand}&apikey={ignored}`
- Create a new logical database/schema:
   - `POST /createdatabase/{database}&apikey={ignored}`

Again, the `apikey` path segment is ignored; only the JWT and the user’s role matter.

## Example Frontend Flow (Auth Perspective)

From a frontend or API client, a typical flow to insert/query data is:

1. **Register or obtain an account**
    - `POST /auth/register` with email/password (first user becomes admin).
    - An admin can later set `allowed_schemas` for other users via `/admin/users/schemas`.

2. **Login to get a token**
    - `POST /auth/login` with email/password.
    - Store the returned `token` on the client (e.g., in memory or secure storage).

3. **Call database endpoints with the token**
    - Example: insert into table `my_table` in schema `my_schema`:

       ```http
       POST /insert/my_schema&table=my_table&apikey=ignored
       Authorization: Bearer <JWT_TOKEN>
       Content-Type: application/json

       [
          { "col1": "value1", "col2": 123 }
       ]
       ```

    - Example: query that same table:

       ```http
       GET /query/my_schema&table=my_table&select=*&where=1=1&expand=false&apikey=ignored
       Authorization: Bearer <JWT_TOKEN>
       ```

4. **Rely on roles/schemas for enforcement**
    - If the JWT belongs to a user who does **not** have access to `my_schema`, the request will be rejected with 403.
    - Admins (role `admin`) can call the admin-only endpoints to inspect or manage schemas, tables, and relationships.

## Architecture & Migration Notes

- The project was originally built against MariaDB/MySQL using the `mysql` crate.
- It has been migrated to PostgreSQL using Diesel (with the `postgres` feature) and a `PgConnection`-based connection layer.
- A single physical Postgres database (`api_main`) is used; logical "databases" are represented as schemas (for example, `"ApiKey"` and `"Relationships"`).
- The Rust code obtains a connection via a type alias `PooledConn = PgConnection` and reads its connection settings from the `DATABASE_URL` environment variable.
- Most queries are still expressed as raw SQL strings and executed via `diesel::sql_query(...)`, with lightweight structs implementing `QueryableByName` for mapping result rows.
