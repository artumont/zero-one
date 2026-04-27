from __future__ import annotations

from .types import ColumnDef, ForeignKeyDef, MigrationPlan, SchemaDef, TableDef


def plan_migration(current: SchemaDef, desired: SchemaDef) -> MigrationPlan:
    plan = MigrationPlan()

    current_tables = set(current.tables.keys())
    desired_tables = set(desired.tables.keys())

    for table_name in sorted(desired_tables - current_tables):
        table = desired.tables[table_name]
        plan.up_statements.append(_create_table_sql(table))
        for idx in table.indexes:
            plan.up_statements.append(_create_index_sql(idx.name, idx.table, idx.columns, idx.unique))

        plan.down_statements.insert(0, f'DROP TABLE IF EXISTS "{table_name}";')

    for table_name in sorted(current_tables - desired_tables):
        plan.warnings.append(
            f"Table '{table_name}' exists in DB schema but not in models. Consider a manual drop migration."
        )
        plan.up_statements.append(
            f"-- WARNING: table '{table_name}' is no longer present in models; manual DROP TABLE review required."
        )

    for table_name in sorted(desired_tables & current_tables):
        _diff_table(table_name, current.tables[table_name], desired.tables[table_name], plan)

    return plan


def _diff_table(table_name: str, current: TableDef, desired: TableDef, plan: MigrationPlan) -> None:
    current_cols = set(current.columns.keys())
    desired_cols = set(desired.columns.keys())

    for col_name in sorted(desired_cols - current_cols):
        col = desired.columns[col_name]
        if not col.nullable and col.default_sql is None:
            plan.warnings.append(
                f"Column '{table_name}.{col_name}' is new NOT NULL without default; generated as nullable for safety."
            )
            add_col_sql = _column_sql(ColumnDef(
                name=col.name,
                sql_type=col.sql_type,
                nullable=True,
                default_sql=col.default_sql,
                primary_key=False,
                autoincrement=False,
            ))
        else:
            add_col_sql = _column_sql(col, for_alter=True)

        plan.up_statements.append(f'ALTER TABLE "{table_name}" ADD COLUMN {add_col_sql};')
        plan.down_statements.insert(
            0,
            f"-- WARNING: SQLite cannot safely DROP COLUMN '{table_name}.{col_name}' without table rebuild.",
        )

    for col_name in sorted(current_cols - desired_cols):
        plan.warnings.append(
            f"Column '{table_name}.{col_name}' exists in DB schema but not in models. Manual drop/rebuild needed."
        )
        plan.up_statements.append(
            f"-- WARNING: column '{table_name}.{col_name}' appears removed in models; manual table rebuild needed."
        )

    for col_name in sorted(current_cols & desired_cols):
        old_col = current.columns[col_name]
        new_col = desired.columns[col_name]
        if old_col.sql_type.upper() != new_col.sql_type.upper():
            plan.warnings.append(
                f"Type change detected for '{table_name}.{col_name}' ({old_col.sql_type} -> {new_col.sql_type}). Manual migration advised."
            )
            plan.up_statements.append(
                f"-- WARNING: type change for '{table_name}.{col_name}' requires manual SQL."
            )

    current_idx_sig = {idx.signature() for idx in current.indexes}
    desired_idx_sig = {idx.signature() for idx in desired.indexes}

    for idx in desired.indexes:
        if idx.signature() not in current_idx_sig:
            plan.up_statements.append(_create_index_sql(idx.name, idx.table, idx.columns, idx.unique))
            plan.down_statements.insert(0, f'DROP INDEX IF EXISTS "{idx.name}";')

    for idx in current.indexes:
        if idx.signature() not in desired_idx_sig:
            plan.warnings.append(
                f"Index '{idx.name}' exists in DB schema but not in models. Consider dropping manually."
            )
            plan.up_statements.append(
                f"-- WARNING: index '{idx.name}' appears removed in models; review DROP INDEX manually."
            )

    current_fks = {(fk.column, fk.ref_table, fk.ref_column) for fk in current.foreign_keys}
    desired_fks = {(fk.column, fk.ref_table, fk.ref_column) for fk in desired.foreign_keys}

    for fk in sorted(desired_fks - current_fks):
        plan.warnings.append(
            f"FK '{table_name}.{fk[0]} -> {fk[1]}.{fk[2]}' added; SQLite requires table rebuild to add FK post-create."
        )
        plan.up_statements.append(
            f"-- WARNING: add foreign key '{table_name}.{fk[0]} -> {fk[1]}.{fk[2]}' manually with table rebuild."
        )

    for fk in sorted(current_fks - desired_fks):
        plan.warnings.append(
            f"FK '{table_name}.{fk[0]} -> {fk[1]}.{fk[2]}' removed; SQLite requires table rebuild."
        )
        plan.up_statements.append(
            f"-- WARNING: remove foreign key '{table_name}.{fk[0]} -> {fk[1]}.{fk[2]}' manually with table rebuild."
        )


def _create_table_sql(table: TableDef) -> str:
    parts = [_column_sql(col) for col in table.columns.values()]
    parts.extend(_fk_sql(fk) for fk in table.foreign_keys)
    joined = ",\n  ".join(parts)
    return f'CREATE TABLE IF NOT EXISTS "{table.name}" (\n  {joined}\n);'


def _column_sql(col: ColumnDef, for_alter: bool = False) -> str:
    chunks: list[str] = [f'"{col.name}" {col.sql_type}']

    if col.primary_key and not for_alter:
        if col.autoincrement and col.sql_type.upper() == "INTEGER":
            chunks.append("PRIMARY KEY AUTOINCREMENT")
        else:
            chunks.append("PRIMARY KEY")

    if not col.nullable and not (col.primary_key and not for_alter):
        chunks.append("NOT NULL")

    if col.default_sql is not None:
        chunks.append(f"DEFAULT {col.default_sql}")

    return " ".join(chunks)


def _fk_sql(fk: ForeignKeyDef) -> str:
    return (
        f'FOREIGN KEY("{fk.column}") REFERENCES "{fk.ref_table}"("{fk.ref_column}") '
        "ON UPDATE CASCADE ON DELETE RESTRICT"
    )


def _create_index_sql(name: str, table: str, cols: tuple[str, ...], unique: bool) -> str:
    unique_kw = "UNIQUE " if unique else ""
    col_sql = ", ".join(f'"{col}"' for col in cols)
    return f'CREATE {unique_kw}INDEX IF NOT EXISTS "{name}" ON "{table}" ({col_sql});'
