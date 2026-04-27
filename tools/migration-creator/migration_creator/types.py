from __future__ import annotations

from dataclasses import dataclass, field


@dataclass(slots=True)
class ColumnDef:
    name: str
    sql_type: str
    nullable: bool = False
    default_sql: str | None = None
    primary_key: bool = False
    autoincrement: bool = False


@dataclass(slots=True)
class ForeignKeyDef:
    column: str
    ref_table: str
    ref_column: str


@dataclass(slots=True)
class IndexDef:
    name: str
    table: str
    columns: tuple[str, ...]
    unique: bool = False

    def signature(self) -> tuple[str, tuple[str, ...], bool]:
        return (self.table, self.columns, self.unique)


@dataclass(slots=True)
class TableDef:
    name: str
    columns: dict[str, ColumnDef] = field(default_factory=dict)
    foreign_keys: list[ForeignKeyDef] = field(default_factory=list)
    indexes: list[IndexDef] = field(default_factory=list)


@dataclass(slots=True)
class SchemaDef:
    tables: dict[str, TableDef] = field(default_factory=dict)
    indexes: dict[str, IndexDef] = field(default_factory=dict)


@dataclass(slots=True)
class MigrationPlan:
    up_statements: list[str] = field(default_factory=list)
    down_statements: list[str] = field(default_factory=list)
    warnings: list[str] = field(default_factory=list)
