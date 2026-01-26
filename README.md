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

## Architecture & Migration Notes

- The project was originally built against MariaDB/MySQL using the `mysql` crate.
- It has been migrated to PostgreSQL using Diesel (with the `postgres` feature) and a `PgConnection`-based connection layer.
- A single physical Postgres database (`api_main`) is used; logical "databases" are represented as schemas (for example, `"ApiKey"` and `"Relationships"`).
- The Rust code obtains a connection via a type alias `PooledConn = PgConnection` and reads its connection settings from the `DATABASE_URL` environment variable.
- Most queries are still expressed as raw SQL strings and executed via `diesel::sql_query(...)`, with lightweight structs implementing `QueryableByName` for mapping result rows.
