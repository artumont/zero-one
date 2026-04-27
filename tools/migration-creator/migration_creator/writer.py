from __future__ import annotations

import json
import re
import time
from pathlib import Path

from .migrations_state import checksum_for_migration, latest_checksum, read_migration_entries
from .types import MigrationPlan


def slugify(name: str) -> str:
    slug = name.strip().lower()
    slug = re.sub(r"[^a-z0-9]+", "_", slug)
    slug = re.sub(r"_+", "_", slug).strip("_")
    return slug or "migration"


def build_up_sql(plan: MigrationPlan) -> str:
    lines = ["BEGIN;", ""]
    for stmt in plan.up_statements:
        lines.append(stmt)
    lines.append("")
    lines.append("COMMIT;")
    return "\n".join(lines).strip() + "\n"


def build_down_sql(plan: MigrationPlan) -> str:
    lines = ["BEGIN;", ""]
    if plan.down_statements:
        lines.extend(plan.down_statements)
    else:
        lines.append("-- No rollback statements generated.")
    lines.append("")
    lines.append("COMMIT;")
    return "\n".join(lines).strip() + "\n"


def write_migration(migrations_dir: Path, name: str, plan: MigrationPlan) -> Path:
    migrations_dir.mkdir(parents=True, exist_ok=True)
    timestamp = str(int(time.time()))
    folder_name = f"{timestamp}_{slugify(name)}"
    folder = migrations_dir / folder_name

    if folder.exists():
        raise FileExistsError(f"Migration folder already exists: {folder}")

    up_sql = build_up_sql(plan)
    down_sql = build_down_sql(plan)

    checksum = checksum_for_migration(folder_name, up_sql, down_sql)
    existing = read_migration_entries(migrations_dir)
    depends = [latest_checksum(existing)] if latest_checksum(existing) else []

    metadata = {
        "checksum": checksum,
        "name": folder_name,
        "depends_on": depends,
    }

    folder.mkdir(parents=True, exist_ok=False)
    (folder / "metadata.json").write_text(json.dumps(metadata, indent=2) + "\n", encoding="utf-8")
    (folder / "up.sql").write_text(up_sql, encoding="utf-8")
    (folder / "down.sql").write_text(down_sql, encoding="utf-8")

    return folder
