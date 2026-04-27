from __future__ import annotations

import hashlib
import json
import sqlite3
from dataclasses import dataclass
from pathlib import Path

from .types import ColumnDef, ForeignKeyDef, IndexDef, SchemaDef, TableDef


@dataclass(slots=True)
class MigrationEntry:
    folder: Path
    checksum: str
    name: str
    depends_on: list[str]


def read_migration_entries(migrations_dir: Path) -> list[MigrationEntry]:
    if not migrations_dir.exists():
        return []

    entries: list[MigrationEntry] = []
    for child in sorted((p for p in migrations_dir.iterdir() if p.is_dir()), key=lambda p: p.name):
        metadata_path = child / "metadata.json"
        if not metadata_path.exists():
            continue
        data = json.loads(metadata_path.read_text(encoding="utf-8"))
        entries.append(
            MigrationEntry(
                folder=child,
                checksum=str(data["checksum"]),
                name=str(data["name"]),
                depends_on=[str(item) for item in data.get("depends_on", [])],
            )
        )
    return entries


def latest_checksum(entries: list[MigrationEntry]) -> str | None:
    if not entries:
        return None
    return entries[-1].checksum


def checksum_for_migration(name: str, up_sql: str, down_sql: str) -> str:
    payload = f"{name}\n--UP--\n{up_sql}\n--DOWN--\n{down_sql}".encode("utf-8")
    return hashlib.sha256(payload).hexdigest()


def reconstruct_current_schema(migrations_dir: Path) -> SchemaDef:
    conn = sqlite3.connect(":memory:")
    conn.execute("PRAGMA foreign_keys = ON;")

    for entry in read_migration_entries(migrations_dir):
        up_path = entry.folder / "up.sql"
        if not up_path.exists():
            continue
        sql = up_path.read_text(encoding="utf-8")
        conn.executescript(sql)

    schema = _introspect_schema(conn)
    conn.close()
    return schema


def _introspect_schema(conn: sqlite3.Connection) -> SchemaDef:
    schema = SchemaDef()
    tables = conn.execute(
        "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'"
    ).fetchall()

    for (table_name,) in tables:
        table = TableDef(name=table_name)
        col_rows = conn.execute(f"PRAGMA table_info('{table_name}')").fetchall()
        for col in col_rows:
            _, col_name, col_type, notnull, default_value, pk = col
            table.columns[col_name] = ColumnDef(
                name=col_name,
                sql_type=(col_type or "TEXT").upper(),
                nullable=not bool(notnull),
                default_sql=default_value,
                primary_key=bool(pk),
                autoincrement=False,
            )

        fk_rows = conn.execute(f"PRAGMA foreign_key_list('{table_name}')").fetchall()
        for fk in fk_rows:
            _, _, ref_table, from_col, to_col, *_ = fk
            table.foreign_keys.append(
                ForeignKeyDef(column=from_col, ref_table=ref_table, ref_column=to_col)
            )

        idx_rows = conn.execute(f"PRAGMA index_list('{table_name}')").fetchall()
        for idx in idx_rows:
            _, idx_name, is_unique, _, _ = idx
            idx_cols = conn.execute(f"PRAGMA index_info('{idx_name}')").fetchall()
            col_names = tuple(row[2] for row in idx_cols)
            index = IndexDef(
                name=idx_name,
                table=table_name,
                columns=col_names,
                unique=bool(is_unique),
            )
            table.indexes.append(index)
            schema.indexes[idx_name] = index

        schema.tables[table_name] = table

    return schema
