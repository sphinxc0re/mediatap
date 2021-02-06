# mediatap

## Development

### Database migrations

Since `mediatap` is using SQLite as a database backend, you have to supply the path to the database file each time you make a migration with the `diesel` cli. The following command simplifies this task massively:

```shell
diesel migration run --database-url "$(cargo run -- emit-database-path)"
```

Notice that the `emit-database-path` subcommand is not available in release mode.
