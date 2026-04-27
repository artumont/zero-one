# z1-migration-creator

Automatic SQLite migration generator for the zero-one Rust models.

## Quick start

```bash
poetry install
poetry run z1-migrate plan
poetry run z1-migrate generate initial_setup
```

## Commands

- `z1-migrate plan`
  - Parse Rust models under `packages/zero-one/src/storage/models`.
  - Reconstruct current schema by replaying existing migration `up.sql` files.
  - Print detected SQL statements and warnings without writing files.

- `z1-migrate generate <name>`
  - Same detection logic as `plan`.
  - Writes migration files into `packages/zero-one/migrations/<timestamp>_<name>/`.

- `z1-migrate check`
  - Validate migration folders and metadata shape.
  - Ensure dependency references exist.

## Notes

- This tool is intentionally lightweight and only uses Python stdlib.
- Destructive or risky changes emit warnings and still generate output.
- Always review generated SQL before running the Rust migrator.
