from __future__ import annotations

import json
from pathlib import Path


def validate_migrations_dir(migrations_dir: Path) -> list[str]:
    issues: list[str] = []
    if not migrations_dir.exists():
        return [f"Migrations directory does not exist: {migrations_dir}"]

    checksums: set[str] = set()
    dependencies: dict[str, list[str]] = {}

    for folder in sorted(p for p in migrations_dir.iterdir() if p.is_dir()):
        metadata_path = folder / "metadata.json"
        up_path = folder / "up.sql"
        down_path = folder / "down.sql"

        if not metadata_path.exists():
            issues.append(f"Missing metadata.json in {folder.name}")
            continue
        if not up_path.exists():
            issues.append(f"Missing up.sql in {folder.name}")
        if not down_path.exists():
            issues.append(f"Missing down.sql in {folder.name}")

        try:
            metadata = json.loads(metadata_path.read_text(encoding="utf-8"))
        except Exception as exc:
            issues.append(f"Invalid metadata.json in {folder.name}: {exc}")
            continue

        checksum = metadata.get("checksum")
        name = metadata.get("name")
        depends_on = metadata.get("depends_on", [])

        if not isinstance(checksum, str) or not checksum:
            issues.append(f"Invalid checksum in {folder.name}")
            continue
        if checksum in checksums:
            issues.append(f"Duplicate checksum {checksum} in {folder.name}")
        checksums.add(checksum)

        if not isinstance(name, str) or not name:
            issues.append(f"Invalid name in {folder.name}")
        if name != folder.name:
            issues.append(f"Metadata name mismatch in {folder.name}: expected '{folder.name}', got '{name}'")

        if not isinstance(depends_on, list) or not all(isinstance(item, str) for item in depends_on):
            issues.append(f"Invalid depends_on in {folder.name}")
        else:
            dependencies[checksum] = depends_on

    for checksum, deps in dependencies.items():
        for dep in deps:
            if dep not in checksums:
                issues.append(f"Checksum {checksum} depends on missing checksum {dep}")

    return issues
