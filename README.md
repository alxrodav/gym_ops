# Gym Operations

## TODO

- dotenv

## SQLite Database

First, make sure [SQLite](https://sqlite.org/index.html) is installed in your system.

```sh
sqlite3 --version
```

To manage DB Migrations, this project uses [SQLx CLI](https://crates.io/crates/sqlx-cli).

```sh
cargo install sqlx-cli --no-default-features --features sqlite,completions
sqlx database create
sqlx migrate run
```

## Server

```sh
cd gops_server && cargo watch -x run
```

### Update dependencies

```sh
cd gops_server && cargo update
```

## Client

```sh
cd gops_client && pnpm dev
```

### Update dependencies

```sh
cd gops_client && pnpm up -Lri
```
