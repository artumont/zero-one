from __future__ import annotations

import argparse
import sys

from .diff import plan_migration
from .migrations_state import reconstruct_current_schema
from .models_parser import parse_models
from .paths import migrations_dir, models_dir
from .validate import validate_migrations_dir
from .writer import write_migration


def _build_plan():
    desired = parse_models(models_dir())
    current = reconstruct_current_schema(migrations_dir())
    return plan_migration(current, desired)


def _print_plan(plan) -> None:
    if plan.warnings:
        print("Warnings:")
        for warning in plan.warnings:
            print(f"- {warning}")
        print()

    if not plan.up_statements:
        print("No schema changes detected.")
        return

    print("UP SQL:")
    for stmt in plan.up_statements:
        print(stmt)
    print()

    print("DOWN SQL:")
    for stmt in plan.down_statements or ["-- No rollback statements generated."]:
        print(stmt)


def cmd_plan(_: argparse.Namespace) -> int:
    plan = _build_plan()
    _print_plan(plan)
    return 0


def cmd_generate(args: argparse.Namespace) -> int:
    plan = _build_plan()
    if not plan.up_statements:
        print("No schema changes detected. Nothing generated.")
        return 0

    folder = write_migration(migrations_dir(), args.name, plan)
    print(f"Generated migration: {folder}")
    if plan.warnings:
        print("Warnings:")
        for warning in plan.warnings:
            print(f"- {warning}")
    return 0


def cmd_check(_: argparse.Namespace) -> int:
    issues = validate_migrations_dir(migrations_dir())
    if not issues:
        print("Migrations are valid.")
        return 0
    print("Migration validation issues:")
    for issue in issues:
        print(f"- {issue}")
    return 1


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        prog="z1-migrate",
        description="Migration creator for zero-one",
    )

    sub = parser.add_subparsers(dest="command", required=True)

    plan_parser = sub.add_parser("plan", help="show detected migration SQL without writing files")
    plan_parser.set_defaults(func=cmd_plan)

    gen_parser = sub.add_parser("generate", help="generate migration files")
    gen_parser.add_argument("name", help="human-readable migration name")
    gen_parser.set_defaults(func=cmd_generate)

    check_parser = sub.add_parser("check", help="validate migration folder structure and metadata")
    check_parser.set_defaults(func=cmd_check)

    return parser


def main() -> None:
    parser = build_parser()
    args = parser.parse_args()
    code = args.func(args)
    raise SystemExit(code)


if __name__ == "__main__":
    main()
