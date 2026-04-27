# Migrations

This document describes the migration contract enforced by the Rust migrator and the migration creator tool.

## Runtime contract (Rust migrator)

The migrator loads migrations from `packages/zero-one/migrations`.

Each migration must be a folder containing:

- `metadata.json`
- `up.sql`
- `down.sql`

`metadata.json` must contain:

- `checksum` (string, unique)
- `name` (string, should match the folder name)
- `depends_on` (array of migration checksums)

The Rust migrator:

- Tracks applied migrations in `__z1_migrations` by checksum.
- Builds a map keyed by `checksum`.
- Resolves dependencies via `depends_on` before applying migrations.
- Executes `up.sql` on upgrade and `down.sql` on rollback for failed apply attempts.

## Migration creator tool

Location: `tools/migration-creator`

The tool parses Rust models from `packages/zero-one/src/storage/models`, reconstructs current schema by replaying migration `up.sql` files, computes a diff, and generates new migration files.

### Setup

```bash
cd tools/migration-creator
poetry install
```

### Usage

```bash
poetry run z1-migrate plan
poetry run z1-migrate generate <name>
poetry run z1-migrate check
```

### Naming and placement

- Generated folders are written to `packages/zero-one/migrations`.
- Folder/name format: `<unix_timestamp>_<slug>`.
- New migration depends on the latest existing migration checksum (linear dependency chain).

### Safety rules

- Additive changes are generated automatically when possible.
- Destructive/risky changes are allowed with warnings.
- If SQLite requires table rebuilds (common for FK drops/type changes/column drops), the tool emits explicit warning comments in generated SQL for manual completion.
- Always review generated SQL before running app migrations.
