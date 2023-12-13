# Creation stages:

- install sqlx-cli crate

```shell
cargo install sqlx-cli --no-default-features --features native-tls,postgres
```

- run postgresql container run_db.sh
- set DATABASE_URL in .env
- create db

```shell
sqlx database create
```

- add migration
```shell
sqlx migrate add -r author_quote
```

- run migration

```shel
sqlx migrate run
```

