# actix-crud

A small CRUD service built with [actix-web](https://actix.rs/), written while learning Actix.
Cargo workspace split into focused crates (server, services, store, models, security, utils, commons).

## Stack

- **actix-web** : HTTP server
- **deadpool-postgres** : Postgres connection pool
- **Prometheus** : metrics

## Layout

```
src/
  server/    HTTP layer: routing, endpoints, middleware, config, metrics
  services/  business logic, session/context
  store/     Postgres providers: client, repositories, tables
  models/    domain entities
  security/  auth helpers
  utils/     shared helpers
  commons/   shared types
sql-scripts/ schema (users table)
```

## Configuration

Config comes from environment variables (see `src/server/src/configs/mod.rs`):

| Variable | Example |
| --- | --- |
| `PG_HOST` | `127.0.0.1` |
| `PG_PORT` | `5432` |
| `PG_USER` | `postgres` |
| `PG_PASSWORD` | `changeme` |
| `PG_DBNAME` | `actix-user` |
| `PG_POOL_MAX_SIZE` | `16` |
| `PG_CONNECTION_TIMEOUT_SECS` | `60` |
| `PG_POOL_TIMEOUTS_WAIT_NANOS` | `0` |
| `SERVER_HOST` | `127.0.0.1` |
| `SERVER_PORT` | `8080` |
| `RUST_LOG` | `actix_web=debug,actix_server=debug` |

A `static_configs()` fallback exists for local development — replace its placeholder values before use.

## Run

```bash
# create the database, then apply the schema
psql -d actix-user -f sql-scripts/user.sql

# build & run
cargo run -p server
```

## License

MIT
