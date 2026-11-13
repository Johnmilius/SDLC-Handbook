# Case 03: change how user IDs are stored

## Plain request

> Plan this change.

## Material (given to both conditions)

The task:

> Change user IDs from auto-increment integers to UUIDs across the service and its Postgres database.

Current schema:

```sql
CREATE TABLE users (
    id         SERIAL PRIMARY KEY,
    email      TEXT NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE orders (
    id       SERIAL PRIMARY KEY,
    user_id  INTEGER NOT NULL REFERENCES users(id),
    total    NUMERIC(10,2) NOT NULL
);

CREATE TABLE sessions (
    token    TEXT PRIMARY KEY,
    user_id  INTEGER NOT NULL REFERENCES users(id),
    expires  TIMESTAMPTZ NOT NULL
);
```

Where the integer ID is used today:

- `GET /users/{id}` and `GET /users/{id}/orders` take the integer in the URL.
- The nightly CSV export for the finance team has a `user_id` column.
- The Rust code uses `struct UserId(i32)` in about 40 places.
- The users table has roughly 2 million rows. The service runs around the clock.

The plan a teammate proposed:

```sql
-- migration 0042
ALTER TABLE orders   DROP CONSTRAINT orders_user_id_fkey;
ALTER TABLE sessions DROP CONSTRAINT sessions_user_id_fkey;
ALTER TABLE users    ALTER COLUMN id DROP DEFAULT;
ALTER TABLE users    ALTER COLUMN id TYPE uuid USING gen_random_uuid();
ALTER TABLE orders   ALTER COLUMN user_id TYPE uuid USING gen_random_uuid();
ALTER TABLE sessions ALTER COLUMN user_id TYPE uuid USING gen_random_uuid();
ALTER TABLE orders   ADD FOREIGN KEY (user_id) REFERENCES users(id);
ALTER TABLE sessions ADD FOREIGN KEY (user_id) REFERENCES users(id);
```

"Then we change `UserId(i32)` to `UserId(Uuid)`, deploy, and run the migration on Friday evening."
