# Architecture Research

**Domain:** Local-first Windows desktop command center. It has a plugin system (built-in connectors, declarative API definitions, and an MCP client), background polling, a notification engine, and an AI morning digest.
**Project:** Mornfold
**Researched:** 2026-10-18
**Confidence:** MEDIUM-HIGH. The process model, MCP mapping, and secrets design rest on current official docs. Some library-level specifics are flagged inline.

> **Stack assumption:** This document assumes the Electron + TypeScript stack (see STACK.md). The deciding factor is that the reference MCP client SDK, the Google API clients, and the AI provider SDKs are all TypeScript-first. If Tauri is chosen instead, the boundaries below still hold. The "Core" process becomes a Node sidecar and the "Main" duties move to Rust. Nothing else changes.

---

## Standard Architecture

### The One Big Decision: Three Processes, with a Headless Core

Commercial "inbox + calendar + feeds" desktop apps (Electron mail and calendar clients, Raycast-style launchers, MCP hosts like Claude Desktop) converge on the same shape:

- A **thin OS shell**, which owns windows, the tray, and notifications.
- A **headless engine**, which owns data, sync, and integrations.
- A **disposable UI**.

Mornfold should use this shape from day one. The core value statement says: "Even if everything breaks, the **elegant shell + service connection infrastructure** must work." That is only achievable if a hung Gmail sync or a crashing MCP server cannot freeze the window or the tray.

```
┌──────────────────────────────────────────────────────────────────────────┐
│ RENDERER PROCESS (UI, sandboxed, no Node)                                │
│  ┌──────────┐ ┌──────────┐ ┌───────────┐ ┌───────────┐ ┌──────────────┐  │
│  │ Today /  │ │ Card     │ │ Sources & │ │ Action    │ │ Onboarding / │  │
│  │ Digest   │ │ Library  │ │ Settings  │ │ Confirm   │ │ Wizards      │  │
│  └────┬─────┘ └────┬─────┘ └─────┬─────┘ └─────┬─────┘ └──────┬───────┘  │
│       └────────────┴──── typed RPC client + event subscriptions ┘        │
└──────────────────────────────┬───────────────────────────────────────────┘
                               │ MessagePort (brokered by Main, direct to Core)
┌──────────────────────────────┴───────────────────────────────────────────┐
│ CORE PROCESS (Electron utilityProcess "mornfold-core", full Node)        │
│                                                                          │
│  RPC Router ──► Application Services (queries, commands)                 │
│                                                                          │
│  ┌─────────────┐  ┌──────────────┐  ┌─────────────┐  ┌───────────────┐   │
│  │ Scheduler   │─►│ Connector    │─►│ Item Store  │─►│ Domain Event  │   │
│  │ (persistent │  │ Host         │  │ (upsert,    │  │ Bus           │   │
│  │ jobs, back- │  │ ├ built-ins  │  │ dedupe,     │  └──┬──────┬─────┘   │
│  │ off, rate   │  │ ├ HTTP-API   │  │ diff)       │     │      │         │
│  │ limits)     │  │ │  engine    │  └─────────────┘     ▼      ▼         │
│  └─────────────┘  │ └ MCP adapter│                 ┌────────┐ ┌───────┐  │
│                   └──────┬───────┘                 │Notify  │ │Digest │  │
│  ┌─────────────┐         │                         │Engine  │ │(AI)   │  │
│  │ Action      │◄────────┘ (execute)               └────────┘ └───────┘  │
│  │ Executor +  │                                                         │
│  │ Policy Gate │   ┌───────────┐  ┌────────────┐  ┌───────────────────┐  │
│  └─────────────┘   │ Secrets   │  │ OAuth      │  │ MCP Client Manager│  │
│                    │ Vault     │  │ (loopback) │  │ (stdio / HTTP)    │  │
│                    └───────────┘  └────────────┘  └─────────┬─────────┘  │
│  ┌────────────────────────────────────────────────┐         │            │
│  │ SQLite (single writer) + FTS5                  │         │            │
│  └────────────────────────────────────────────────┘         │            │
└───────────────────────────────▲─────────────────────────────┼────────────┘
                                │ parentPort (OS services)    │ stdio / HTTPS
┌───────────────────────────────┴──────────────────┐  ┌───────┴──────────┐
│ MAIN PROCESS (thin OS shell)                     │  │ MCP servers      │
│ lifecycle, single-instance lock, tray, windows,  │  │ (child processes │
│ Windows toasts, deep links, powerMonitor,        │  │  or remote HTTP) │
│ login item, auto-update, Core supervisor,        │  └──────────────────┘
│ RenderFetch (hidden BrowserWindow for JS pages)  │
└──────────────────────────────────────────────────┘
```

### Why each process exists

| Process | Why separate | What happens if it dies |
|---------|-------------|-------------------------|
| **Main** | Only Main can own the tray, BrowserWindows, Notifications, `powerMonitor`, and `safeStorage`. It must never block, because a blocked Main freezes the tray and every window. | The app exits. Keep it tiny so this effectively never happens. |
| **Core** (`utilityProcess`) | All I/O-heavy and failure-prone work lives here: sync, MCP, AI, DB. `utilityProcess` is a full Node process with MessagePort support, forkable only after `app.ready` ([Electron docs](https://www.electronjs.org/docs/latest/api/utility-process)). | Main's supervisor restarts it with backoff. The UI shows "reconnecting". Persisted job state means nothing is lost. |
| **Renderer** | It has to be sandboxed, because it displays untrusted content (email bodies, web diffs, MCP output). | Main reloads the window. All state is re-fetched from Core. |
| **MCP stdio servers** | They are third-party code and must stay isolated. | The MCP Client Manager marks the connection `degraded` and restarts it with backoff. |

### Tray and window lifecycle (the "always running" model)

- **Close button hides the window; it does not quit.** Quit only from the tray menu. Hiding keeps reopening instant, which matters for the "wow on open" goal.
- **Optional memory saver:** after about 30 minutes hidden, *destroy* the BrowserWindow (frees roughly 100+ MB) and recreate it on the tray click. This is safe because the UI holds no authoritative state; Core does.
- **Single-instance lock:** a second launch focuses the existing window. Deep links and notification activations route through it.
- **Start at login:** use `app.setLoginItemSettings({ openAtLogin: true, args: ['--hidden'] })`, so the app starts in the tray without flashing a window.
- **`powerMonitor`** (`suspend`, `resume`, `unlock-screen`) is forwarded from Main to Core. On sleep the Scheduler pauses. On resume it coalesces missed runs, and the morning digest triggers "on first unlock after the configured time".

### Component Responsibilities

| Component | Lives in | Owns | Typical implementation |
|-----------|----------|------|------------------------|
| **App Shell / Lifecycle** | Main | Single-instance lock, tray menu, window create/hide/destroy, login item, auto-update | Electron `app`, `Tray`, `BrowserWindow` |
| **Core Supervisor** | Main | Forks Core, brokers MessagePorts to windows, restarts Core on crash | `utilityProcess.fork`, `MessageChannelMain` |
| **OS Bridge** | Main | Toasts, `shell.openExternal`, power events, tray badge, `safeStorage` fallback | Small RPC handler on `parentPort` |
| **RenderFetch** | Main | Renders JS-heavy pages for the web watcher (only Main can create BrowserWindows) | Hidden, offscreen `BrowserWindow`s in an isolated `session` partition, concurrency 1–2 |
| **RPC Router** | Core | Typed request/response plus push subscriptions to renderers | Shared zod contracts; one MessagePort per window |
| **Item Store** | Core | Unified `items` table, upsert/dedupe/diff, retention, FTS search | SQLite (better-sqlite3), migrations, repository layer |
| **Scheduler** | Core | Persistent job table, next-run computation, jitter, backoff, per-host rate limits, pause/resume | Single timer loop + `p-queue`/token buckets; cron for the digest time |
| **Connector Host** | Core | Loads connectors, builds a scoped `ConnectorContext`, runs `sync()`/`execute()`, tracks connection health | Plain TS modules behind one interface |
| **HTTP-API Engine** | Core | Runs declarative API definitions (catalog + user-defined): fetch, extract fields, produce items | A connector whose "code" is JSON data |
| **MCP Client Manager** | Core | MCP connection lifecycle (spawn/connect/idle-shutdown), capability discovery, list-changed handling, OAuth | `@modelcontextprotocol/client` v2 |
| **MCP Adapter** | Core | Turns MCP resources/tools into Mornfold feeds and actions (the same interface as built-ins) | Mapping layer and per-server user config |
| **Action Executor + Policy Gate** | Core | Validation, risk classification, confirmation tokens, preview, undo window, idempotency, audit log | Command pipeline |
| **Notification Engine** | Core decides, Main displays | Rules, dedupe, throttling/batching, quiet hours, deep-link payloads | Event subscriber, then OS Bridge |
| **Digest / AI Pipeline** | Core | Collect, rank, build context, call LLM, parse, store; deterministic fallback | AI SDK (`ai`) with Anthropic/Google/OpenRouter providers |
| **Secrets Vault** | Core | Encrypted secret storage, scoped access, write-only from the UI | Master key in Windows Credential Manager plus AES-GCM ciphertext in SQLite |
| **OAuth Service** | Core | Loopback redirect flows (Google, MCP HTTP servers), token refresh, re-auth state | Ephemeral `http://127.0.0.1:<random>` listener plus PKCE |
| **Card Library** | Renderer | A *fixed* set of beautifully designed card types that render any source | React components keyed by card type |
| **Confirm Surface** | Renderer | Shows exact previews for outgoing/destructive actions and returns a confirmation | Modal/sheet, bound to a server-issued token |

---

## Recommended Project Structure

```
mornfold/
├── src/
│   ├── shared/                  # Imported by all three processes; NO runtime deps on Electron/Node
│   │   ├── contracts/           # zod schemas: RPC methods, events, Item kinds, Card defs, Action defs
│   │   ├── connector-sdk/       # Connector interface, manifest types, ActionEffect enum
│   │   └── i18n/                # tr (default), en keys
│   ├── main/                    # Electron main: keep it under ~1.5k LOC
│   │   ├── lifecycle.ts         # single instance, login item, quit semantics
│   │   ├── tray.ts
│   │   ├── windows.ts           # create/hide/destroy, port brokering
│   │   ├── core-supervisor.ts   # fork + restart core utilityProcess
│   │   ├── os-bridge.ts         # notifications, openExternal, power events
│   │   └── render-fetch.ts      # hidden BrowserWindow pool for web watcher
│   ├── preload/
│   │   └── index.ts             # contextBridge: exposes ONLY the port handshake
│   ├── core/                    # utilityProcess entry; all business logic
│   │   ├── index.ts             # boot: open DB, unlock vault, start scheduler, start RPC
│   │   ├── rpc/                 # handlers per contract namespace
│   │   ├── db/                  # schema, migrations, repositories
│   │   ├── events/              # in-process domain event bus
│   │   ├── scheduler/
│   │   ├── secrets/
│   │   ├── auth/                # OAuth loopback, token refresh, Google + MCP providers
│   │   ├── connectors/
│   │   │   ├── host.ts          # registry, context factory, health state machine
│   │   │   ├── gmail/
│   │   │   ├── gcal/
│   │   │   ├── web-watch/
│   │   │   ├── x/
│   │   │   └── http-api/        # declarative API engine
│   │   ├── mcp/                 # client manager, adapter, transports, oauth provider
│   │   ├── extract/             # JSONPath/field mapping shared by http-api AND mcp
│   │   ├── actions/             # policy, confirmation tokens, executor, audit, undo queue
│   │   ├── notify/              # rules, throttle, dispatch
│   │   └── digest/              # collectors, ranking, prompt, providers, fallback
│   └── renderer/
│       ├── design-system/       # tokens, glass/blur surfaces, motion, typography
│       ├── cards/               # list, agenda, value/metric, timeline, diff, digest
│       ├── screens/             # Today, Source detail, Sources, Settings, Onboarding
│       ├── confirm/             # action preview + confirm sheet
│       ├── email-view/          # sandboxed iframe renderer for email HTML
│       └── rpc/                 # typed client, TanStack Query hooks, event → cache invalidation
├── catalog/                     # declarative free-API definitions (JSON), shipped with the app
├── resources/                   # icons, tray icons (light/dark), installer assets
└── test/
    ├── fixtures/                # recorded Gmail/Calendar/MCP/API payloads
    └── fake-mcp-server/         # in-repo stdio MCP server for integration tests
```

### Structure Rationale

- **`shared/contracts` is the spine.** Every cross-process message has a zod schema. This catches drift between Core and the UI at compile time *and* at runtime, where the renderer is untrusted.
- **`core/` holds 90% of the code**, and it is testable on Linux CI with plain Node. Development happens on Linux, so keeping Windows-specific code confined to `main/` plus the small `secrets/` adapter minimizes what needs the Windows CI runner.
- **`core/extract/` is shared** between the HTTP-API engine and the MCP adapter. "API JSON to card" and "MCP tool result to card" are the same problem, so build it once.
- **`renderer/cards/` is a closed set.** Plugins choose a card type and supply data; they never supply UI. That is how the Apple/Linear aesthetic survives an open plugin ecosystem.
- **`catalog/` is data, not code.** Adding a free API to the catalog is a JSON PR, with no code review of executable logic.

---

## Architectural Patterns

### Pattern 1: One Connector Contract, Three Implementation Tiers

**What:** Everything that produces items or performs actions implements the same `Connector` interface. Only the trust level and the implementation mechanism differ.

| Tier | Example | Code runs where | Trust |
|------|---------|-----------------|-------|
| **Built-in** | Gmail, Google Calendar, Web Watch, X | Core process | First-party, full context |
| **Declarative** | Catalog APIs, user "URL + field" APIs | Core process, *no plugin code*; the engine interprets JSON | Data only; cannot execute code |
| **MCP** | Any MCP server (GitHub, Notion, finance…) | Separate process or remote server | Untrusted; every write needs confirmation |

**When:** Always. This is how the "add a service without touching core code" requirement is met without loading arbitrary third-party JS into the process that holds your secrets.

**Trade-offs:** No third-party *JS* plugins in v1, so community extensions must be MCP servers or JSON definitions. That is the right trade for a privacy-first app. MCP is already the ecosystem's extension standard.

**Contract (sketch):**
```typescript
// shared/connector-sdk/manifest.ts
export type ItemKind = 'email' | 'event' | 'post' | 'page_change' | 'value' | 'task' | 'document' | 'generic';

export type ActionEffect =
  | 'read'            // no side effects (e.g. fetch thread)
  | 'local'           // changes only Mornfold state (pin, dismiss)
  | 'external_write'  // modifies remote state, reversible (label, mark read, create draft)
  | 'outgoing'        // sends something to other humans (send/reply email, invite attendees)
  | 'destructive';    // deletes / irreversible remote change

export interface ConnectorManifest {
  id: string;                       // 'google.gmail'
  version: string;
  displayName: string;              // i18n key
  auth: { type: 'oauth2'; provider: 'google'; scopes: string[] }
      | { type: 'apiKey'; fields: SecretField[] }
      | { type: 'none' };
  settingsSchema: JSONSchema;       // rendered as a generated form in Settings
  feeds: FeedDecl[];                // things that can be polled into items
  actions: ActionDecl[];            // things the user can do
}

export interface FeedDecl {
  id: string;                       // 'inbox', 'today-events'
  itemKind: ItemKind;
  poll: { defaultSec: number; minSec: number };
  supportsCursor: boolean;          // Gmail historyId, Calendar syncToken, ETag
  defaultCard: CardType;            // 'list' | 'agenda' | 'value' | 'timeline' | 'diff'
}

export interface ActionDecl {
  id: string;                       // 'reply'
  title: string;
  appliesTo?: ItemKind;             // shows as a button on items of this kind
  inputSchema: JSONSchema;          // generates the action form
  effect: ActionEffect;             // drives confirmation policy (Pattern 4)
}

export interface Connector {
  manifest: ConnectorManifest;
  sync(ctx: ConnectorContext, feedId: string, cursor: Cursor | null): Promise<SyncResult>;
  preview?(ctx: ConnectorContext, actionId: string, input: unknown): Promise<ActionPreview>;
  execute(ctx: ConnectorContext, actionId: string, input: unknown, opts: { idempotencyKey: string }): Promise<ActionResult>;
}

export interface SyncResult {
  items: ItemDraft[];               // normalized; the store does dedupe/diff
  cursor: Cursor | null;            // persisted by the scheduler
  fullResync?: boolean;             // e.g. Gmail history 404 → full sync done
  nextPollHintSec?: number;         // connector can ask to slow down
}

// The ONLY capabilities a connector gets. No raw DB, no other connectors' secrets.
export interface ConnectorContext {
  http: RateLimitedFetch;           // per-host limits, Retry-After, backoff baked in
  secrets: ScopedSecrets;           // get/set within this connection's namespace only
  oauth: TokenProvider;             // auto-refresh; throws AuthRequired
  log: Logger;                      // secret-redacting
  signal: AbortSignal;              // cancellation on shutdown/timeout
  settings: Readonly<Record<string, unknown>>;
}
```

### Pattern 2: Unified Item Model (store everything as "items", render by kind)

**What:** Every source normalizes into one `items` table with common columns and a typed, kind-specific `payload`. Cards are *queries* over items. The digest, notifications, and search all operate on items without knowing about Gmail or MCP.

```typescript
// shared/contracts/item.ts
interface Item {
  id: string;                    // ulid
  connectionId: string;          // which configured source (e.g. "gmail:me@x.com")
  feedId: string;
  kind: ItemKind;
  externalId: string;            // UNIQUE(connectionId, feedId, externalId)
  contentHash: string;           // detects "changed" vs "unchanged" on re-sync
  title: string;
  snippet?: string;              // short, plain text, never HTML
  url?: string;                  // deep link to the source
  author?: { name?: string; handle?: string; email?: string };
  occurredAt: string;            // email received / post time / change detected
  startsAt?: string; endsAt?: string;   // events
  importance?: number;           // 0..1, from rules + source signals (Gmail IMPORTANT, starred)
  state: { read: boolean; dismissed: boolean; pinned: boolean };
  payload: EmailPayload | EventPayload | PostPayload | PageChangePayload | ValuePayload | GenericPayload;
  schemaVersion: number;
  fetchedAt: string;
}
```

Supporting tables: `connections` (configured instances + health state), `feeds_config` (interval, enabled, card binding), `sync_cursors`, `jobs`, `cards` (layout + query + display mapping), `watch_rules`, `notifications_log`, `actions_audit`, `digests`, `secrets` (ciphertext), `settings`, `value_history` (time series for API value cards), `page_snapshots` (latest normalized text per watched page).

**Trade-offs:** Kind-specific queries (for example "unread emails from X") hit JSON payload fields. Use SQLite generated columns plus indexes for the few hot fields. Don't create per-service tables; they defeat the unified digest.

### Pattern 3: Upsert-and-Diff Ingestion, then Domain Events

**What:** Connectors *never* emit notifications directly. They return item drafts. The Item Store upserts by `(connectionId, feedId, externalId)`, compares `contentHash`, and emits `ItemCreated`, `ItemChanged`, or `ItemRemoved` events. Notification rules, UI push, and digest invalidation subscribe to those events.

**Why:** It gives one place for dedupe, which prevents duplicate toasts after a re-sync or Gmail's 404 full-resync fallback. It also gives a consistent "new vs. changed" semantic across Gmail, web pages, tweets, and API values, and makes the notification engine testable with fixtures.

**Critical detail:** Mark the *first* sync of a newly added feed as a `baseline` and emit **no notifications** for it. Otherwise connecting Gmail fires 500 toasts.

### Pattern 4: Policy-Gated Action Pipeline with Server-Issued Confirmation Tokens

**What:** All actions go through one pipeline in Core. The renderer can *request* an action but can never *force* one. Outgoing and destructive actions require a confirmation token that Core issued for the exact input the user saw.

```
UI: "Yanıtla → Gönder"
   │ actions.prepare({connectionId, actionId, input})
   ▼
Core Policy Gate
   1. validate input against ActionDecl.inputSchema (ajv)
   2. resolve effect:
        built-in   → manifest.effect
        MCP tool   → derive from annotations, but annotations are UNTRUSTED
                     (spec: "clients MUST consider tool annotations untrusted
                      unless they come from trusted servers")
                     default for any non-readOnly MCP tool: 'outgoing' (always confirm)
        user override per tool: "always ask" | "ask once per session" | "never ask" (never for outgoing)
   3. build preview (connector.preview or generic JSON-args view)
   4. if effect ∈ {external_write?, outgoing, destructive}:
        issue token = HMAC(actionId, canonicalJSON(input), expiry=2min), store pending
        return {needsConfirm: true, preview, token}
   ▼
UI Confirm Sheet: shows EXACT recipients / subject / body / event time+TZ / tool args
   │ actions.confirm({token})
   ▼
Core Executor
   5. verify token, single-use, input hash unchanged
   6. outgoing email → undo window (default 8s) held in Core queue; cancellable
   7. execute with idempotencyKey; timeout; one retry only if effect is idempotent
   8. write actions_audit (who/what/when/result, no secrets)
   9. trigger targeted re-sync of the affected feed
```

**Hard rules:**
- **No action is ever executed by a scheduled job, a notification click, or AI output in v1.** Notification buttons may *open* the reply UI; they may not send.
- **The LLM has no tool access in v1.** Email and web content are prompt-injection vectors. The digest model reads text and writes text, nothing else.
- MCP's `tools/call` input must be shown to the user before calling. The MCP spec says clients SHOULD "show tool inputs to the user before calling the server" and "prompt for user confirmation on sensitive operations" ([MCP tools spec](https://modelcontextprotocol.io/specification/latest/server/tools)).

### Pattern 5: MCP Server = Connection; Resources/Tools to Feeds/Actions via the MCP Adapter

**What:** An MCP server is configured as a *connection* and wrapped by `McpConnector`, which implements the same `Connector` interface. The user, not the server, decides which MCP primitives become cards. Automatic inference only proposes options.

| MCP primitive | Mornfold concept | How |
|---------------|------------------|-----|
| `resources/list` + `resources/read` | **Feed** | User picks a resource and polls it. JSON content goes through the extraction mapping to items; text/markdown becomes one `document` item. Honor `ttlMs` from `CacheableResult` (2026-07-28 spec) as a minimum poll interval. |
| Resource update notifications | **Push feed** | 2026-07-28 servers use `client.listen({ resourceSubscriptions: [...] })`; 2025-era servers use `subscribeResource`. The notification triggers an immediate re-read. Fall back to polling when it is not supported. |
| Tool with `readOnlyHint: true` **and** user-approved for polling | **Polled feed** ("query tool") | User sets fixed args and an interval in the wizard. `structuredContent` (validated against `outputSchema`) or the JSON text goes through the extraction mapping to items. |
| Any other tool | **Action** | `inputSchema` generates the form. The effect defaults to `outgoing`, so confirmation is always required unless the user explicitly downgrades it for a non-outgoing tool. |
| Tool `inputSchema` | Action form | JSON Schema to a generated form (flat-object subset, with a JSON editor fallback for complex schemas). |
| `InputRequiredResult` / elicitation (MRTR) | Inline form or browser hand-off | Form mode renders a generated form; URL mode opens the browser via the OS Bridge, then the request is retried with `inputResponses`. |
| `tools/list_changed` | Re-list + diff | New tools appear as *disabled* until the user reviews them. This prevents silent capability changes ("rug pull"). |
| Prompts | Ignored in v1 | Possible future: digest templates. |
| Sampling, Roots, Logging | **Do not implement** | Deprecated in the 2026-07-28 spec ([changelog](https://modelcontextprotocol.io/specification/2026-07-28/changelog)). |
| MCP Apps (`ui://` HTML) | **Defer to v2+** | Rendering would require a sandboxed iframe host (AppBridge or `@mcp-ui/client`) and would break visual consistency. Note it as a future "rich card" type. |

**Naming:** Namespace everything as `mcp:<connectionId>/<toolName>`. The spec warns that `serverInfo.name` is not unique and that clients aggregating multiple servers SHOULD disambiguate.

**Protocol-era compatibility (important, verified 2026-10):** MCP changed fundamentally on 2026-07-28. It is now stateless: no `initialize` handshake, no sessions, per-request `_meta`, `server/discover`, `subscriptions/listen`, and MRTR replacing server-initiated requests. TypeScript SDK v2 (`@modelcontextprotocol/client` 2.3.x) implements it. Most MCP servers users will install in the near term are still 2025-era. The v2 client defaults to the legacy `initialize` handshake, and `versionNegotiation: { mode: 'auto' }` also accepts 2026-era discovery. **Use auto mode and test against both eras.** Confidence: MEDIUM on the exact option shape, HIGH that both eras must be supported.

**Lifecycle:**
- stdio servers are spawned lazily on first use and shut down after N minutes idle, *unless* they back an active push feed.
- On Windows, spawn with `windowsHide: true` and kill the whole process tree on shutdown, because orphaned `npx`/`uvx` processes are common.
- HTTP servers get an `OAuthClientProvider` implementation that stores tokens in the Secrets Vault, keyed by issuer (an SDK and spec requirement).

### Pattern 6: Shared Extraction Engine ("URL/tool + field path to card")

**What:** One component turns arbitrary JSON into items and card display fields. The custom API wizard, the free-API catalog, and the MCP card wizard all use it.

```jsonc
// catalog/exchange-rates.json  (also the shape a user-defined API produces)
{
  "id": "catalog.frankfurter.usdtry",
  "title": "USD/TRY",
  "request": {
    "method": "GET",
    "url": "https://api.frankfurter.app/latest?from=USD&to=TRY",
    "headers": {},                       // may reference {{secret:apiKey}}, resolved in Core only
    "intervalSec": 900
  },
  "extract": {
    "mode": "single",                    // "single" → value card; "list" → list card
    "itemsPath": null,                   // for list mode: JSONPath to the array
    "fields": {
      "value":    "$.rates.TRY",
      "subtitle": "$.date"
    },
    "format": { "value": { "type": "number", "decimals": 2, "suffix": " ₺" } }
  },
  "card": { "type": "value", "sparkline": true },
  "alerts": [{ "when": "value < 30", "notify": true }]   // simple comparison DSL, NOT eval
}
```

**Rules:**
- The wizard flow is: fetch a sample, show a JSON tree, let the user *click* the field, and generate the path. Users should never type JSONPath.
- Evaluate paths with a library configured for **no script evaluation**. JSONPath implementations have had RCE CVEs via filter-expression eval. Alert conditions use a tiny comparison grammar, never `eval`/`Function`.
- Store each fetch's value in `value_history` for sparklines and threshold-crossing detection (notify on *crossing*, not on every poll below the threshold).

### Pattern 7: Persistent Scheduler with Per-Host Budgets

**What:** A `jobs` table, not `setInterval` per watcher.

```
jobs(id, connection_id, feed_id, kind, interval_sec, next_run_at, last_run_at,
     last_status, consecutive_failures, backoff_until, paused)
```

- **One loop:** sleep until `min(next_run_at)`, then dispatch due jobs into a `p-queue` with global concurrency (about 4) and **per-host** concurrency/token buckets (Gmail, X, and a user's own tiny API server each get their own budget).
- **Jitter** of ±10% on intervals, so 20 watchers don't fire in the same second.
- **Backoff:** exponential (×2, capped at 1h). Honor `Retry-After` and 429s. `AuthRequired` errors move the connection to `auth_required` and stop the job (no retry storm). The UI surfaces a "Yeniden bağlan" chip.
- **Resume after sleep:** run each overdue job once (coalesced), staggered over about 30s.
- **Offline:** pause network jobs while offline (signal from the renderer's `navigator.onLine` or a periodic probe) to avoid backoff escalation during Wi-Fi loss.
- **Digest job:** a cron-style daily time plus a "first unlock after time T" trigger. It runs a fresh sync of digest-relevant feeds first, then builds the digest.

### Pattern 8: Secrets Vault (envelope encryption, never crosses to the UI)

**What:**
- A random 256-bit **master key** is stored as one generic credential in **Windows Credential Manager** (e.g. `Mornfold/master-key`) via `@napi-rs/keyring`. The key is protected by the user's Windows login (DPAPI under the hood).
- All secrets (API keys, OAuth refresh/access tokens, MCP tokens, HTTP header secrets) are stored as **AES-256-GCM ciphertext in SQLite**, under the namespace `connectionId/secretName`.

**Why envelope encryption instead of one Credential Manager entry per secret:**
- Credential Manager blobs are capped at 2560 bytes, and OAuth token sets can exceed that (Docker had to chunk tokens for this reason).
- Users can browse entries in the Credential Manager UI.
- Enumeration and cleanup are cleaner.
- Backup/export can be explicit.

**Fallback:** Electron `safeStorage` (DPAPI on Windows) is *main-process only*. If the keyring fails, Core asks Main via the OS Bridge to wrap/unwrap the master key with `safeStorage`.

**Boundary rules:**
- The renderer API is write-only: `secrets.set(ref, value)`, `secrets.has(ref)`, `secrets.delete(ref)`. There is no `get`.
- Connectors get a `ScopedSecrets` limited to their own connection namespace.
- The logger redacts known secret values and `Authorization` headers.

### Pattern 9: Digest Pipeline with Deterministic Fallback

```
collect (today's events, unread important email, overnight page changes,
         X matches, API alerts, MCP feed items)
  → rank (rules: importance, recency, sender allowlist, meeting soon)
  → compact context (strip quoted replies/signatures, cap per-item chars,
                     cap total tokens, item IDs as refs)
  → provider.generate (AI SDK; Anthropic | Google | OpenRouter; user key)
      structured output: { headline, sections[{title, bullets[{text, itemRefs[]}]}] }
  → validate (zod) → store digests row → DigestReady event
  ↘ on no key / error / timeout: deterministic template digest from the ranked list
```

- The digest **always renders**, with or without AI. This satisfies "even if everything breaks".
- Output references item IDs, so every bullet is clickable and verifiable (and hallucinations are visible).
- The prompt and output language is Turkish by default (taken from the UI locale).
- Email and web text is wrapped in clear "untrusted content" delimiters. The model has no tools.

---

## Data Flow

### Ingestion Flow (background, every N minutes)

```
Scheduler (job due)
   ↓
Connector Host ── builds ConnectorContext (scoped secrets, rate-limited http, oauth)
   ↓
Connector.sync(feed, cursor)  ──►  external API / MCP server / web page
   ↓ ItemDraft[] + new cursor
Item Store: upsert → dedupe → contentHash diff → persist cursor (same transaction)
   ↓ ItemCreated / ItemChanged / ItemRemoved   (suppressed during baseline sync)
Domain Event Bus
   ├──► Notification Engine → rules → dedupe/throttle → OS Bridge (Main) → Windows toast
   ├──► RPC push → Renderer → invalidate TanStack Query keys → cards re-render
   └──► Digest: mark stale (re-generate only on demand / schedule)
```

### User Action Flow (e.g. reply to an email)

```
Renderer (Reply sheet)
   ↓ actions.prepare
Core Policy Gate → validate → classify → preview → confirmation token
   ↓
Renderer Confirm Sheet (exact recipients/body) → user taps "Gönder"
   ↓ actions.confirm(token)
Core Executor → undo window (8s) → Gmail API send → audit log → targeted re-sync
   ↓ ActionCompleted event
Renderer toast "Gönderildi" (with undo during window)
```

### Notification Click Flow

```
Windows toast click (app running)      → Main 'click' → focus window → deep link mornfold://item/<id>
Windows toast click (app NOT running)  → app launches → Notification.handleActivation → same deep link
   ↓
Renderer router → item detail (no action is executed by the click itself)
```

### OAuth Flow (Google and MCP HTTP servers)

```
Renderer "Google ile bağlan" → Core OAuth Service
   → start ephemeral listener http://127.0.0.1:<random port>/callback  (Google: loopback IP
     is the supported desktop method; custom URI schemes and OOB are NOT supported)
   → PKCE (S256) + state → OS Bridge shell.openExternal(authUrl) → system browser
   → callback: verify state (and `iss` for MCP per RFC 9207) → exchange code → Secrets Vault
   → close listener → connection state = healthy → schedule baseline sync
```

### State Management (Renderer)

```
Core (source of truth, SQLite)
   ↓ RPC query                    ↑ RPC command (prepare/confirm/updateSettings)
TanStack Query cache (renderer) ←─ invalidation on pushed domain events
   ↓
Card components (pure, by card type)
```

The renderer holds only UI state (layout drag state, open sheets). Everything else is a cached query and can be thrown away at any time, which is what makes destroying the window safe.

### Key Data Flows

1. **Sync to card:** Scheduler, connector, item upsert, event, UI invalidation. Latency target: under 1s from fetch to the card updating.
2. **Change to toast:** Item diff, rule match, throttle (batch more than 3 per source per 2 min into one "5 yeni e-posta" toast), toast.
3. **Morning digest:** Time/unlock trigger, pre-sync, rank, LLM (or fallback), Today screen and one summary toast.
4. **Action:** Prepare, confirm (token), undo window, execute, audit, re-sync.
5. **MCP discovery:** Connect, list tools/resources, user maps to feeds/actions in the wizard, persist the mapping, scheduler jobs created.

---

## Scaling Considerations

Mornfold is single-user and local, so "scale" means **data volume, number of sources, and machine resources**, not users.

| Scale | Architecture adjustments |
|-------|--------------------------|
| 1 user, ~5 sources, ~10k items | Defaults are fine: SQLite WAL, one Core process, polling at 1–15 min. |
| 20+ sources, 10+ MCP servers, ~500k items | Retention policy (e.g. keep email items 30 days, page snapshots latest + 20 diffs, value history downsampled). Lazy and idle-shutdown MCP processes. FTS5 index maintenance. |
| Heavy web watching (50+ pages, JS-rendered) | RenderFetch pool capped at 1–2 windows; prefer the plain HTTP fetch path; per-domain minimum intervals. |

### Scaling Priorities

1. **First bottleneck: idle resource usage.** It is a tray app that runs all day. A hidden renderer plus Core plus N MCP processes (each `npx` server is a full Node process at roughly 40–80 MB) adds up quickly. Fix with lazy MCP spawn plus idle shutdown, window destruction after a long hide, and no polling under 60s by default.
2. **Second bottleneck: API rate limits and costs** (X API especially, plus LLM tokens). Fix with per-host budgets in the scheduler, incremental cursors (Gmail `historyId`, Calendar `syncToken`, HTTP `ETag`/`If-Modified-Since`), and digest caching (regenerate only when inputs change materially).
3. **Third: DB growth.** Fix with a retention job, storing email bodies lazily (fetch the full body on open, cache with LRU), and avoiding raw payload blobs.

---

## Anti-Patterns

### Anti-Pattern 1: Business logic in Main or Renderer

**What people do:** Run sync and MCP in Electron main ("it's just Node"), or call Google APIs from the renderer.
**Why it's wrong:** A slow sync or a sync-heavy JSON parse blocks Main, which freezes the tray, all windows, and notifications. Renderer-side fetching requires exposing tokens to the UI.
**Do this instead:** Main does OS integration only. Everything else lives in the Core utilityProcess.

### Anti-Pattern 2: Loading third-party JS plugins in-process

**What people do:** Build a "plugins folder" where `.js` files are `require()`d.
**Why it's wrong:** Any plugin can read every secret and the whole DB, and a supply-chain compromise becomes total compromise.
**Do this instead:** Third-party code arrives only as MCP servers (process-isolated, talking through a protocol you mediate). Simple integrations arrive as declarative JSON.

### Anti-Pattern 3: Trusting MCP tool annotations or auto-executing tools

**What people do:** Skip confirmation when `destructiveHint: false`, or let the LLM call MCP tools for the digest.
**Why it's wrong:** The spec says annotations are untrusted. A malicious or buggy server can label `send_payment` as read-only. Letting the LLM call tools turns every email into a prompt-injection attack surface.
**Do this instead:** Default every non-read-only MCP tool to "confirm with full args preview". Treat `readOnlyHint` only as a *suggestion* in the wizard for which tools could be polled; the user approves. The LLM gets no tools in v1.

### Anti-Pattern 4: Plugin-provided UI

**What people do:** Let each connector or MCP server render its own HTML card.
**Why it's wrong:** It destroys the Apple/Linear aesthetic (the product's main differentiator) and creates an XSS and exfiltration surface.
**Do this instead:** Use a fixed card library (list, agenda, value + sparkline, timeline, diff, digest) and have sources map data into it. MCP Apps can come later as an explicitly sandboxed "rich card" type.

### Anti-Pattern 5: Rendering email HTML in the app DOM

**What people do:** `dangerouslySetInnerHTML` the email body, or load it in a webview with the preload attached.
**Why it's wrong:** Script and CSS injection, tracking pixels phoning home (a privacy violation for a privacy-first app), and potential access to the RPC bridge, which means the ability to request "send email".
**Do this instead:** Use a sandboxed `<iframe sandbox>` *without* `allow-scripts`/`allow-same-origin`, sanitize (DOMPurify), apply a strict CSP, and block remote images by default with a "Görselleri göster" toggle. Combined with server-issued confirmation tokens, even a compromised renderer cannot silently send.

### Anti-Pattern 6: `setInterval` per watcher and notifying from connectors

**What people do:** Each watcher owns a timer and calls `new Notification()` when it sees something new.
**Why it's wrong:** This loses state on restart, produces a thundering herd after sleep, gives duplicate toasts after a re-sync, has no global rate limiting, and can't be tested.
**Do this instead:** Use the persistent scheduler (Pattern 7) and upsert-diff-events (Pattern 3). Only the Notification Engine notifies.

### Anti-Pattern 7: One Credential Manager entry per secret, or secrets in settings JSON

**What people do:** `keytar.setPassword()` for every token, or keep API keys in `config.json` / `localStorage`.
**Why it's wrong:** It hits the 2560-byte blob limit with OAuth token sets. Plaintext files get synced and backed up. `keytar` is archived.
**Do this instead:** Use envelope encryption (Pattern 8).

### Anti-Pattern 8: Multiple processes writing SQLite

**What people do:** Open the DB from Main *and* Core (or from the renderer through IPC shortcuts).
**Why it's wrong:** Lock contention, `SQLITE_BUSY`, and migrations racing at startup.
**Do this instead:** Core is the single owner and writer. Everyone else goes through RPC.

### Anti-Pattern 9: "First sync = new items"

**What people do:** Treat everything fetched on the first connection as new.
**Why it's wrong:** Connecting Gmail produces 500 notifications, and adding a web watcher produces a "changed" toast immediately.
**Do this instead:** Baseline mode on first sync and after any full resync (e.g. after Gmail returns 404 for a stale `historyId`).

---

## Integration Points

### External Services

| Service | Integration pattern | Notes |
|---------|---------------------|-------|
| **Gmail API** | Poll `history.list` from the stored `historyId` (1–2 min). Full sync on first connect and on a 404. | History is kept for "at least one week" but can be shorter, so the 404 fallback is mandatory ([Gmail sync guide](https://developers.google.com/workspace/gmail/api/guides/sync)). Push needs Cloud Pub/Sub, which a no-server desktop app can't reasonably own, so poll. Gmail scopes are *restricted*: verification and possibly a security assessment are needed for public distribution (see PITFALLS.md). |
| **Google Calendar API** | Incremental sync with `syncToken`. Handle `410 Gone` with a full resync. `events.insert` is an action. | Use explicit timezone fields on create. Inviting attendees is `outgoing` (sends emails), while creating a solo event is `external_write`. |
| **Google OAuth** | Loopback `http://127.0.0.1:<random>` plus PKCE S256 in the system browser | Custom URI schemes and OOB are not supported for desktop clients. The desktop client secret is not confidential ([Google native-app OAuth](https://developers.google.com/identity/protocols/oauth2/native-app)). |
| **MCP servers (stdio)** | Spawned by Core via the SDK's `StdioClientTransport` (`cross-spawn`) | Users may lack Node/Python (`npx`/`uvx`). Consider bundling execution with Electron's own Node via `ELECTRON_RUN_AS_NODE` for JS servers (LOW confidence; validate). Use `windowsHide` and tree-kill. |
| **MCP servers (HTTP)** | `StreamableHTTPClientTransport` plus `OAuthClientProvider` backed by the vault; loopback callback | Validate `state` yourself (the SDK does not). `finishAuth` checks `iss`. Key client registrations by issuer. |
| **AI providers** | AI SDK (`ai`) with `@ai-sdk/anthropic`, `@ai-sdk/google`, `@openrouter/ai-sdk-provider` | One `generateObject`-style call per digest. Show per-provider errors clearly (invalid key, quota). |
| **X / Twitter** | Built-in connector behind the same interface; strategy TBD in FEATURES/STACK research | Treat as the highest-cost, lowest-reliability source. Strict scheduler budget. |
| **Arbitrary web pages** | HTTP fetch, then CSS-selector extraction, normalization, hashing, and a text diff. RenderFetch (Main) handles JS sites. | Normalize away timestamps, counters, and rotating tokens, or it will "change" every poll. Respect robots.txt and per-domain minimum intervals. |
| **Windows notifications** | Electron `Notification` from Main; `Notification.handleActivation` for cold-start clicks | Requires `app.setAppUserModelId()` matching the installer's Start Menu shortcut, or toasts silently fail. Verify on Windows CI/VM. |

### Internal Boundaries

| Boundary | Communication | Notes |
|----------|---------------|-------|
| Renderer ↔ Core | MessagePort (`MessageChannelMain`, port transferred by Main to both sides); typed RPC + event stream | Main is not in the hot path. zod-validate both directions. |
| Renderer ↔ Main | Minimal `ipcRenderer.invoke` for window chrome only (minimize, hide, theme) | contextIsolation on, sandbox on, nodeIntegration off; validate `event.senderFrame`. |
| Core ↔ Main | `process.parentPort` RPC: `notify`, `openExternal`, `renderFetch`, `safeStorage` fallback, power events, tray badge | Main restarts Core if it exits; Core re-requests OS state on boot. |
| Core ↔ MCP server | MCP over stdio / Streamable HTTP | One `Client` per connection; per-call timeouts; output size caps. |
| Connector ↔ Core internals | Only via `ConnectorContext` | No direct DB access, no cross-connection secrets. |
| Event Bus ↔ subscribers | In-process typed emitter | Subscribers must not throw into the publisher; errors are logged per subscriber. |

---

## Suggested Build Order (dependencies)

```
[0] Shell skeleton + Windows CI ──┬──► [1] Core data foundation (DB, items, vault, RPC)
                                  │            │
                                  └──► [2] Design system + card library (fixtures) ─┐
                                               │                                    │
                                     [3] Connector SDK + Scheduler + Notify engine  │
                                         + HTTP-API engine/catalog (no OAuth)  ◄────┘
                                               │
                         ┌─────────────────────┼──────────────────────┐
                         ▼                     ▼                      ▼
              [4] OAuth + Google read   [6] MCP client + adapter   [7] Web watch (+RenderFetch)
                         │                     │                      and X tracking
                         ▼                     │
              [5] Action pipeline + Gmail reply/Calendar create ◄─────┘ (MCP actions reuse it)
                         │
                         ▼
              [8] AI digest + Today screen (needs ≥2 real sources)
                         │
                         ▼
              [9] Onboarding wizard, packaging/signing, auto-update, distribution hardening
```

| # | Phase | Why here | Depends on |
|---|-------|----------|------------|
| 0 | **Shell skeleton**: Electron + 3 processes + MessagePort RPC + tray + single instance + login item + installer built on **Windows CI** | Proves the riskiest platform pieces (tray, toast with AUMID, utilityProcess, native module in Core) on real Windows before features pile on. Dev is on Linux. | — |
| 1 | **Core data foundation**: SQLite + migrations, Item model, event bus, Secrets Vault (Credential Manager), settings | Every later component writes items or secrets. | 0 |
| 2 | **Design system + card library** with fixture data | Can run parallel to 1. The aesthetic is the differentiator and should be settled before real data shapes harden. | 0 |
| 3 | **Connector SDK + Scheduler + Notification engine**, proven with the **HTTP-API engine + catalog** | Exercises the whole pipeline (schedule, sync, upsert/diff, event, card, toast) with zero OAuth complexity. It also delivers the "add a service without code" requirement early. | 1, 2 |
| 4 | **OAuth service + Gmail/Calendar read** | First high-value built-ins. Starts the Google verification clock early. | 3 |
| 5 | **Action pipeline** (policy gate, confirmation tokens, undo window, audit) + Gmail reply + Calendar create | Safety infrastructure must exist before any write action. Built-ins first, because their effects are known. | 4 |
| 6 | **MCP client manager + adapter + MCP wizard** (stdio + HTTP + OAuth) | Reuses the connector contract (3), extraction engine (3), OAuth (4), and action pipeline (5). Can start in parallel with 5 for read-only feeds. | 3, 4 (5 for MCP actions) |
| 7 | **Web change watcher + X tracking** | Independent built-ins on the proven SDK. X depends on the API-strategy decision from research. | 3 |
| 8 | **AI digest + Today screen** | Needs real items from at least Calendar and Gmail to be meaningful and tunable. The deterministic fallback ships first within the phase. | 4 (ideally 6, 7) |
| 9 | **Onboarding, packaging, code signing, auto-update, Google verification, i18n scaffolding** | Required before distributing to others. Signing and notification identity were already proven in phase 0. | all |

**Ordering rationale:**
- The core value is "shell + connection infrastructure works even if everything else breaks", so phases 0–3 build exactly that before any single integration.
- The action safety pipeline (5) is a hard prerequisite for *any* write action, built-in or MCP. Never ship Gmail reply "temporarily" without it.
- The AI digest is the wow feature but the *last consumer* in the data flow. Building it before real sources means tuning prompts on fake data.
- The MCP adapter goes after the built-ins so the `Connector` interface is shaped by two real implementations (HTTP-API and Google) before it is generalized to MCP.

**Research flags for phases:**
- **Phase 0:** Windows-only behaviors (AUMID/toasts, Credential Manager from utilityProcess, native modules in utilityProcess, tray icon DPI). This phase needs Windows CI and VM verification, not just research.
- **Phase 4:** Google restricted-scope verification path and the testing-mode refresh-token expiry. Needs deeper research.
- **Phase 6:** MCP era compatibility (2025 vs. 2026-07-28 servers), Windows stdio quirks, and bundling Node for `npx` servers. Needs deeper research.
- **Phase 7:** X API strategy and cost. Needs deeper research (owned by FEATURES/STACK).
- **Phases 1, 2, 3, 5:** standard patterns, unlikely to need extra research.

---

## Sources

- MCP specification 2026-07-28, Tools (annotations untrusted, human-in-the-loop SHOULDs, structuredContent/outputSchema, name disambiguation): https://modelcontextprotocol.io/specification/latest/server/tools (HIGH)
- MCP 2026-07-28 changelog (stateless, `server/discover`, `subscriptions/listen`, MRTR, deprecations of Sampling/Roots/Logging, `ttlMs`): https://modelcontextprotocol.io/specification/2026-07-28/changelog (HIGH)
- MCP schema.ts, `ToolAnnotations` defaults (destructiveHint default true, openWorldHint default true): https://github.com/modelcontextprotocol/modelcontextprotocol/blob/main/schema/2026-07-28/schema.ts (HIGH)
- MCP TypeScript SDK v2: package `@modelcontextprotocol/client` 2.3.1 (npm, published 2026-10-05; engines node>=20; ships both 2025-11-25 and 2026-07-28 protocol constants) (HIGH)
- SDK v2 client guides, connect/versionNegotiation, OAuth provider, subscriptions: https://ts.sdk.modelcontextprotocol.io/v2/clients/connect, /v2/clients/oauth, /v2/clients/subscriptions (MEDIUM; fetched summaries)
- MCP Apps extension: https://modelcontextprotocol.io/docs/extensions/apps (HIGH, for the deferral decision)
- Electron utilityProcess: https://www.electronjs.org/docs/latest/api/utility-process (HIGH)
- Electron safeStorage (DPAPI on Windows, main-process module): https://www.electronjs.org/docs/latest/api/safe-storage (HIGH)
- Electron Notification (Windows `toastXml`, `handleActivation`): https://www.electronjs.org/docs/latest/api/notification (HIGH)
- Electron releases (44.9.0 stable, Node 24.22): https://releases.electronjs.org/releases.json (HIGH)
- Google OAuth for desktop apps (loopback IP, PKCE, no custom schemes/OOB): https://developers.google.com/identity/protocols/oauth2/native-app (HIGH)
- Gmail sync guide (historyId, 404 → full sync): https://developers.google.com/workspace/gmail/api/guides/sync (HIGH)
- Credential Manager 2560-byte blob limit, chunking workarounds: Docker secrets-engine PR #474, Go `wincred` README, Chilkat example (MEDIUM; multiple secondary sources agree)
- Package versions verified via npm on 2026-10-18: electron 44.9.0, better-sqlite3 13.0.3, @napi-rs/keyring 2.1.1, ai 7.0.143, @ai-sdk/anthropic 4.0.82, @ai-sdk/google 4.0.99, @openrouter/ai-sdk-provider 3.1.0, croner 10.0.1, p-queue 10.0.0, ajv 8.20.0, zod 4.6.7 (HIGH)

---
*Architecture research for: local-first Windows command center with plugin/MCP integration*
*Researched: 2026-10-18*
