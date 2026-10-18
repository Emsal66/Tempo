# Feature Research

**Domain:** Local-first personal command center / "everything on one screen" desktop dashboard (Windows) with an AI morning briefing, Google (Gmail + Calendar) integration, an MCP-based plugin model, and background monitoring (website changes, X/Twitter, free APIs)
**Researched:** 2026-10-18
**Confidence:** MEDIUM overall. Official docs were checked for X API pricing, the MCP spec, Google restricted-scope verification, OpenRouter usage accounting, Distill, changedetection.io, Glance and Homepage. Competitor UX patterns for Raycast, Sunsama, Akiflow, Shift, Station, Rambox and Home Assistant come mostly from training data plus comparison articles, so treat those as MEDIUM/LOW.

---

## How to Read This Document

PROJECT.md says v1 must ship all monitoring features ("Tüm takip özellikleri v1'de"). So "table stakes" here means **the minimum depth each of those features needs before it stops feeling broken**. It does not mean the whole category is optional. Each category below has its own table-stakes, differentiator and anti-feature lists. Consolidated MVP and priority views come at the end.

Complexity key: **LOW** (days), **MEDIUM** (1–2 weeks), **HIGH** (multi-week, or has external/approval risk).

---

## Feature Landscape

### 1. App Shell & Dashboard

Reference products: Raycast (command palette, keyboard-first), Linear (Cmd/Ctrl+K, polish), Sunsama and Akiflow (daily planning home screen), Notion Calendar, Home Assistant dashboards (sections and cards), Glance, Homepage and Dashy (YAML-defined widget grids).

#### Table Stakes

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| "Günün özeti" home screen built from deterministic cards (today's meetings, important or unread mail, monitor alerts, pinned API cards) | It is the core promise. Glance, Homepage, Sunsama and Notion Calendar all open on a "today" view. | MEDIUM | **Must render without AI.** The AI narrative sits on top of the cards and never replaces them. Core Value says the shell has to work even when other parts break. |
| Card/widget system with a common card contract: title, icon, loading, empty, error and stale states, last-updated timestamp, refresh button | Every dashboard product has this. Without consistent states, broken connectors make the whole screen look broken. | MEDIUM | Gmail, Calendar, monitors, API cards, MCP cards and the briefing all go through this contract. Build it first. |
| Layout control: reorder cards by drag, 2–3 size presets (S/M/L), hide/show, sensible default layout | Home Assistant "sections", Glance columns and Homepage groups all give some layout control. | MEDIUM | Use a column/section grid with size presets. Avoid a free-form pixel canvas (see anti-features). |
| Command palette (Ctrl+K): navigate, run actions (compose mail, new event, add monitor, refresh all, open settings), search cards and items | Expected in any app aiming for the Linear/Raycast feel. Akiflow is known for its command bar. | MEDIUM | An action registry that connectors can contribute to. MCP tools can be exposed later. |
| Keyboard shortcuts (palette, refresh, navigate sections, Esc to close panels) | Part of the "Linear" bar. | LOW | Show the shortcuts in the palette next to each action. |
| First-run onboarding wizard: language, AI provider and key (with a "test key" button), connect Google, choose starter cards, notification and start-at-login preferences | PROJECT.md: others will install it with their own keys and OAuth. Every BYO-key product needs this. | MEDIUM | Every step must be skippable, and the app must stay usable with nothing connected. Resumable from settings. |
| Empty states that teach ("No meetings today", "Connect Google to see mail") | Polish expectation. Empty widgets look broken. | LOW | Design work rather than engineering. |
| Light/dark theme following Windows, with glass/Mica-style surfaces | Apple/Linear aesthetic is the stated differentiator. Windows 11 users expect Mica/acrylic. | MEDIUM | Check the performance cost of blur on low-end GPUs and provide a "reduce transparency" fallback. |
| Turkish UI with an i18n-ready string layer | PROJECT.md: Turkish UI, multi-language later. | LOW | Externalize strings from day one. Retrofitting i18n is expensive. Use Turkish date/number formatting (24h clock, dd.MM). |
| Global "last sync / offline" indicator | Local-first apps go offline. Users need to know when data is stale. | LOW | |

#### Differentiators

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| Global hotkey to summon Mornfold or the palette from anywhere in Windows (Raycast-style) | Makes it a real command center and not just another window | MEDIUM | Needs care around conflicts with existing Windows shortcuts. |
| Time-aware home: the morning shows the briefing, during the day "next meeting in 12 min" and new alerts, the evening shows "what happened today" | Sunsama's planning and shutdown rituals, done passively | MEDIUM | Can start as layout presets keyed to time of day. |
| Motion and micro-interactions (card enter animations, smooth refresh, skeleton loaders) | The "hayran bırakan" ("wow") factor is the product's stated differentiator | MEDIUM | Respect Windows "show animations" and reduced-motion settings. |
| Multiple dashboards or pages (Home, Monitoring, Finance) | Glance pages, HA views | LOW–MEDIUM | v1.x. A single home page plus detail pages is enough for v1. |
| "Focus" item detail panel (slide-over) used by every card (mail thread, event, diff, tweet) | Keeps users in context instead of opening new windows | MEDIUM | One shared panel component reused by all connectors. |

#### Anti-Features

| Feature | Why Requested | Why Problematic | Alternative |
|---------|---------------|-----------------|-------------|
| Free-form pixel canvas (place widgets anywhere, arbitrary sizes) | "Total freedom" | Breaks the curated aesthetic, makes responsive layout and DPI scaling painful, and adds a lot of layout-engine work | Section grid with S/M/L presets and drag reorder |
| YAML/JSON config as the primary way to build the dashboard (Glance/Homepage style) | Power users like it | Bad for "others will download it". Conflicts with the Apple-like UX. | GUI-first. Offer JSON import/export of the layout and card definitions as a power-user extra. |
| User-supplied CSS/theme injection | Customization | Untestable visual regressions, and it undercuts brand polish | 2–3 curated accent colors plus light/dark |

---

### 2. Connector / Plugin Management

Reference products: Claude Desktop (Settings > Extensions, `.mcpb` one-click bundles, `mcpServers` JSON config), Raycast Store, Station/Shift/Rambox (app list with per-app status and notifications), Home Assistant integrations page (status, reconfigure, "repair" prompts), Zapier/Make connection pages.

#### Table Stakes

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| "Connections" page listing every connector with a status badge: Connected / Needs re-auth / Error / Rate-limited / Paused / Disabled | Every integration hub shows this (HA integrations, Zapier). Without it, failures are invisible. | MEDIUM | Status comes from a shared connector health model. |
| Per-connector details: account identity (which Google account), scopes or permissions granted, last successful sync, next scheduled sync, last error message in plain language | Trust and debuggability | LOW–MEDIUM | |
| Connect / reconnect / disconnect, and on disconnect, delete that connector's local data and its keys from Credential Manager | Privacy promise in PROJECT.md | MEDIUM | "Disconnect" has to actually purge data. Ask for confirmation. |
| "Test connection" button for every credential (AI keys, X key, custom API auth, MCP server) | Users paste keys wrong. Immediate feedback is expected. | LOW | |
| Built-in OAuth via the system browser with loopback redirect (no embedded webview login) | Google blocks OAuth in embedded webviews. Users expect "Sign in with Google" in the browser. | MEDIUM | Authorization Code + PKCE, loopback `127.0.0.1` redirect. Refresh tokens stored in Credential Manager. |
| Add an MCP server: (a) local stdio (command, args, env vars, with secret env vars stored in the vault) and (b) remote Streamable HTTP URL with OAuth | PROJECT.md's extensibility promise. Claude Desktop, Cursor and VS Code all support both. | HIGH | Current MCP spec is **2026-07-28**. Remote auth follows its authorization section (protected resource metadata, RFC 8707 `resource` param, Client ID Metadata Documents preferred over Dynamic Client Registration). Needs phase-specific research. |
| Paste or import a Claude-Desktop-style `mcpServers` JSON block | That is how MCP servers are documented across the ecosystem. Users copy READMEs. | LOW | Big UX win for little cost. Optionally offer one-click import from an existing Claude Desktop config on the same PC. |
| Per-MCP-server tool list with enable/disable per tool | Users need to see what a server can do. The MCP spec says hosts must get explicit consent before invoking tools. | MEDIUM | Default write-like tools to "ask every time". |
| Server process supervision: auto-start, restart with backoff on crash, "view logs" (stderr capture) | Local MCP servers crash. Without logs, users are stuck. | MEDIUM | Show "crashed 3× — paused" instead of looping forever. |
| Rate-limit and quota handling: backoff on 429, show "rate limited, retrying at 14:05", and for paid APIs (X, AI) show usage and spend | X and AI are pay-per-use under the user's key. Surprise bills kill trust. | MEDIUM | See the X and AI sections for spend caps. |
| Global "pause all background activity" | Users want a kill switch (metered network, presenting, travel) | LOW | Also lives in the tray menu. |

#### Differentiators

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| **MCP-backed cards**: pick an MCP server and tool, set fixed arguments and a refresh interval, and render the result (text, markdown, or JSON mapped like a custom API card) | This turns MCP from "tools for an LLM chat" into "data for the dashboard", which is the "add a service without touching core code" promise. No mainstream MCP client does this. | HIGH | MCP tool results are untyped text/JSON, so reuse the custom-API field mapper. The MCP "Tasks" and "MCP Apps" extensions are worth watching for v2. |
| MCP context in the AI briefing: the user marks which MCP resources or tools feed the briefing ("include GitHub notifications") | Briefing grows as plugins are added | HIGH | Tool calls during briefing generation need consent rules. Start with read-only, pre-approved tools. |
| Curated "recommended connectors" gallery (GitHub, Notion, Todoist, finance) with prefilled MCP configs | Lowers the barrier for non-technical users (the "others will download it" future) | MEDIUM | A static JSON catalog shipped with the app or fetched from a public repo. No server needed. |
| `.mcpb` bundle install (drag-drop a bundle, fill in a settings form from the manifest) | Matches Claude Desktop's one-click experience | MEDIUM | v1.x. The manifest format is public. |
| Health timeline per connector (sparkline of successes and failures over 7 days) | Polish and trust | LOW | |

#### Anti-Features

| Feature | Why Requested | Why Problematic | Alternative |
|---------|---------------|-----------------|-------------|
| Native code plugin system (DLLs, arbitrary JS plugins loaded into the app) | "Real" extensibility | A large security surface and a versioned plugin API to maintain. MCP already solves this with process isolation. | MCP servers plus declarative custom API definitions as the only extension points |
| Hosted plugin marketplace with accounts and reviews | App-store feel | Needs a server, which breaks the local-only principle | Static, signed catalog JSON in a public GitHub repo |
| Auto-running MCP tools without consent ("agent mode") | Convenience | The MCP spec requires explicit consent before tool invocation. Prompt-injection risk via mail and web content. | Per-tool allowlist, with read-only tools pre-approvable |
| Embedding full web apps of services (Rambox/Shift/Station style tabs) | "Everything in one app" | Turns the product into a browser shell, with heavy memory use and no real integration | Native cards plus "open in browser" deep links |

---

### 3. Google Gmail & Calendar Inside the Dashboard

Reference products: Notion Calendar, Morgen, Akiflow (calendar with quick add), Superhuman and Shortwave (fast triage, AI drafts), Google's own CC agent (daily "Your Day Ahead" email built from Gmail, Calendar and Drive, with suggested drafts).

#### Table Stakes: Gmail

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| Mail card: unread or important threads (sender, subject, snippet, time), filterable to Primary, Important or a label | Base requirement | MEDIUM | Use Gmail's own `IMPORTANT` label and category labels. Don't invent ranking for v1. |
| Open a thread in the detail panel with safe HTML rendering (sanitized, remote images blocked by default with a "load images" option) | Reading in-app is a stated requirement | MEDIUM | Sanitization is a security requirement, not polish. Blocking tracking pixels fits the privacy positioning. |
| Reply and reply-all with quoted text and correct threading (`In-Reply-To`/`References`, `threadId`) | Stated requirement | MEDIUM | Plain text plus basic formatting is fine for v1. |
| Quick actions: archive, mark read/unread, star, and "open in Gmail" | Users expect to triage without leaving | LOW | Each one is a single `modify` call. |
| New-mail notification (toast) for important mail only, configurable | Background-monitoring promise | MEDIUM | Poll `history.list` with the stored `historyId` every 1–5 minutes. Push (`users.watch`) needs a Google Cloud Pub/Sub topic, which is not viable for distributed BYO-account users. |
| Undo window on send (5–10 seconds) | Gmail trained everyone to expect it | LOW | Delay the send locally. |

#### Table Stakes: Calendar

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| Today and upcoming agenda (merged across the user's calendars, with per-calendar toggles and colors) | Base requirement | MEDIUM | Use `syncToken` incremental sync. `events.watch` webhooks need a public HTTPS endpoint, so poll instead. |
| "Next meeting" with countdown and a **Join** button (Meet/Zoom/Teams link extracted from conferenceData, location or description) | Notion Calendar's most loved feature | LOW–MEDIUM | |
| Create event: title, date/time, duration, optional guests, location, "add Google Meet", calendar picker | Stated requirement | MEDIUM | |
| Event detail panel: attendees, RSVP status, description, links | Expected | LOW | |
| Meeting reminder toast (e.g., 5 minutes before) with Join | Background-notification promise | LOW | |

#### Differentiators

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| Natural-language quick add in Turkish ("yarın 15:00 Ahmet ile kahve 30dk") parsed by the configured LLM into a structured event, with a confirmation preview | Fantastical/Akiflow-level speed. Google's `quickAdd` endpoint is English-centric. | MEDIUM | Always show the preview before creating. Never create silently. |
| AI-drafted reply ("reply: accept, propose Thursday") that the user edits, then sends | Superhuman/Shortwave/CC-style value | MEDIUM | Draft only. The send button stays with the user. |
| Compose new mail from the palette | Natural extension once reply exists | LOW | Cheap after reply. Recommend v1. |
| Calendar conflict and "busy day" flags fed into the briefing | Google CC flags conflicts | LOW | Deterministic computation, with the AI only phrasing it |
| Mail-to-event ("create event from this mail") | Saves copy-paste | MEDIUM | v1.x |
| Multiple Google accounts | Common for people with work and personal accounts | MEDIUM | Design the data model for N accounts from day one. The UI can ship with one. |

#### Anti-Features

| Feature | Why Requested | Why Problematic | Alternative |
|---------|---------------|-----------------|-------------|
| Full email client (folders, rules, signatures editor, full-text offline search of the whole mailbox, attachments composer) | "Replace Gmail" | Months of work, and it competes with Gmail itself | Triage plus reply plus "open in Gmail" for everything else. Attachments: view and download only in v1. |
| Full week/month calendar with drag-to-reschedule | Looks impressive | Calendar-grid UI is a big project (Notion Calendar-scale) | Agenda plus a lightweight day timeline. Defer the full grid to v2. |
| Gmail push via Pub/Sub | "Real-time" | Each user would need their own Google Cloud project and Pub/Sub setup | Polling `history.list` every 1–5 minutes. Effectively real-time for humans, and cheap on quota. |
| Requesting the full `https://mail.google.com/` scope | Simplicity | Broader than needed. Restricted-scope review is harder. | `gmail.modify` (read, label, archive) plus `gmail.send`. Both are still restricted (see PITFALLS). |

**Distribution note (HIGH confidence, Google official docs):** `gmail.modify` and `gmail.readonly` are **restricted** scopes. Unverified apps are capped at 100 users (lifetime), and in "Testing" mode refresh tokens expire after 7 days. Google's docs tie the CASA security assessment to apps that "store or transmit restricted scope data on servers", so a purely local app likely avoids CASA but still needs restricted-scope verification for public distribution. A practical v1 path for the single owner-user: the user's own GCP project, or "In production, unverified". This affects onboarding: other users may need to bring their own OAuth client ID. Flag for phase research.

---

### 4. AI Daily Briefing

Reference products: Google Labs CC ("Your Day Ahead" email every morning from Gmail, Calendar and Drive, with suggested drafts; launched December 2025), ChatGPT Pulse (overnight visual cards; **OpenAI's help page now says Pulse is being retired** in favor of Scheduled Tasks), Visualping and changedetection.io AI summaries (per-event summarization), and Morgen/Reclaim AI planners.

#### Table Stakes

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| Provider choice: Anthropic (Claude), Google (Gemini), OpenRouter, each with BYO key stored in Credential Manager | Stated requirement | MEDIUM | One provider abstraction: chat completion plus streaming plus usage reporting. |
| Model picker per provider, with a sensible default (fast and cheap) | Users with BYO keys expect to choose | LOW | Fetch the model list where APIs allow (OpenRouter `/models`). Otherwise ship a curated list. |
| Scheduled generation (e.g., 07:30, or "first time I unlock the PC after 05:00") plus a manual "regenerate" | "Sabah özeti" implies it is ready when you arrive | MEDIUM | Catch-up rule: if the PC was asleep at 07:30, generate on resume, once. |
| Plain-language narrative in Turkish: "Bugün şunlar önemli" with sections (meetings, mail needing reply, monitor alerts, other) | Stated requirement | MEDIUM | |
| **Graceful degradation**: no key or provider error leads to a deterministic template summary built from the same data | Core Value: the shell must work even if things break | LOW | Never show an empty home screen because the LLM failed. |
| Streaming output with a skeleton state | Perceived performance | LOW | |
| Cost and token display per briefing (and a running monthly total) | BYO-key users watch spend. PROJECT.md: the app is not a billing intermediary, so it has to be transparent. | MEDIUM | **OpenRouter returns `usage.cost` in every response by default (verified).** Anthropic and Gemini return token counts only, so Mornfold needs a local price table (per model, input/output/cached) that it updates with releases and that the user can edit. Label it "estimated". |
| Briefing history (past days, stored locally) | Users want to look back ("what did yesterday's say?") | LOW | |
| Data-sent transparency: "this briefing sent N emails (subjects + snippets), M events, K alerts to {provider}" | Privacy-first positioning. Mail content goes to a third-party LLM. | LOW–MEDIUM | Exclusion rules: skip labels or senders, and "subjects only" mode. |

#### Differentiators

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| **Grounded briefing with clickable references**: every bullet links to the source item (mail thread, event, diff, tweet) | Fights hallucination and lets the briefing drive action. CC and Pulse do not deep-link into a local dashboard. | MEDIUM | Give each input item an ID and require the model to cite IDs (structured JSON output, then render). Validate that cited IDs exist. |
| Prompt customization in layers: (1) presets for tone, length and focus; (2) a "things I care about" free-text list ("prioritize mail from my bank, ignore newsletters"); (3) an advanced raw system-prompt editor with reset-to-default | Covers both casual and power users | MEDIUM | Store prompt versions so users can roll back. |
| Suggested actions in the briefing ("Reply to Ayşe", "Join 10:00 standup", "Price dropped, open page") as buttons | Turns reading into doing, like CC's drafts | MEDIUM | Actions run only on click, never automatically. |
| Monthly budget cap per provider (stop auto-generation over the cap and notify) | BYO-key trust | LOW | Shares its logic with the X spend cap. |
| Evening "what changed today" mini-briefing | Mirrors Sunsama's shutdown ritual | LOW | Same pipeline, different prompt and inputs. v1.x. |
| Per-event AI summaries for monitors ("price went 1.299 TL → 1.149 TL") | Visualping and changedetection.io now offer this, so it is becoming expected in that niche | MEDIUM | Reuses the provider layer. Optional and off by default because of cost. |
| Local model support (Ollama / LM Studio via OpenAI-compatible endpoint) | Fully private briefing, zero cost | LOW–MEDIUM | Cheap if the provider layer has an "OpenAI-compatible base URL" option. Strong fit for the privacy positioning. v1.x. |

#### Anti-Features

| Feature | Why Requested | Why Problematic | Alternative |
|---------|---------------|-----------------|-------------|
| Free-form chat assistant with autonomous tool use (sends mail, creates events, calls MCP tools on its own) | "AI agent" hype | Prompt injection via incoming mail and web pages is a real attack path. The scope explodes. | Briefing plus suggested actions that require a click. A chat panel can come in v2 with strict tool consent. |
| Bundled or proxied AI usage ("we pay, you subscribe") | Easier onboarding | Contradicts PROJECT.md ("app is not a billing intermediary") and needs a server | BYO key with a guided "get a key" link per provider |
| Sending full raw mail bodies by default | Better summaries | Privacy and cost | Subjects, snippets and metadata by default. Full body is opt-in per briefing. |
| Using AI for ranking and importance when deterministic signals exist | "Smart" | Slow, costly, non-deterministic | Gmail IMPORTANT label, calendar times and monitor conditions do the ranking. The AI only narrates. |

---

### 5. Website Change Detection

Reference products: Distill.io (visual selector plus XPath/CSS/regex refinement, conditions with all/any logic, local monitors from 5 seconds and cloud monitors, version history, desktop app), Visualping (AI summaries and "Important Alerts" filtered by a plain-English prompt, on all plans), changedetection.io (CSS/XPath/JSONPath/jq filters, ignore and trigger text, price/restock detection with thresholds, word/line/char diff, Playwright browser steps, AI change rules), ChangeTower.

#### Table Stakes

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| Add monitor by URL, with a whole-page text default | Entry point in every product | LOW | Extract readable text, strip scripts and styles, normalize whitespace. |
| Scope to part of the page with a CSS selector (XPath optional) | Distill and changedetection.io both offer it. Whole-page monitoring is too noisy. | LOW–MEDIUM | |
| Check interval per monitor (presets 15m / 1h / 6h / daily) with jitter | Universal | LOW | Enforce a minimum, e.g. 5 minutes, and per-domain politeness. Distill's local monitors go down to 5 seconds, but that is abuse-prone and gets users IP-banned. |
| JavaScript rendering toggle (static fetch vs headless render) | Many modern sites (e-commerce, listings) render client-side. changedetection.io has a plain fetcher and a Chrome fetcher. | HIGH | Headless rendering on Windows has a real cost. Static fetch is the default, with a render fallback. Needs stack research. |
| Diff view: added and removed highlighted inline (word level), plus a side-by-side option | Universal. changedetection.io offers word, line and char levels. | MEDIUM | Use a proven diff library. Don't hand-roll it. |
| Change history (snapshots with timestamps, configurable retention) | Distill version history, changedetection.io history | LOW–MEDIUM | Store compressed text locally. Cap retention per monitor. |
| Noise filtering: ignore-text rules (plain or regex), exclude sub-selectors, ignore whitespace, numbers-only/dates option | False alerts are the #1 complaint (Visualping G2 reviews, Distill forums) | MEDIUM | |
| Conditions or triggers: "notify only if text appears or disappears", "number below X / changed by ≥ N%" (price drop) | Stated use cases: price drop, new listing, announcement. changedetection.io and Distill both have these. | MEDIUM | Combine with all/any logic (Distill model). |
| Error states: page unreachable, selector no longer matches, blocked (403/captcha) | Silent failure is the worst outcome | LOW | A "selector not found" state is different from "no change". |
| Pause/resume and "check now" | Universal | LOW | |

#### Differentiators

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| **Visual element picker** (open the page in an in-app webview, hover-highlight, click to select, which generates the selector) | The biggest UX gap for non-technical users. Distill and changedetection.io both treat it as a headline feature. | HIGH | Needs an in-app browser view with an injected picker script. v1 if possible, otherwise v1.x (CSS input is the v1 floor). |
| Templates for common intents: "price tracking", "new items in a list", "keyword appears" | Faster setup and better defaults | MEDIUM | Price template: auto-detect a price via schema.org `Product`/`Offer` JSON-LD, the way changedetection.io does restock and price. |
| AI "only notify me if…" filter (plain-language criteria, judged by the user's LLM) | Visualping Important Alerts, changedetection.io AI rules. Kills noise. | MEDIUM | Non-important changes are still logged, just with no toast (Visualping's model). Off by default because of cost. |
| AI one-line summary of each change | Readable alerts | LOW (after the AI layer exists) | |
| "New items" list mode (track a list of links or cards, alert only on newly added entries) | Covers "yeni ilan" (new listing) precisely | MEDIUM | Item identity is the link URL or hash. Much less noisy than a text diff. |
| Number and price history chart | Gives context to price drops | LOW | Reuses the sparkline component from API cards. |

#### Anti-Features

| Feature | Why Requested | Why Problematic | Alternative |
|---------|---------------|-----------------|-------------|
| Screenshot or pixel visual diff | Visualping's signature | Heavy (needs headless screenshots), noisy (ads, carousels), storage-hungry | Text/DOM diff. Maybe "screenshot on change" for context in v2. |
| Login-required pages with recorded browser steps (fill form, click, wait) | Distill and changedetection.io have it | High complexity, brittle, and storing site passwords is a security liability | Defer to v2. Cookie import is a possible middle ground. |
| Seconds-level intervals | "Instant" | Gets the user IP-banned and burns CPU and battery | 5-minute floor, with a warning below 15 minutes |
| Cloud monitoring while the PC is off | Distill and Visualping cloud monitors | Needs a server, which is out of scope by principle | Be honest in the UI: "checks run while your PC is on", plus catch-up on wake |
| Captcha solving or anti-bot evasion | Some shops block bots | Legal and ToS risk, an arms race | Detect the block, mark the monitor "blocked", suggest the JS-render mode |

---

### 6. Twitter/X Account & Keyword Monitoring

Reference products: X Pro (formerly TweetDeck: columns for profile, search, topics and lists, with column filters for keyword, date and media, "Decks" grouping columns, paid X Premium only), Hootsuite and Sprout (streams, keyword monitoring), social-listening tools.

**Market reality (verified, docs.x.com pricing page):** The X API is now **pay-per-usage with prepaid credits and no free tier for reads**: **$0.005 per post read**, **$0.010 per user lookup**, Counts: Recent $0.005 per request, Trends $0.010 per request. There is a 24h UTC dedup window, so re-reading the same post the same day is not billed again (a "soft guarantee"), and a cap of 3M post reads per month before Enterprise. Nitter is effectively dead: guest accounts ended in January 2024, and remaining instances are unreliable as of September 2026. Third-party paid APIs (twitterapi.io, Apify actors, roughly $0.15–$0.40 per 1K tweets according to vendor claims) are cheaper but carry ToS and reliability risk.

#### Table Stakes

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| Account watchlist: add @handles, get a toast plus feed entry when they post | Stated requirement | MEDIUM | Poll the user timeline with `since_id`, so each new post is billed only once. Resolving a handle to a user ID costs $0.01 once and should be cached. |
| Keyword / hashtag / query monitors (X search query syntax: `from:`, `-is:retweet`, `lang:tr`) | Stated requirement | MEDIUM | Recent search with `since_id`. Popular keywords can return hundreds of posts per hour, so cost guardrails are mandatory. |
| Filters per monitor: exclude replies and reposts, language, minimum engagement, has-media | X Pro column filters, Hootsuite stream filters | LOW–MEDIUM | Push into the query where possible. Filtering client-side after billing still costs money. |
| X feed card (TweetDeck-like column on the dashboard) showing the text, author, time and media thumbnail, with "open on X" | Expected visual | MEDIUM | Render text plus links. No embedded X widgets, which load X tracking. |
| **Spend guardrails**: estimated monthly cost shown before saving a monitor, per-monitor max results per poll, a global monthly cap that pauses X polling, and spend to date | Pay-per-use under the user's key, and keyword monitors can run away | MEDIUM | This is table stakes for this product, not polish. One bad keyword monitor could cost tens of dollars a day. |
| Poll interval per monitor (15m default, with a minimum) | Universal | LOW | Shorter intervals don't increase cost much thanks to `since_id`, but they do raise rate-limit risk. |
| Graceful "X not configured" state (the feature is optional, with a setup guide for getting an X developer key and buying credits) | Many users won't pay for X | LOW | |

#### Differentiators

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| Provider abstraction: official X API (default) **or** a pluggable third-party data provider, or an MCP server that exposes X search | Lets cost-sensitive users choose and keeps core code clean | MEDIUM | Ship official only. Allow others via MCP or custom-API definition to keep ToS risk at arm's length. |
| AI digest of keyword monitors ("12 posts about #deprem today; the main themes are…") instead of 12 toasts | Turns noise into signal | MEDIUM | Feeds the morning briefing. |
| Smart notify rules: toast only for watched accounts, digest-only for keywords | Keeps keyword monitors from spamming | LOW | Default behavior. |
| X Lists as a source (monitor a list the user curates on X) | One list read replaces many account polls | LOW–MEDIUM | Billing is still per post returned. Verify the list endpoints' cost. |

#### Anti-Features

| Feature | Why Requested | Why Problematic | Alternative |
|---------|---------------|-----------------|-------------|
| Scraping x.com or using Nitter instances | "Free" | Nitter is effectively dead, scraping violates ToS, and the user's account can be banned. Very fragile. | Official API with spend caps. Third-party APIs only via user-installed MCP or custom definitions. |
| Filtered stream or real-time firehose | "Instant" | Streaming is higher-tier and costly, and pointless for a personal dashboard | Polling every 5–15 minutes |
| Posting, replying or liking from the app | Natural next step | Explicitly out of scope in PROJECT.md | "Open on X" |
| Mentions and DM monitoring | Common in social tools | Explicitly out of scope in PROJECT.md | — |
| Full-archive historical search and analytics | Social-listening tools offer it | Expensive and out of a personal-dashboard scope | Recent search only |

---

### 7. Free-API Cards (Catalog + Custom API Definition)

Reference products: Homepage `customapi` widget (URL, method, headers, `mappings` with dot-path fields, formats text/number/float/percent/bytes/date, `remap/scale/prefix/suffix`, `refreshInterval` in ms, block or list display), Glance `custom-api` widget (Go templates over gjson paths, subrequests, but the user writes HTML), Homarr custom API, Home Assistant RESTful sensor (value_template, scan_interval) and template cards.

#### Table Stakes

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| Catalog of ready-made cards, added with one click plus minimal parameters (city, currency pair, coin) | Stated requirement | MEDIUM | Each catalog entry **is** a custom-API definition shipped as JSON, so the catalog and custom definitions share one engine. Turkish-relevant picks: Open-Meteo weather, Frankfurter / TCMB exchange rates, CoinGecko, USGS/Kandilli/AFAD earthquakes, Aladhan prayer times, Hacker News, GitHub repo stats, NASA APOD, Wikipedia "on this day". Verify each API's ToS, key needs and rate limits during the phase. |
| Custom API wizard: URL, method (GET/POST), query params, headers, body | Stated requirement ("URL gir") | MEDIUM | |
| Auth options: none / API key in header / API key in query / Bearer / Basic, with secrets stored in Credential Manager | Most free APIs need a key | LOW–MEDIUM | Secrets never go into the exportable definition. |
| "Test request" that shows the response as a collapsible JSON tree, where clicking a field generates its path (JSONPath or dot-path) | Stated requirement ("gösterilecek alanı seç") and the key UX moment | MEDIUM | Click-to-pick avoids teaching JSONPath. Allow manual path editing. |
| Visualizations: single value (big number with label, prefix/suffix, number/percent/date format), key-value list, list of items (array to rows with title, subtitle and link), small table | Homepage formats and display modes cover this | MEDIUM | Keep the set small and polished. |
| Refresh interval per card (with a floor, e.g. 1 minute) plus manual refresh, with ETag/Last-Modified respected | Universal | LOW | |
| Error display (HTTP status, JSON path not found, timeout) | Debuggability | LOW | |

#### Differentiators

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| Sparkline or trend history for numeric cards (values stored locally on every refresh) | Turns a number into insight (exchange rate trend) | MEDIUM | Shares storage with the monitor price history. |
| Threshold alerts on API values ("notify if USD/TRY > 35" or "earthquake magnitude ≥ 5 near Istanbul") | Merges API cards with monitoring. Most dashboards (Homepage, Glance) can't alert. | MEDIUM | Reuses the condition engine from website monitors. A strong unifying feature. |
| Value transforms: math (scale), remap (code to label), and date relative formatting | Homepage has remap/scale/prefix/suffix | LOW–MEDIUM | Declarative only. |
| Share and import definitions as JSON (no secrets), so a community catalog can grow from user submissions | Extensibility without code | LOW | Pairs with the static catalog repo. |
| API card values available to the AI briefing ("Dolar 34,2 (+0,4%)") | The briefing covers everything on screen | LOW | Feed rendered values, not raw JSON. |
| Subrequests or chaining (call A, use a field in B) | Glance supports it | HIGH | v2. Covers OAuth-less multi-step APIs. |

#### Anti-Features

| Feature | Why Requested | Why Problematic | Alternative |
|---------|---------------|-----------------|-------------|
| User-written HTML/JS templates for rendering (Glance style) | Unlimited flexibility | XSS inside a privileged desktop webview that holds keys. Inconsistent visuals. | A fixed set of polished visualization types plus declarative mappings |
| Arbitrary script transforms (JS eval of responses) | Power users | Code execution from shared definitions is a security hole | Declarative transforms. Anything more complex becomes an MCP server. |
| GraphQL, SOAP, WebSocket sources | Completeness | Low demand for free public APIs | REST/JSON only in v1. XML or RSS can come later. |

---

### 8. Background Running & Notifications

Reference products: Rambox, Station and Shift (unified notifications across apps, do-not-disturb schedule, per-app mute), Slack and Discord desktop (tray, start at login, DND schedules), Distill desktop (local monitors only run while the app is open), Windows Focus / Do Not Disturb.

#### Table Stakes

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| Close-to-tray (window close hides the window and keeps the scheduler running), with a tray menu: Open, Pause all, Refresh all, Settings, Quit | Stated requirement. Every background app has it. | MEDIUM | Explain it on first close ("Mornfold keeps running in the tray"). |
| Start at Windows login (optional, starts minimized) | Required for background monitoring to mean anything | LOW | |
| Windows toast notifications with app identity (name and icon), click-through deep link to the item | Stated requirement | MEDIUM | Tauri's notification plugin notes that Windows toasts only show the right app identity for **installed** apps (they show PowerShell in dev). Action buttons in toasts are not confirmed in the Tauri plugin (the underlying crate lists actions as TODO), so treat that as stack research. |
| In-app notification center (history list with read/unread and filter by source) | Toasts disappear. Users need to find "that alert from 10:14". | MEDIUM | This is also an input to the briefing. |
| Per-source notification settings (on/off, toast vs silent-only, sound) | Rambox/Station-style per-app mute | LOW–MEDIUM | |
| Quiet hours (schedule) plus respecting Windows Do Not Disturb / Focus | Universal in Slack and Station | LOW–MEDIUM | During quiet hours, queue into the notification center and the digest. Don't drop. |
| Notification batching and anti-storm: group bursts ("5 site changes") and rate-limit per source | Without it, monitors spam | MEDIUM | |
| Sleep/resume catch-up: on wake, run overdue jobs **once** (not N missed runs), with staggered start | Laptops sleep. Naive schedulers stampede on resume. | MEDIUM | Same logic as the morning-briefing catch-up. |
| Unread badge on the tray icon | Small expectation | LOW | |

#### Differentiators

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| Digest mode: low-priority alerts collected into a scheduled digest (midday or evening), optionally AI-summarized | Calm-tech positioning, Linear-like restraint | MEDIUM | Natural reuse of the briefing pipeline |
| Priority levels per alert (urgent toast that bypasses quiet hours vs normal vs digest-only) | Important things still get through (e.g., a price threshold) | LOW–MEDIUM | |
| Power and network awareness: slow polling on battery saver or metered connections, pause when away for a long time | Laptop-friendly, matches the "zarif" (graceful) ethos | MEDIUM | |
| Snooze an alert or monitor ("remind me in 1h", "mute this site today") | Faster triage | LOW | |

#### Anti-Features

| Feature | Why Requested | Why Problematic | Alternative |
|---------|---------------|-----------------|-------------|
| Separate always-on Windows service for monitoring when the user is logged out | "Runs even when logged out" | Elevation, installer complexity, and secrets access issues (DPAPI is per user) | User-session tray app with start-at-login |
| Email/SMS/Slack/Telegram outbound alert channels in v1 | Distill and changedetection.io have many channels | Each is a connector, and some need servers or accounts. Not the core use. | Windows toast plus notification center. Add outbound channels later as MCP or webhooks. |
| Notification sounds per event type, custom sound packs | Fun | Low value, takes design time | One subtle default sound, on or off |

---

## Feature Dependencies

```
Secure vault (Credential Manager/DPAPI) ─┐
Local storage (SQLite) ──────────────────┼──> Connector framework (health, status, sync scheduler, errors)
Job scheduler (intervals, catch-up) ─────┘            │
                                                      ├──> Google OAuth (loopback+PKCE) ──> Gmail card / reply / compose
                                                      │                                 └─> Calendar agenda / create event / join
                                                      ├──> HTTP fetch engine ──> Custom API engine (auth, JSON path, mapping)
                                                      │                     │        ├──> API catalog (= shipped definitions)
                                                      │                     │        └──> MCP-backed cards (reuse mapper)
                                                      │                     └──> Website monitor (extract, diff, history)
                                                      │                              └──> Visual picker (needs in-app webview)
                                                      ├──> MCP client (stdio + Streamable HTTP + OAuth) ──> MCP tools/cards/briefing context
                                                      └──> X connector (API key, since_id polling, spend tracking)

Card contract + layout grid ──> Home "Günün özeti" (deterministic) ──> AI briefing narrative overlay
AI provider layer (Claude/Gemini/OpenRouter, usage, cost table) ──> Briefing
                                                               ├──> NL quick-add event
                                                               ├──> AI reply drafts
                                                               ├──> AI change summaries / "only notify if"
                                                               └──> Keyword digest

Condition engine (text appears, number < X, % change) ──> Website triggers
                                                       ├──> API threshold alerts
                                                       └──> (X engagement filters)

Notification pipeline (toast + center + quiet hours + batching) <── every monitor, Gmail, Calendar reminders
Tray + start-at-login ──> background monitoring is meaningful at all
Spend tracker (shared) <── AI provider layer, X connector
Command palette action registry <── every connector registers actions
```

### Dependency Notes

- **Everything requires the vault, storage, scheduler and connector framework.** These are the "shell + service-binding infrastructure" that Core Value says must work when everything else breaks. They belong in the first phases.
- **The card contract comes before any connector UI.** Gmail, Calendar, monitors and API cards all render through it. Retrofitting consistent loading, error and stale states later is painful.
- **The AI briefing depends on the deterministic home.** The briefing is a narration of data the home screen already holds. Building the deterministic home first gives a working fallback and makes the AI phase "prompt + rendering" instead of "data plumbing + AI".
- **The catalog depends on the custom API engine.** Implement catalog entries as shipped custom definitions. Building the catalog as hardcoded widgets would duplicate the engine and violate "add a service without touching code".
- **MCP-backed cards reuse the custom-API field mapper.** Build the mapper first, generically (input: any JSON).
- **The condition engine is shared** by website triggers, API thresholds and X filters. Build it once, in the first monitoring phase.
- **The notification pipeline must exist before the monitors ship**, or every monitor phase reinvents toasts.
- **The spend tracker is shared by AI and X.** Building it with the AI phase means X inherits it.
- **The visual picker needs an in-app webview or browser.** The JS-render fetch mode also needs a headless browser. Decide both together in stack research, since the same component may serve both.
- **Conflict: Gmail push (Pub/Sub) conflicts with local-only, distributable BYO accounts.** Choose polling.
- **Conflict: user HTML templates conflict with storing secrets in the same webview.** Choose declarative visualizations.

---

## MVP Definition

### Launch With (v1)

Scope set by PROJECT.md (all monitoring in v1), trimmed to the minimum depth per feature.

- [ ] Shell: window, Turkish UI (i18n-ready), light/dark, glass styling, card contract, section grid with reorder and S/M/L, command palette, keyboard shortcuts. This is the Core Value.
- [ ] Onboarding wizard (skippable steps, test buttons). Others will install it.
- [ ] Connections page: status, last sync, reconnect, disconnect-and-purge, test. Core Value.
- [ ] Vault (Credential Manager), local SQLite, scheduler with sleep/resume catch-up. The foundation.
- [ ] Google OAuth plus a Gmail card (important/unread), thread view (sanitized), reply, archive/read/star, new important mail toast. Stated requirement.
- [ ] Calendar agenda, next-meeting with Join, create event, reminder toast. Stated requirement.
- [ ] Deterministic "Günün özeti" home. Works without AI.
- [ ] AI briefing: Claude/Gemini/OpenRouter BYO key, model picker, schedule plus regenerate, Turkish narrative, cost per run (estimated for Claude/Gemini, exact for OpenRouter), history, data-sent transparency, deterministic fallback.
- [ ] Prompt customization at preset level plus a "things I care about" text. The raw prompt editor can be v1 if cheap.
- [ ] MCP client: add stdio and remote servers, paste `mcpServers` JSON, tool list with per-tool enable, consent, logs, restart. Stated requirement for extensibility.
- [ ] MCP-backed card (one tool, fixed args, mapped output). This is what makes "add a service without code" visible on the dashboard. Without it, MCP has no surface in v1 except the briefing.
- [ ] Website monitor: URL, CSS selector, interval, static fetch (JS render if stack research allows), word diff, history, ignore rules, conditions (text appears, number below / %), error states.
- [ ] X monitor: official API BYO key, account watchlist, keyword/query monitors, filters, feed card, **spend estimate and caps**.
- [ ] API cards: custom definition wizard (URL, auth, test, click-to-pick JSON field, visualization types, refresh) plus a starter catalog (~10 entries) shipped as definitions.
- [ ] Background: tray, close-to-tray, start at login, toasts with deep link, notification center, per-source settings, quiet hours, batching.

### Add After Validation (v1.x)

- [ ] Visual element picker for website monitors. Add it when users struggle with CSS selectors, or in v1 if the webview approach turns out easy.
- [ ] AI "only notify if…" and AI change summaries. Add when noise complaints appear.
- [ ] Threshold alerts on API cards plus sparklines. Natural once the condition engine and history exist.
- [ ] Natural-language quick-add (Turkish) for events, and AI reply drafts.
- [ ] Digest mode (scheduled evening digest), evening mini-briefing.
- [ ] Local LLM via OpenAI-compatible base URL (Ollama/LM Studio).
- [ ] Global summon hotkey.
- [ ] Multiple Google accounts in the UI (data model ready from v1).
- [ ] `.mcpb` bundle install and a recommended-connectors gallery.
- [ ] Compose new mail (if not squeezed into v1), mail-to-event.
- [ ] Price template with JSON-LD auto-detect and "new items" list mode for monitors.

### Future Consideration (v2+)

- [ ] Chat panel with strict tool-consent (agentic use of MCP tools). Prompt-injection risk needs careful design.
- [ ] Full calendar grid with drag-to-reschedule.
- [ ] Login-required website monitoring and browser steps.
- [ ] Screenshot capture on change.
- [ ] Outbound alert channels (Telegram, webhook).
- [ ] API chaining and subrequests.
- [ ] MCP Apps / Tasks extensions (interactive UI from servers, long-running jobs).
- [ ] Multiple dashboard pages with time-aware layouts.

---

## Feature Prioritization Matrix

| Feature | User Value | Implementation Cost | Priority |
|---------|------------|---------------------|----------|
| Card contract + layout grid + home | HIGH | MEDIUM | P1 |
| Command palette | HIGH | MEDIUM | P1 |
| Onboarding wizard | HIGH | MEDIUM | P1 |
| Connections page + health model | HIGH | MEDIUM | P1 |
| Vault + storage + scheduler (catch-up) | HIGH | MEDIUM | P1 |
| Google OAuth + Gmail triage/reply | HIGH | MEDIUM–HIGH | P1 |
| Calendar agenda + create + Join | HIGH | MEDIUM | P1 |
| AI briefing (3 providers, cost, fallback) | HIGH | MEDIUM | P1 |
| Grounded briefing with clickable refs | HIGH | MEDIUM | P1 (core differentiator, cheap if designed in from the start) |
| MCP client (stdio + remote + consent) | HIGH | HIGH | P1 |
| MCP-backed cards | MEDIUM–HIGH | HIGH | P1 (minimal) |
| Website monitor core (selector, diff, conditions, ignore) | HIGH | MEDIUM | P1 |
| JS-rendered fetch | MEDIUM | HIGH | P1/P2 (stack-dependent) |
| X monitoring + spend caps | MEDIUM | MEDIUM | P1 (stated), with caps mandatory |
| Custom API wizard + catalog | HIGH | MEDIUM | P1 |
| Tray, toasts, notification center, quiet hours, batching | HIGH | MEDIUM | P1 |
| Visual element picker | HIGH | HIGH | P2 |
| AI notify filter / change summaries | MEDIUM | LOW–MEDIUM | P2 |
| API threshold alerts + sparklines | MEDIUM | MEDIUM | P2 |
| NL quick-add, AI reply drafts | MEDIUM | MEDIUM | P2 |
| Digest mode / evening briefing | MEDIUM | LOW–MEDIUM | P2 |
| Local LLM support | MEDIUM | LOW | P2 |
| Global summon hotkey | MEDIUM | MEDIUM | P2 |
| `.mcpb` install + connector gallery | MEDIUM | MEDIUM | P2 |
| Chat agent, full calendar grid, browser steps, screenshots | LOW–MEDIUM | HIGH | P3 |

**Priority key:**
- P1: Must have for launch
- P2: Should have, add when possible
- P3: Nice to have, future consideration

---

## Competitor Feature Analysis

| Feature | Competitor A | Competitor B | Our Approach |
|---------|--------------|--------------|--------------|
| Daily briefing | Google CC: email "Your Day Ahead" from Gmail, Calendar and Drive, with suggested drafts; US-only early access | ChatGPT Pulse: overnight visual cards; being retired per OpenAI help | In-app, grounded, clickable references, BYO provider, cost shown, works without AI |
| Command palette | Raycast: global launcher, extension store | Akiflow: command bar for capture and triage | In-app Ctrl+K action registry that connectors extend. Global hotkey in v1.x. |
| Dashboard widgets | Glance/Homepage: YAML config, custom API widgets | Home Assistant: GUI card editor, sections view | GUI-first cards, curated visual types, JSON import/export for power users |
| Custom API cards | Homepage: mappings + formats + refreshInterval | Glance: Go templates over gjson, subrequests | Click-to-pick JSON field, declarative visualizations, secrets in vault, threshold alerts |
| Website change detection | Distill: visual selector, conditions, local plus cloud monitors, many channels | Visualping / changedetection.io: AI importance filters and summaries, browser steps, price/restock | Local-only, text/DOM diff, conditions, ignore rules. Visual picker and AI filter as fast-follows. |
| X monitoring | X Pro: columns, filters, Decks (requires Premium) | Hootsuite: streams, keyword listening | Official API BYO key, polling with `since_id`, mandatory spend caps, AI keyword digest |
| Plugin model | Claude Desktop: MCP stdio + remote, `.mcpb` bundles, extension directory | Station/Rambox/Shift: embedded web apps | MCP client plus MCP-backed dashboard cards. No embedded web apps. |
| Background/notifications | Station/Rambox: unified notifications, DND schedule | Distill desktop: local monitors only while running | Tray, start at login, notification center, quiet hours, batching, sleep catch-up |

---

## Sources

- X API pricing (official): https://docs.x.com/x-api/getting-started/pricing — pay-per-usage, $0.005/post read, $0.010/user, 3M reads/month cap, 24h dedup (HIGH)
- X pay-per-use launch coverage: https://www.medianama.com/2026/02/223-x-developer-api-pricing-pay-per-use-model/ (MEDIUM)
- X Pro features: https://help.x.com/using-x/x-pro , https://help.x.com/using-twitter/advanced-tweetdeck-features (MEDIUM)
- Nitter status: https://en.wikipedia.org/wiki/Nitter , https://www.getxapi.com/alternatives/nitter (MEDIUM, the latter is vendor content)
- Third-party X data pricing (vendor claims): https://www.getxapi.com/blogs/cheapest-twitter-api-2026-8-providers-ranked-by-real-per-1000-tweet-cost , https://apify.com/altimis/scweet/pricing (LOW)
- MCP specification (latest = 2026-07-28; tools, resources, prompts, elicitation; consent requirements; Tasks / MCP Apps extensions): https://modelcontextprotocol.io/specification/latest (HIGH)
- MCP auth changes (CIMD over DCR, RFC 8707 resource): https://ssojet.com/blog/mcp-authorization-spec-2026-07-28-what-changed , https://den.dev/blog/mcp-november-authorization-spec/ (MEDIUM, verify against the spec in phase research)
- Claude Desktop extensions / MCPB: https://www.anthropic.com/engineering/desktop-extensions , https://support.claude.com/en/articles/10949351-getting-started-with-local-mcp-servers-on-claude-desktop (HIGH)
- Google restricted-scope verification: https://developers.google.com/identity/protocols/oauth2/production-readiness/restricted-scope-verification (HIGH); 100-user cap and 7-day test tokens: https://nango.dev/docs/api-integrations/google-shared/google-security-review (MEDIUM)
- Gmail push vs history polling: https://developers.google.com/gmail/api/guides/push (HIGH)
- OpenRouter usage/cost in responses: https://openrouter.ai/docs/use-cases/usage-accounting (HIGH)
- Distill features and local vs cloud: https://distill.io/features , https://distill.io/docs/web-monitor/cloud-local-monitors/ (HIGH)
- Visualping AI summaries and Important Alerts: https://visualping.io/pages/blog/visualping-ai , https://visualping.io/pages/blog/q1-2026-release-recap (MEDIUM)
- changedetection.io features: https://github.com/dgtlmoon/changedetection.io (HIGH)
- Glance custom-api: https://github.com/glanceapp/glance/blob/main/docs/custom-api.md (HIGH)
- Homepage customapi: https://gethomepage.dev/widgets/services/customapi (HIGH)
- Google CC daily briefing: https://blog.google/technology/google-labs/cc-ai-agent/ (HIGH)
- ChatGPT Pulse (retirement): https://help.openai.com/articles/12293630-chatgpt-pulse (MEDIUM)
- Sunsama vs Akiflow: https://www.morgen.so/blog-posts/sunsama-vs-akiflow (LOW–MEDIUM, vendor-authored)
- Tauri notification plugin (Windows installed-app caveat), tauri-winrt-notification actions TODO: https://v2.tauri.app/plugin/notification/ , https://docs.rs/tauri-winrt-notification (MEDIUM)
- Raycast, Home Assistant dashboards, Station/Rambox/Shift, Notion Calendar, Superhuman/Shortwave UX patterns: training knowledge (LOW–MEDIUM, not re-verified this session)

---
*Feature research for: local-first personal command center / AI briefing dashboard (Windows)*
*Researched: 2026-10-18*
