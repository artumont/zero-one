from __future__ import annotations

import re
from dataclasses import dataclass
from pathlib import Path

from .naming import to_snake_case
from .types import ColumnDef, ForeignKeyDef, IndexDef, SchemaDef, TableDef


FIELD_LINE_RE = re.compile(r"^\s*(?:pub\s+)?(?P<name>[a-zA-Z_][a-zA-Z0-9_]*)\s*:\s*(?P<typ>[^,]+),")
STRUCT_RE = re.compile(r"(?P<prefix>(?:\s*#\[[^\]]+\]\s*)*)\s*pub\s+struct\s+(?P<name>[A-Za-z_][A-Za-z0-9_]*)\s*\{(?P<body>.*?)\n\}", re.S)
ATTR_RE = re.compile(r"^\s*#\[(?P<attr>[^\]]+)\]\s*$")


@dataclass(slots=True)
class ParsedField:
    attrs: list[str]
    name: str
    rust_type: str


def _extract_fields(body: str) -> list[ParsedField]:
    fields: list[ParsedField] = []
    pending_attrs: list[str] = []
    for line in body.splitlines():
        attr_match = ATTR_RE.match(line)
        if attr_match:
            pending_attrs.append(attr_match.group("attr").strip())
            continue

        field_match = FIELD_LINE_RE.match(line)
        if field_match:
            fields.append(
                ParsedField(
                    attrs=pending_attrs[:],
                    name=field_match.group("name"),
                    rust_type=field_match.group("typ").strip(),
                )
            )
            pending_attrs.clear()
            continue

        if line.strip() and pending_attrs:
            pending_attrs.clear()
    return fields


def _is_model_struct(prefix: str) -> bool:
    return "derive(" in prefix and "Model" in prefix


def _unwrap_option(rust_type: str) -> tuple[str, bool]:
    stripped = rust_type.strip()
    if stripped.startswith("Option<") and stripped.endswith(">"):
        return stripped[len("Option<") : -1].strip(), True
    return stripped, False


def _rust_type_to_sql_type(rust_type: str) -> str:
    base, _ = _unwrap_option(rust_type)
    base = base.strip()
    if base in {"i8", "i16", "i32", "i64", "u8", "u16", "u32", "u64", "usize", "isize", "bool"}:
        return "INTEGER"
    if base in {"f32", "f64"}:
        return "REAL"
    if base in {"String", "str", "jiff::Timestamp"}:
        return "TEXT"
    return "TEXT"


def _default_to_sql(default_attr: str, sql_type: str) -> str:
    if default_attr == "false":
        return "0"
    if default_attr == "true":
        return "1"
    if default_attr == "jiff::Timestamp::now()":
        return "CURRENT_TIMESTAMP"
    if default_attr.startswith("\"") and default_attr.endswith("\""):
        # Convert Rust double-quoted string literal to a proper SQL single-quoted string
        # literal (double quotes are identifiers in SQL, not string literals)
        inner = default_attr[1:-1].replace("'", "''")
        return f"'{inner}'"
    if sql_type in {"INTEGER", "REAL"} and re.fullmatch(r"-?\d+(?:\.\d+)?", default_attr):
        return default_attr
    return f"'{default_attr}'"


def _extract_default(attrs: list[str]) -> str | None:
    for attr in attrs:
        if attr.startswith("default(") and attr.endswith(")"):
            return attr[len("default(") : -1].strip()
    return None


def _extract_belongs_to(attrs: list[str]) -> tuple[str, str] | None:
    for attr in attrs:
        if not attr.startswith("belongs_to("):
            continue
        key_match = re.search(r"key\s*=\s*([a-zA-Z_][a-zA-Z0-9_]*)", attr)
        ref_match = re.search(r"references\s*=\s*([a-zA-Z_][a-zA-Z0-9_]*)", attr)
        if key_match and ref_match:
            return key_match.group(1), ref_match.group(1)
    return None


def _extract_relation_target(rust_type: str) -> str | None:
    generic_match = re.search(r"<\s*([A-Za-z_][A-Za-z0-9_:]*)\s*>", rust_type)
    if not generic_match:
        return None
    return generic_match.group(1).split("::")[-1]


def parse_models(models_path: Path) -> SchemaDef:
    schema = SchemaDef()
    struct_to_table: dict[str, str] = {}
    parsed_structs: list[tuple[str, list[ParsedField]]] = []

    for file_path in sorted(models_path.glob("*.rs")):
        content = file_path.read_text(encoding="utf-8")
        for match in STRUCT_RE.finditer(content):
            prefix = match.group("prefix")
            if not _is_model_struct(prefix):
                continue
            struct_name = match.group("name")
            fields = _extract_fields(match.group("body"))
            parsed_structs.append((struct_name, fields))
            struct_to_table[struct_name] = to_snake_case(struct_name)

    for struct_name, fields in parsed_structs:
        table_name = struct_to_table[struct_name]
        table = TableDef(name=table_name)

        for field in fields:
            if any(attr.startswith("has_many") for attr in field.attrs):
                continue
            if any(attr.startswith("belongs_to") for attr in field.attrs):
                continue

            base_type, is_optional = _unwrap_option(field.rust_type)
            sql_type = _rust_type_to_sql_type(base_type)
            is_key = any(attr == "key" for attr in field.attrs)
            is_auto = any(attr == "auto" for attr in field.attrs)
            default_attr = _extract_default(field.attrs)
            default_sql = _default_to_sql(default_attr, sql_type) if default_attr else None

            table.columns[field.name] = ColumnDef(
                name=field.name,
                sql_type=sql_type,
                nullable=is_optional and not is_key,
                default_sql=default_sql,
                primary_key=is_key,
                autoincrement=is_auto,
            )

            if any(attr == "index" for attr in field.attrs):
                idx_name = f"idx_{table_name}_{field.name}"
                index = IndexDef(name=idx_name, table=table_name, columns=(field.name,))
                table.indexes.append(index)
                schema.indexes[idx_name] = index

        for field in fields:
            rel = _extract_belongs_to(field.attrs)
            if not rel:
                continue
            key_field, ref_col = rel
            ref_struct = _extract_relation_target(field.rust_type)
            if not ref_struct:
                continue
            ref_table = struct_to_table.get(ref_struct)
            if not ref_table:
                continue
            table.foreign_keys.append(ForeignKeyDef(column=key_field, ref_table=ref_table, ref_column=ref_col))

        schema.tables[table_name] = table

    return schema
