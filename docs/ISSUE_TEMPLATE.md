# Zero-One Issue Prefixes and Template

Use these prefixes at the start of the issue title so the main theme is obvious
at a glance and triage stays fast.

## Title Prefixes

Use a short bracketed prefix in the title, followed by a concise summary. For a full list of available prefixes and their descriptions, see [TITLE_PREFIXES.md](TITLE_PREFIXES.md).

Examples:

- `[ACP] implement session handshake`
- `[UI] fix empty state spacing`
- `[CORE] add shared session model`
- `[AUTH] add provider token refresh`
- `[CLI] improve command parsing`

### General issue type prefixes

- `bug` - Something is broken or incorrect
- `enhancement` - New capability or improvement
- `documentation` - Docs-only change
- `refactor` - Behavior-preserving code change
- `test` - Tests or test infrastructure
- `chore` - Maintenance or housekeeping work
- `question` - Needs clarification
- `priority:high` - Important work that needs attention soon

### Project area prefixes

- `package:zero-one` - Core Rust engine, agent runtime, storage, and integrations
- `package:ui` - Desktop UI package built with Tauri and Vue
- `area:cli` - CLI entry points and shell workflows
- `area:core-platform` - Configuration, storage, state, and shared contracts
- `area:agent-runtime` - Agent orchestration, context, tools, and execution flow
- `area:agent-orchestration` - Task planning and agent lifecycle
- `area:context-assembly` - Prompt and memory context building
- `area:auth` - Provider credentials, tokens, and account access
- `area:model-provider` - Model adapters, auth, routing, and provider integrations
- `area:model-routing` - Provider selection and fallback policies
- `area:developer-interfaces` - MCP, ACP, LSP, IDE, and command integrations
- `area:ui-platform` - Desktop UI shell, views, navigation, and interaction surfaces
- `area:ui-interaction` - Chat panels and tool visualization
- `area:workflow` - Sessions, projects, memory, annotations, and search
- `area:sessions` - Session lifecycle, history, and replay
- `area:storage` - Local storage, persistence, and indexing
- `area:prompts` - Custom prompts, templates, and authoring
- `area:skills` - Skill definitions and execution metadata
- `area:tooling` - Web search, writer, file, shell, diff, and git tools
- `area:projects` - Project grouping and workspace organization
- `area:ide-integration` - Editor and IDE extension details
- `area:observability` - Logging, diagnostics, and telemetry
- `area:security` - Permissions, secrets, and trust boundaries
- `area:quality-trust` - Security, permissions, testing, telemetry, and reliability
- `area:packaging` - Release, versioning, updates, docs, and migrations
- `area:automation` - Hooks, triggers, and automated workflows
- `area:lsp` - Language Server Protocol integration work

### Integration and protocol prefixes

- `protocol:mcp` - Model Context Protocol support
- `protocol:acp` - Agent communication protocol support
- `protocol:lsp` - Language Server Protocol integration
- `integration:copilot` - GitHub Copilot provider integration
- `integration:openrouter` - OpenRouter provider integration
- `integration:git` - Git-aware features and repository context

### Suggested title themes

Use the main title prefix that best matches the issue scope:

- `[CORE]` for shared engine and domain work
- `[UI]` for desktop interface and interactions
- `[ACP]` for ACP protocol and agent-bridge work
- `[MCP]` for MCP server and protocol support
- `[LSP]` for editor protocol support
- `[AUTH]` for sign-in, tokens, and provider access
- `[CLI]` for command-line workflow and commands
- `[MODEL]` for provider selection, routing, and response handling
- `[WORKFLOW]` for sessions, memory, and project management
- `[TOOLS]` for tool execution and integrations
- `[GIT]` for Git-aware actions and repository context
- `[PACKAGING]` for release and distribution work
- `[SECURITY]` for permissions, secrets, and trust boundaries
- `[OBSERVABILITY]` for logging, diagnostics, and telemetry
- `[AUTOMATION]` for hooks, triggers, and automated workflows
- `[TEST]` for test coverage and test infrastructure
- `[DOCS]` for documentation-only work
- `[REFACTOR]` for structural changes without behavior changes
- `[CHORE]` for maintenance and housekeeping

## How to Use Prefixes

Use one title prefix from the most relevant category when possible:

- Main theme: `[UI]`, `[ACP]`, `[MCP]`, `[LSP]`, `[CORE]`, `[MODEL]`, `[AUTH]`, `[CLI]`, `[WORKFLOW]`, `[TOOLS]`, `[GIT]`, `[PACKAGING]`, `[SECURITY]`, `[OBSERVABILITY]`, `[AUTOMATION]`, `[TEST]`, `[DOCS]`, `[REFACTOR]`, or `[CHORE]`
- Secondary labels: use the matching repo labels for area, package, or protocol
- Priority: add `priority:high` only when the issue should be treated as urgent

Example title and labels:

- Title: `[UI] fix empty state spacing`
- Labels: `area:ui-platform`, `bug`

- Title: `[ACP] implement session handshake`
- Labels: `protocol:acp`, `area:developer-interfaces`, `enhancement`

- Title: `[CORE] add shared session model`
- Labels: `package:zero-one`, `area:core-platform`, `refactor`

- Title: `[AUTH] add provider token refresh`
- Labels: `area:auth`, `integration:openrouter`, `enhancement`

## Issue creation template

Use this template when opening a new issue:

```md
## Summary

<!-- Start with a bracketed prefix, then a short summary. Example: [UI] fix empty state spacing -->

## Title prefix

- Prefix:
- Matching label(s):
- Priority:

## Problem

<!-- Describe the bug, feature gap, or task. -->

## Expected behavior

<!-- Describe what should happen instead. -->

## Current behavior

<!-- Describe what is happening now. -->

## Notes

- Reproduction steps:
- Related files or modules:
- Screenshots or logs:
- Suggested approach:

## Acceptance criteria

- [ ]
- [ ]
- [ ]
```

## Tips

- Keep the summary specific.
- Keep the prefix short and high-level.
- Prefer one primary area label over multiple overlapping ones.
- Add screenshots, logs, or reproduction steps when relevant.
- If the issue touches both packages, note that clearly in the notes section.
