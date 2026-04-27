from __future__ import annotations

from pathlib import Path


def repo_root() -> Path:
    return Path(__file__).resolve().parents[3]


def models_dir() -> Path:
    return repo_root() / "packages" / "zero-one" / "src" / "storage" / "models"


def migrations_dir() -> Path:
    return repo_root() / "packages" / "zero-one" / "migrations"
