# Zero-One Roadmap

## Parallel Package Structure

The project should be developed as two packages in parallel:

- `packages/zero-one` for the core Rust engine, agent runtime, storage, and integrations
- `packages/ui` for the desktop UI, built on the existing Tauri + Vue stack

Shared contracts between the packages should be kept stable and versioned so
core and UI work can move independently.

## Phase 1: Core Platform

**Goal:** Establish the foundational primitives, configuration, and data models required to operate the engine locally.

- **CLI:** Headless execution and basic commands
- **Configuration & Environment:** Loading `.env` and standard user configs
- **Observability:** Centralized logging, error handling, and diagnostics
- **Storage & State:** SQLite/local storage for sessions, memory, and settings
- **Project Model:** Structures to represent workspaces and project context
- **Shared Contracts:** Stable API boundaries for the UI

## Phase 2: Agent Runtime

**Goal:** Build the execution engine capable of orchestrating LLM calls, managing context, and executing tools safely.

- **Agent Modes:** Base templates for different interactive behaviors
- **Orchestration:** Task planning, streaming responses, and lifecycle events
- **Context Assembly:** Combining prompt templates, custom instructions, and memory
- **Tooling:** Sandboxed execution, tool registry, and skill abstractions
- **UI Event Stream:** Real-time updates for frontend clients

## Phase 3: Model and Provider Access

**Goal:** Implement a modular adapter layer for communicating with various local and remote LLMs.

- **Integrations:** OpenRouter, Copilot, and other LLM endpoints
- **Authentication:** Securely managing and storing provider credentials
- **Routing:** Model selection, fallback policies, and response normalization
- **Budgeting:** Context window management and rate-limiting

## Phase 4: Developer Interfaces

**Goal:** Enable bidirectional communication and control from external editors and standard protocols.

- **Protocols:** Model Context Protocol (MCP), ACP, and LSP support
- **IDE Extensions:** Directly exposing agent functions to VS Code/JetBrains
- **Commands:** Command palette integration and Git-aware tasks
- **UI Bridge:** Seamless request handling between the Rust core and Vue frontend

## Phase 5: UI Platform

**Goal:** Develop the Tauri-based desktop app, providing a rich, responsive interface mapping to core capabilities.

- **Layout & Navigation:** App shell, responsive windows, and project switcher
- **Core Views:** Chat panels, session list, and memory/notes browser
- **Tooling Views:** Visual feedback for tool execution, diffs, and loading states
- **Control Panels:** Settings for models, auth, storage, and prompts
- **Keyboard & UX:** Command palette, shortcuts, accessibility, and error states

## Phase 6: Workflow Features

**Goal:** Implement user-facing workflow concepts for long-term project management.

- **Sessions & History:** Resuming past conversions and replay capabilities
- **Workspaces:** Grouping related projects and managing shared memory
- **Annotations:** Built-in notes and tracking metadata
- **Search:** Deep semantic and keyword search across all past sessions

## Phase 7: Tooling and Automation

**Goal:** Equip agents with a comprehensive, secure set of abilities to modify code and state.

- **System Operators:** File system tools, diffs, patch generation, and shell runners
- **Research:** Web search and document reading utilities
- **Git Context:** Understanding local repository state, branches, and diffs
- **Hooks:** Automating tasks based on lifecycle triggers (e.g., on PR creation)

## Phase 8: Quality and Trust

**Goal:** Ensure continuous stability, user privacy, and secure execution of third-party or generated code.

- **Security:** Permissions model, consent prompts, and secure secrets handling
- **Reliability:** Audit trails, telemetry (opt-in), bench-marking, and crash recovery
- **Testing:** Comprehensive feature fixtures and UI usability tests

## Phase 9: Packaging and Distribution

**Goal:** Finalize release pipelines to deliver smooth, auto-updating binaries for users.

- **CI/CD Pipeline:** Automated installers and multi-platform build workflows
- **Versioning:** Strategy for coordinating both package updates
- **Documentation:** Public docs site, examples, and migration guides
