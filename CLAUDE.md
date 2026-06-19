# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Development

```bash
# Rust backend (build, lint, test)
cargo build
cargo clippy
cargo test

# Frontend dev server (proxies /api and /pool to :8180)
cd frontend && npm run dev

# Frontend production build (output to frontend/dist/ — embedded by Rust)
cd frontend && npm run build        # tsc + vite build
cd frontend && npx vite build       # skip type-check for faster iteration

# Run full stack (after frontend build)
cargo run
# Server listens on http://0.0.0.0:8180, serves frontend UI and proxy API
```

The frontend dist is embedded into the Rust binary via `rust-embed` (`src/api/router.rs`). Always rebuild the frontend before `cargo run` when making UI changes.

## Architecture

### Rust backend (`src/`)

**`main.rs`** — Startup: loads `config.yaml`, discovers OpenCode workspaces, builds workspace scheduler, starts axum server on `0.0.0.0:8180`. Spawns background refresh task.

**`workspace/`** — Workspace-level scheduling:
- `credential.rs` — `WorkspaceCredential` stores the single upstream credential retained for a workspace and exposes masked display helpers.
- `record.rs` — `WorkspacePool` stores workspace identity, account identity, subscription state, Go usage, and one `WorkspaceCredential`.
- `scheduler.rs` — `WorkspaceScheduler` schedules available workspaces, preserves workspace affinity, rotates by workspace usage, and returns `SelectedWorkspaceCredential` for proxy requests.
- `handle.rs` — `WorkspaceSchedulerHandle` owns the runtime scheduler, configuration snapshots, refresh guard, upstream clients, and request logs.
- `discovery.rs` — discovers accounts and workspaces, reads each workspace key list, and keeps the first key as the workspace proxy credential.

Workspace is the quota boundary. One account can have multiple workspaces, and workspace quotas are independent. One workspace can expose multiple OpenCode keys, while those keys share the same workspace quota, so runtime scheduling stores exactly one credential per workspace and never models key-level capacity.

**`proxy/`** — Request forwarding:
- `openai.rs` — `/go/v1/chat/completions` handler: parses `ChatCompletionRequest`, applies image filter, retries across available workspaces, and removes exhausted workspaces from the runtime scheduler.
- `claude.rs` — `/go/v1/messages` handler: parses `AnthropicMessagesRequest` and uses the same workspace retry pattern.
- `filter.rs` — Image content filter (remove/replace/pass_through per model config).
- `stream.rs` — SSE forwarding with `data: [DONE]` termination for OpenAI.
- `error.rs` — OpenAI/Anthropic error response formatters.

**`protocol/`** — Strict typed request structs with enums (`ChatRole`, `AnthropicRole`, `ContentPart`, `AnthropicContentBlock`, etc.). Uses `#[serde(untagged)]` for string-vs-array content discriminator and `#[serde(flatten)] extra: HashMap` for passthrough of unmodeled fields.

**`opencode/`** — OpenCode.ai integration:
- `client.rs` — `OpencodeClient`: scrapes workspace pages (HTML) and calls SolidStart server functions to discover workspaces, workspace keys, and Go usage.
- `serverfn.rs` — SolidStart `$R` response parser.
- `types.rs` — `Workspace`, `ApiKeyEntry`, `BillingInfo`, `GoUsage`, `SubscriptionPlan`.

**`api/`** — Axum REST routes: pool status, accounts CRUD, config read/write (`/api/config` supports `image_filter` field), model list proxy, request logs, force refresh.

**`model/`** — `LogEntry` and `Direction` (OpenAI/Claude) types for request logging.

**`config.rs`** — YAML config: `fixed.listen` for startup-only settings and `runtime` for hot-updated accounts, refresh interval, retry policy, Go upstream, image filter, and API token.

### Frontend (`frontend/src/`)

React 19 SPA, embedded in Rust binary. Uses Tailwind (custom cream/espresso/terra/harvest palette), framer-motion animations, TanStack Query for data fetching.

- `features/dashboard/` — Workspace stats + Go usage overview
- `features/workspaces/` — Workspace scheduling view with search/filter and affinity workspace controls
- `features/models/` — Model list by provider (OpenAI/Claude tabs)
- `features/logs/` — Request log table with protocol/success filters
- `features/accounts/` — Account CRUD with workspace sections
- `features/settings/` — General config form + `ImageFilterForm` (model picker with API-fetched suggestions, custom Select with framer-motion dropdown)
- `features/pool/` — Pool status hook, WorkspaceSection, UsageBar
- `shared/ui/` — Button, Input, Select (custom, not native), Card, Dialog, Sidebar, Badge, etc.
- `shared/types/api.ts` — TypeScript mirrors of all API response types

### Workspace trust hierarchy

1. **Upstream API response** — authoritative for quota exhaustion (402/429 + specific keywords: `insufficient`, `quota`, `balance` only — NOT `exceeded`, `rate_limit`, or `overloaded_error`)
2. **Go usage data** (`go_usage`) — scraped from OpenCode workspace Go page, used for workspace scheduling and exhaustion detection
3. **Memory cache** — workspace queue, affinity workspace, current workspace — refreshed asynchronously, overridden by API signals

### Config (`config.yaml`)

```yaml
fixed:
  listen: "0.0.0.0:8180"
runtime:
  accounts:
    - name: "main"
      auth: "<opencode-auth-cookie>"
      label: "主账号"
  refresh_interval_secs: 6000
  max_retries: 10
  go:
    base_url: "https://opencode.ai/zen/go/v1"
    connect_timeout_secs: 90
    request_timeout_secs: 90
  image_filter:
    models:
      - model: "gpt-4"
        action: remove           # pass_through | remove | replace
      - model: "claude-3-haiku"
        action: replace
        replacement: "[Image not supported]"
  api_token: "admin-token"
```
