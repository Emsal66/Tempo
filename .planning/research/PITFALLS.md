# Pitfalls Research

**Domain:** Local-first Windows desktop command center. It covers Gmail/Calendar, X monitoring, website change detection, an MCP client, BYO-key AI summaries, custom API cards, and tray + toast notifications. The owner uses it first, and it gets distributed to other users later.
**Researched:** 2026-10-18
**Confidence:** MEDIUM-HIGH. X pricing, Google OAuth rules, the MCP spec, code signing and SmartScreen are verified against official docs dated 2026. Windows background-throttling details and third-party X alternatives are MEDIUM/LOW and are flagged inline.

> Phase names below follow a suggested roadmap order, not a fixed numbering:
> **P1 Foundation** (Windows CI build, shell, tray, notifications, secure storage, background scheduler) →
> **P2 Connector framework + Google** →
> **P3 AI summary ("Günün özeti")** →
> **P4 Website change detection** →
> **P5 Custom API cards + catalog** →
> **P6 MCP client** →
> **P7 X/Twitter monitoring** →
> **P8 Distribution** (installer, signing, auto-update, onboarding wizard, Google verification decision).
> The roadmapper may renumber these. The *ordering constraints* in each pitfall are what matter.

---

## Critical Pitfalls

### Pitfall 1: Assuming X/Twitter monitoring is cheap, free, or scrapeable

**What goes wrong:**
The plan assumes a free or cheap X tier, or a scraper or Nitter feed. In reality:
- **The free tier is gone for new developers, and fixed tiers are closed to new signups.** The official model is now **pay-per-usage with prepaid credits**. There is no subscription and no minimum. Official rates are **$0.005 per post read** and **$0.010 per user read** (e.g. resolving a @handle to an ID). Resources are deduplicated within a 24h UTC window, but X calls this a "soft guarantee". There is a cap of **3M post reads per month**, and anything above that requires Enterprise. (docs.x.com pricing, HIGH.) Third-party sources say legacy Basic ($200/mo) and Pro ($5,000/mo) remain only for existing subscribers and are being migrated (MEDIUM).
- **Rate limits** (official): user timeline `GET /2/users/:id/tweets` allows 900 req/15 min per user and 10,000 per app. Recent search allows 300 req/15 min per user and 450 per app, with max 100 results and a 512-char query. Filtered stream allows 1 connection and 1,000 rules. (HIGH)
- **Scraping is off the table for a distributed product.** X's ToS (effective Nov 15, 2024) prohibits crawling and scraping without consent. It adds a liquidated-damages clause of $15,000 for accessing more than 1M posts in 24h, and that clause extends to anyone who "induces or knowingly facilitates" violations. Sources disagree on whether the figure is flat or per million (MEDIUM). Shipping a scraper puts *you*, the distributor, in the "facilitates" category.
- **Nitter is effectively dead.** X sent cease-and-desist letters on Aug 24, 2026, and the GitHub repo was archived on Aug 25, 2026. There was a brief revival in early September, then reports of instances going offline again around Sep 15, 2026 (MEDIUM). Any RSS-via-Nitter design will break.
- **Third-party "Twitter APIs"** (scraping-backed resellers) are cheaper but sit on the ToS-violation side. They can vanish overnight, and X has sued scrapers before (Bright Data, CCDH; both dismissed at district level, so this is not settled law). LOW-MEDIUM on any specific vendor's longevity.

**Why it happens:**
Training data and old blog posts describe the 2023-2025 tier model ($100/$200 Basic, free write-only tier). The landscape changed twice in 2025-2026.

**Cost reality check (keep this in the plan):**
- *Account monitoring:* 20 accounts × ~10 new posts/day = 200 reads/day, about **$1/day, or ~$30/month**, per user, paid by each user from their own X credits. One-time handle→ID resolution costs $0.01 each. Cache the ID forever.
- *Keyword monitoring:* a broad keyword or hashtag can return thousands of posts/day. Each 100-result page costs $0.50. A single careless "AI" keyword at 5-minute polling can burn **tens of dollars per day**. This feature is the main cost hazard in the whole app.
- Whether a request returning *zero* new posts is billed is not stated explicitly. Billing is described as "per resource returned" (LOW: verify in the Developer Console during P7).

**How to avoid:**
1. Use the **official X API pay-per-use only**, with the user's own developer app and credentials. The app is not a billing intermediary, which matches the PROJECT.md constraint. Do not ship scraping, Nitter, or reseller-API backends as defaults. If offered at all, they should be an opt-in "community MCP server" the user installs at their own risk, outside the core.
2. Always poll with `since_id` and minimal `tweet.fields`. Cache user IDs. Store the last-seen ID per account and per query.
3. Build a **local cost meter**: count resources returned × the rate card, and show "~$X this month" per monitored item. Enforce a hard per-day budget in-app *in addition to* the X Developer Console spending limit.
4. For keyword monitoring, require a **narrow query** (operators such as `from:`, `lang:tr`, `-is:retweet`). Show an estimated daily cost before saving, and default to a longer interval (e.g. 15-30 min).
5. Optional cheaper or alternative path for keyword *summaries* rather than real-time alerts: xAI's server-side `x_search` tool (~$5 per 1k calls plus tokens, per xAI pricing docs; MEDIUM, sources conflict with $10/1k). It is an official X-family channel, but results are LLM-mediated and not a reliable "every new post" feed. Treat it as a later experiment, not the core.
6. Put X **last among the monitoring features**. It is the most externally volatile dependency, and its pricing changed again in 2026.

**Warning signs:**
- Any design doc mentioning "free tier", "Basic $100/$200", Nitter, RSSHub-for-X, or guest tokens.
- No cost estimate shown in the "add keyword" UI.
- Credits draining while the app sits idle overnight (missing `since_id`, or re-fetching after resume).

**Phase to address:** P7 (X monitoring), placed late. Add a research flag: re-verify docs.x.com pricing and rate limits at P7 start. Cost-meter infrastructure can be shared with AI cost tracking from P3.

---

### Pitfall 2: Google OAuth: treating "distribute to others" as a small step after "works for me"

**What goes wrong:**
Gmail read access requires a **restricted** scope (`gmail.readonly` / `gmail.modify` / `https://mail.google.com/`). This leads to several problems:
- **Unverified apps are capped at 100 new users total** after the unverified-app screen is shown. Google may adjust this per app (support.google.com/cloud/answer/7454865, HIGH). The cap lifts only after verification.
- **"Testing" publishing status means refresh tokens expire after 7 days** (developers.google.com/identity/protocols/oauth2, HIGH). An app left in Testing makes the owner re-login every week. Background monitoring silently dies on day 8.
- **Restricted-scope verification** takes ~6 weeks (an estimate, not guaranteed). It requires a **CASA security assessment** from a Google-empanelled assessor, **repeated at least every 12 months**. Adding a new restricted scope can trigger reassessment. Google charges no fee, but assessors do. Tier 2 may be free, and Tier 3 requires a deployment review (HIGH on the process; cost figures are LOW). The trigger wording is: apps that can access restricted data "from or through a third-party server".
- **Mornfold-specific trap:** "local-only" does *not* obviously exempt the app. Sending email bodies to **Anthropic / Google Gemini / OpenRouter** for summaries plausibly counts as transmitting restricted data through third-party servers. Google's published exemption list (personal use, dev/testing, service-owned data, internal-only, admin-trusted) contains **no "data stays on device" exemption** (HIGH that it's not listed; the exact interpretation for LLM transfer is unresolved).
- **Limited Use policy:** transfers of Google user data to third parties are allowed "only with the user's consent" (plus security/legal/M&A). Use must be for "user-facing features that are prominent". The app therefore needs an explicit, prominent consent step before email content goes to an AI provider (HIGH).
- Other refresh-token killers (official list): 6 months of non-use; **a password change when the token has Gmail scopes**; 100 refresh tokens per account per client ID (the oldest is silently invalidated, which dev re-auth loops hit); user revocation; admin policy (`admin_policy_enforced`).

**Why it happens:**
Developers build against their own account in Testing mode, everything works, and the wall appears only at distribution time. By then the architecture (shared client ID, AI-summary data flow) is baked in.

**How to avoid (personal-use-first strategy):**
1. **v1 / owner phase:** create a dedicated GCP project, set the consent screen to External, and **publish it to "In production" without verification**. The owner clicks through the unverified-app warning once. Refresh tokens then don't expire weekly. This is Google's "personal use" exception (fewer than 100 known users). rclone documents this exact pattern (MEDIUM-HIGH).
2. **Design for BYO OAuth client from day one.** The connector reads `client_id`/`client_secret` from user-supplied config in secure storage, with the app's own client as just one option. Early distribution to friends can then use the **"bring your own Google Cloud project"** route: an onboarding-wizard step with screenshots. Each user is their own personal-use app, with no 100-user pool shared across everyone and no CASA for you. It is a clunky but legitimate path.
3. **Request minimal scopes:** `gmail.readonly` (restricted, unavoidable for reading) plus `gmail.send` (sensitive) for replies, rather than `gmail.modify` or `mail.google.com`. Use `calendar.events` (sensitive) rather than full `calendar`. Fewer restricted scopes mean an easier verification later.
4. **Use loopback redirect + PKCE:** register a "Desktop app" client and listen on `http://127.0.0.1:<random port>`. Custom URI schemes are **no longer supported** by Google, and OOB copy-paste is **removed** (HIGH). Bind explicitly to 127.0.0.1. Binding 0.0.0.0 triggers a Windows Defender Firewall prompt. Always use PKCE with S256. Treat the embedded client secret as public, as Google itself says.
5. **Defer the "verify + CASA" decision to P8** as an explicit go/no-go. Document the data flow (device → Google APIs; device → chosen LLM provider, only after consent) and get Google's written position before paying an assessor.
6. Handle `invalid_grant` gracefully. Show a "Google bağlantısı yenilenmeli" card with one-click re-auth, never a silent failure.

**Warning signs:**
- Consent screen still in "Testing" when daily use begins, or background mail checks stopping after about a week.
- A hard-coded client ID with no config override.
- Requesting `https://mail.google.com/` "because it's easier".
- No consent dialog before the first AI summary that includes email content.

**Phase to address:** P2 (scopes, loopback/PKCE, BYO client, In-production personal project, re-auth UX). P3 (AI-transfer consent). P8 (verification/CASA go/no-go). Research flag on P8.

---

### Pitfall 3: Prompt injection from email, tweets and web pages into the AI summary and actions

**What goes wrong:**
All of Mornfold's inputs (emails, tweets, scraped pages, MCP tool output) are attacker-controllable text, and they flow into an LLM. Documented real-world precedents:
- **Gemini in Gmail (0DIN, July 2025):** hidden zero-font, white-on-white instructions in an email made "Summarize this email" output a fake Google security warning with a phone number (MEDIUM-HIGH, multiple outlets).
- **EchoLeak, CVE-2025-32711 (M365 Copilot, CVSS 9.3):** a zero-click crafted email that the assistant later pulled into context led to data exfiltration (MEDIUM-HIGH).

For Mornfold the risk compounds. The morning summary mixes **private data** (all your mail and calendar) with **untrusted content** (inbound mail, tweets, web pages). Once MCP tools or "reply from app" are present, it also gains **outbound capability**. Private data, untrusted input and an exfiltration channel together form the "lethal trifecta".

**Why it happens:**
"It's just a summary" feels read-only. But summaries are *rendered* (markdown links, images, which can leak data via image URLs), *trusted* by the user ("the app told me to call this number"), and later get wired to tools.

**How to avoid:**
1. **Render AI output as plain text** (the project already says "düz metin"). Do not auto-render markdown images or remote URLs from model output, and strip or neutralize links. That alone kills the image-URL exfil class.
2. **Pre-process inputs:** convert HTML to text and **drop hidden content** (display:none, zero font-size, color matching the background, off-screen). Truncate per item. Wrap each item in clear delimiters with a "this is untrusted data, never instructions" system framing. This mitigates but does not prevent.
3. **No tool access in the summary call.** The summary LLM call gets data in and text out, with zero tools and zero MCP. Keep MCP tool-use flows separate, user-initiated, and never fed raw inbox content automatically.
4. **Every side effect needs a human click:** send reply, create calendar event, call an MCP tool with write semantics. Show the exact payload. No "auto-reply" or "auto-act" in v1.
5. Attribute every summary bullet to its source item, with a link to the original in-app, so users can verify claims such as "your account was compromised".
6. Add a red-team fixture set (hidden-text email, instruction-override tweet, malicious page) to the test suite for P3/P4/P6.

**Warning signs:**
- The summary call's request includes `tools` or MCP definitions.
- The model output is rendered through a markdown renderer that allows `<img>` or auto-links.
- Phone numbers or URLs appear in the summary that don't exist verbatim in the source items.

**Phase to address:** P3 (summary pipeline design). Re-checked in P4 (web content), P6 (MCP), and P7 (tweets).

---

### Pitfall 4: MCP client as an arbitrary-code-execution and data-exfiltration hole

**What goes wrong:**
- **A local stdio MCP server is an arbitrary program running with the user's privileges.** The spec's own security page shows malicious "startup commands" (e.g. `npx malicious-package && curl ... ~/.ssh/id_rsa`). It says that clients offering one-click server setup **MUST show the exact full command, warn that it executes code, and require explicit approval** (modelcontextprotocol.io security best practices, HIGH).
- **Tool poisoning:** malicious instructions sit in tool *descriptions* that the model reads but users never see. **Rug pulls:** a server changes its descriptions after approval. **Shadowing:** one server's descriptions hijack calls to another server. CVE-2025-54136 is cited as a real rug-pull instance. OWASP lists this as MCP03 in its MCP Top 10 (MEDIUM: secondary sources, consistent across CSA, arXiv 2506.02040 and CyberArk).
- **Malicious server metadata:** remote-server OAuth discovery URLs can point at internal IPs (SSRF). `javascript:`/`file:` authorization URLs lead to XSS. URLs opened via `cmd.exe`/PowerShell lead to command injection. The spec says clients **MUST** allow only http(s) and **MUST NOT** open URLs via a shell (HIGH).
- **Protocol churn:** the **current MCP version is 2026-07-28**, and it is a breaking redesign. It removed the `initialize` handshake (stateless) and protocol sessions. It added a mandatory `server/discover` and replaced server-initiated requests with Multi Round-Trip Requests. It **deprecated Sampling, Roots and Logging** and deprecated Dynamic Client Registration in favor of Client ID Metadata Documents (HIGH, official changelog). Most community servers in the wild still speak 2025-11-25 or earlier. A hand-rolled client will break on one side or the other.

**Why it happens:**
"Add any MCP server without touching code" is a core requirement. That makes the easy path ("paste a JSON config and we spawn it") the dangerous path.

**How to avoid:**
1. Use the **official MCP SDK** for whatever runtime is chosen, at a version that implements 2026-07-28 *and* backward compatibility with handshake-based servers. Never hand-roll the protocol.
2. **Consent dialog on add:** show the full command and args, untruncated, highlight risky patterns (shell operators, `curl`, paths under the user profile), and require explicit approval.
3. **Pin tool definitions:** hash each server's tool list (names, descriptions, schemas) at approval time. On change, disable the server until the user re-approves with a visible diff. This counters rug pulls.
4. **Per-tool permission levels:** read-only tools can auto-run in user-initiated chats. Anything with side effects requires per-call confirmation. Never expose MCP tools to the background summary job (see Pitfall 3).
5. Don't implement the deprecated Sampling/Roots features at all.
6. Treat tool *results* as untrusted content with the same sanitization as email.
7. Validate URLs from servers (scheme allowlist; block private ranges for remote OAuth discovery). Open them with the OS URL API, never a shell.

**Warning signs:**
- A config import that spawns processes without a confirmation UI.
- Tool descriptions are never shown to the user.
- No stored hash or approval record per server.

**Phase to address:** P6 (MCP client). The connector framework (P2) should define the permission/consent model so MCP and built-in connectors share it. Research flag for P6: SDK support status for 2026-07-28 in the chosen runtime.

---

### Pitfall 5: Windows stdio process management for MCP servers (and any sidecars)

**What goes wrong:**
- `npx`/`npm`/`uvx` launchers on Windows are often `.cmd` shims. Since the CVE-2024-27980 fix (Node 18.20.2 / 20.12.2+), **spawning a `.cmd`/`.bat` without `shell: true` throws `EINVAL`**. Without a shell, `npx` gives `spawn ENOENT` (HIGH, Node advisory). Rust's `std::process::Command` has analogous batch-file argument-escaping hazards (the BatBadBut class).
- **Orphaned children:** on Windows, child processes survive the parent's exit unless they are placed in a **Job Object with `JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE`**. A 2026 Claude Code issue reported exactly this, with zombie MCP servers piling up across restarts (MEDIUM).
- **Console windows flash** for each spawned process unless created with `CREATE_NO_WINDOW`.
- **End users don't have Node or Python.** Most community MCP servers assume `npx`/`uvx` exist. "Add an MCP server" fails for non-developers with cryptic errors.
- **Smart App Control (Windows 11)** blocks unsigned executables without positive reputation, and this applies to *all* executables, not only downloads (HIGH, Microsoft SmartScreen doc). Unsigned helper binaries or sidecars you ship may simply not run on SAC-enabled machines.

**How to avoid:**
1. Resolve commands explicitly. Prefer launching a real `.exe` (`node.exe path/to/cli.js`). When a `.cmd` is unavoidable, use `cmd.exe /d /s /c` with strict argument quoting and validation.
2. Spawn every child into a Job Object with kill-on-close. Use `CREATE_NO_WINDOW`. Capture stderr to a per-server log, because the stdio protocol uses stdout.
3. Add health checks with restart backoff, and show a per-server status indicator in the UI.
4. Prerequisite detection: "This server needs Node.js 20+. It isn't installed." Link to the installer. Consider first-party support for remote (Streamable HTTP) servers as the non-developer path.
5. Sign every binary you ship, including sidecars.

**Warning signs:**
- Growing counts of `node.exe` processes in Task Manager after restarting the app.
- MCP works in dev on the Linux cloud box but fails on the real Windows PC.

**Phase to address:** P6 (MCP), with the Job Object utility built in P1 if any sidecar exists. It must be verified on real Windows, not CI only.

---

### Pitfall 6: Building and verifying Windows behavior from a Linux dev environment

**What goes wrong:**
Development happens in a Linux cloud environment, but every differentiating behavior is Windows-only: tray, toasts, Credential Manager/DPAPI, autostart, installer, SmartScreen, glass effects.
- **Tauri:** cross-compiling Windows from Linux is officially "possible with caveats" and "not tested as much". It produces **NSIS only (no MSI)**, and **signing cross-compiled builds is unsupported** (v2.tauri.app, HIGH). Tauri's own advice is a Windows VM or CI.
- **Electron:** electron-builder can produce NSIS from Linux via Wine, but MSIX/AppX targets and signing tooling are Windows-centric (MEDIUM).
- Toasts **don't appear at all** for desktop apps without a Start Menu shortcut carrying an **AppUserModelID**, plus a ToastActivatorCLSID for activation (Microsoft and Electron docs, HIGH). Dev builds commonly show nothing, or show "PowerShell"/"electron.exe" as the sender.
- Glass effects: `window-vibrancy` Acrylic has **bad resize/drag performance on Windows 10 1903+ and Windows 11 build 22000**. Blur lags on Win11 22621+. **Mica is Windows 11 only** (official README, HIGH). An Apple-like glass design that is smooth in screenshots can feel janky on the actual machine.

**How to avoid:**
1. **P1 deliverable:** a GitHub Actions `windows-latest` workflow that builds the real installer on every push and uploads the artifact, plus a written "Windows smoke test" checklist the owner runs on their PC (tray, toast, autostart, Credential Manager round-trip, sleep/resume).
2. Install via the real installer before testing notifications, so the shortcut and AUMID exist. Set an explicit AUMID in the installer.
3. Design glass with a **graceful fallback**: Mica on Win11, a solid/tinted background on Win10. Avoid Acrylic on resizable main windows. Decide in P1, not after the UI is built.
4. Never treat "it compiles for Windows" as "it works on Windows".

**Warning signs:**
- Several phases complete without anyone installing a build on the Windows PC.
- Notification code is "done" but was only tested in dev mode.

**Phase to address:** P1 (CI plus the smoke-test loop is the very first thing built). This is foundational.

---

### Pitfall 7: Code signing, SmartScreen and auto-update discovered at the end

**What goes wrong:**
- **Unsigned installers** get "Windows protected your PC" with a hidden "Run anyway". Enterprise policy can block them entirely. **Smart App Control** can block unsigned executables outright (HIGH).
- **EV certificates no longer bypass SmartScreen (since 2024).** OV, EV and Artifact Signing all build reputation over time: "several weeks and hundreds of clean installs" (Microsoft Learn, updated Aug 2026, HIGH).
- **Azure Artifact Signing (~$9.99/mo)** is available to *individual* developers **only in the USA and Canada**, and to organizations in the US, Canada, EU and UK (HIGH). **A Turkey-based individual cannot use it.** The fallback is an OV certificate ($150-300/yr). OV keys must live on an HSM or token (CA/B Forum rule since June 2023), so CI signing needs a cloud-HSM product from the CA.
- **Since March 1, 2026, code-signing certificates are limited to 460 days of validity** (CA/B Forum ballot CSC-31, HIGH). Expect yearly renewal, and plan for the renewal's effect on publisher reputation.
- **The Microsoft Store (MSIX) is free, worldwide, and has no SmartScreen warnings**, because Microsoft re-signs the package (HIGH). It is the best fit for an individual in Turkey. It constrains you, though: updates go through the Store (no custom auto-updater), autostart goes through the MSIX StartupTask, and you need a privacy policy and certification. MSI/EXE submissions to the Store must still be signed by you.
- **Auto-update key loss:** Tauri's updater signature check **cannot be disabled**. Lose the private key and existing installs can never auto-update again. electron-updater verifies `publisherName` against the certificate, so certificate rotation needs both names listed (MEDIUM-HIGH).

**How to avoid:**
1. Decide the distribution channel in the **stack decision**, because it affects framework choice. Electron-builder has an MSIX/AppX target. Tauri v2's built-in bundlers are NSIS/MSI (MSIX support status: LOW confidence, verify). For an individual in Turkey, the recommended order is **(a) Microsoft Store MSIX**, then **(b) an OV certificate with cloud HSM plus NSIS and a self-updater**.
2. For owner-only v1, unsigned is acceptable (the owner clicks "Run anyway"). Do not distribute to others unsigned.
3. Generate the updater signing key in P1. Back it up offline in two places and store it as a CI secret. Document a rotation plan.
4. Use a consistent publisher identity across every release, so reputation accumulates.

**Warning signs:**
- "We'll figure out signing later" when P8 is less than two phases away.
- The updater private key exists only on the cloud dev box.

**Phase to address:** Stack decision / P1 (channel choice, updater key). P8 (actual signing, Store submission, onboarding).

---

### Pitfall 8: Background scheduler that silently stops (sleep, throttling, hidden windows)

**What goes wrong:**
Monitoring is the product, and Windows actively fights background work:
- On Modern Standby, the Desktop Activity Monitor **pauses desktop apps** after the system enters standby. Background activity on battery may be throttled unless the user allows it to run in the background (Microsoft Learn, HIGH). Timers fire late or appear dead after resume. Resume callbacks are reported as unreliable on some machines (MEDIUM, forum evidence).
- Polling loops written in **renderer/webview JavaScript** get throttled when the window is hidden or minimized. Chromium background timer throttling and Electron `backgroundThrottling` both apply (MEDIUM). A tray app's window is hidden most of the time.
- **Resume floods:** after 8 hours of sleep, every monitor fires at once, X/AI credits spike, and 30 toasts appear.
- Autostart breaks: the user toggles the app off in Startup Apps, there is a duplicate instance, or the Run key points to the old install path after an update.

**How to avoid:**
1. Run the scheduler in the **native/main process** (Rust core in Tauri, or the main process in Electron), never in UI JS.
2. Use wall-clock catch-up scheduling: on every tick, compare `now - lastRun` and run overdue jobs. On resume (or detected clock jump), stagger catch-up with jitter and a concurrency limit.
3. Persist `lastRun`, `lastSeenId`, `lastContentHash` and `historyId` in SQLite, so restarts don't re-alert.
4. Enforce a single-instance lock. Use an autostart mechanism matched to the channel (Run key for NSIS, StartupTask for MSIX). Show an "Arka planda çalışıyor, son kontrol: 3 dk önce" health line in the tray menu.

**Warning signs:**
- A "last checked" timestamp that is hours old while the PC was awake.
- Bursts of notifications right after unlocking the PC.

**Phase to address:** P1 (scheduler, persistence, single instance, autostart). Every monitor phase plugs into it.

---

### Pitfall 9: Notification spam and false "changed!" alerts destroying trust

**What goes wrong:**
- **First-run and baseline bug:** on first poll, every post or line is "new", so the app sends 50 toasts. The same happens after state loss or reinstall.
- **Website noise:** rotating ads, "updated 3 minutes ago" timestamps, view counters, CSRF tokens, cookie banners, A/B variants and carousels each trigger a "page changed" alert every poll.
- Keyword monitoring on X turns into a firehose.
- Do Not Disturb (Windows 11, formerly Focus Assist) sends toasts silently to Notification Center. Users think alerts are broken. Users who get spammed disable the app's notifications in Windows Settings, permanently.

**How to avoid:**
1. Establish a baseline silently on add ("İzlemeye başlandı"). Notify only on diffs after the baseline.
2. Website monitors must use a **CSS selector or region by default** (the visual picker), plus ignore-patterns (dates, numbers-only lines). Normalize text (collapse whitespace, strip digits in "N minutes ago" patterns) and show a "test now: what changed?" preview before saving.
3. Global and per-source rate limits. Batch into digests ("X'te 7 yeni paylaşım"). Quiet hours. Priority levels (only "important" items get toasts; everything else goes to the in-app feed).
4. Use toast grouping/tags so updates replace rather than stack. Every toast deep-links to the item.

**Warning signs:**
- More than ~10 toasts a day on the owner's machine during dogfooding.
- A monitor whose diff history shows changes on every single poll.

**Phase to address:** P1 (notification service with throttling/digest primitives). P4 (website noise handling). P7 (X digesting).

---

### Pitfall 10: Website change detection: JS rendering, bot walls, and running untrusted pages inside the app

**What goes wrong:**
- Plain HTTP fetch returns an empty shell for SPA/JS-rendered pages, so "no change ever" is reported.
- Bundling headless Chromium (Playwright/Puppeteer) adds 150MB+ and a second browser to update.
- Cloudflare-style challenges and anti-bot systems on e-commerce and classifieds sites block automated fetches. A monitor either fails silently or reports the challenge page itself as "the content changed" (MEDIUM: general knowledge; no authoritative per-site source).
- **Security trap (Tauri especially):** rendering arbitrary third-party pages in a hidden app webview that has **IPC/command access** gives any monitored site a path to your native commands. In Electron, the equivalent is loading remote content with `nodeIntegration` or a permissive preload.
- Legal and etiquette: aggressive polling (every minute) of small sites, ignoring robots.txt, or monitoring logged-in pages against site ToS.

**How to avoid:**
1. Use a two-tier fetcher: (a) HTTP GET with ETag/If-Modified-Since and an honest User-Agent as the default; (b) a "render with browser" option that uses the system WebView2 in an **isolated, capability-less hidden window** (no IPC, no preload, separate session/partition, navigation locked to the monitored origin). Don't bundle Chromium.
2. Detect challenge/blocked pages (HTTP 403/429/503, known challenge markers) and surface "Bu site otomatik erişimi engelliyor" instead of a false diff. Do not build bot-wall bypassing (stealth plugins, CAPTCHA solvers), which is legal and ToS risk for a distributed app.
3. Use a per-domain minimum interval (default 15-60 min, floor around 5 min), respect robots.txt by default with a visible override, and back off on errors.
4. Store normalized text snapshots plus a diff, not full HTML. Cap history per monitor.

**Warning signs:**
- A monitor that has "never changed" for weeks on a page you know updates.
- Diff content that contains "Checking your browser" or "Just a moment".

**Phase to address:** P4. Webview isolation rules get decided in P1 (security capability model).

---

### Pitfall 11: "Everything in v1" means never shipping

**What goes wrong:**
PROJECT.md records "Tüm takip özellikleri v1'de". The integration surface is: Gmail read/reply, Calendar read/add, the AI summary with 3 providers, website monitoring, X accounts plus keywords, an API catalog, custom API cards, an MCP client, tray, toasts, an installer, an onboarding wizard, and secure storage. That is about 8 independent external dependencies, each with its own auth, quota, failure and cost model. Two of them (X and Google verification) have *policy* risk outside your control. A solo project that builds them breadth-first hits 60%-done-everywhere with nothing dogfoodable, and loses momentum.

**How to avoid:**
1. Read "v1 = all features" as **"v1 *milestone* = all features, shipped in thin vertical slices the owner uses daily from slice 2 onward"**. Every phase ends with something installed on the owner's PC.
2. Order by dependency and risk:
   - Foundation: shell, tray, scheduler, notifications, secure store, Windows CI. This is the Core Value: "the elegant shell plus the service-connection infrastructure must keep working even if everything else breaks".
   - Connector framework plus Google, the first real value.
   - AI summary, which creates the "wow" moment.
   - Then the cheap, self-contained monitors (website, custom API cards).
   - Then MCP.
   - X last, because of cost and policy volatility.
   - Distribution concerns (signing, Store, verification, onboarding for others) form their own final phase. They gate *other users*, not the owner.
3. Hold each connector to a *definition of done*: auth renewal, error state UI, rate-limit handling, cost display, and a test against the real service. This prevents half-built connectors.
4. Keep an explicit cut line. If time runs out, X keyword search and the public API catalog are the first deferrable items. The connector framework, Google and the summary are not.

**Warning signs:**
- More than two connectors "in progress" simultaneously.
- Three or more phases without the owner using the app in the morning.
- UI polish (glass, animations) consuming phases before data flows are real.

**Phase to address:** Roadmap structure itself, enforced at every phase transition.

---

### Pitfall 12: BYO AI keys: cost blow-up, PII leakage and key handling

**What goes wrong:**
- **Cost:** summarizing full raw HTML emails (newsletters can run 50-100KB), re-summarizing unchanged items every run, or using a frontier model for triage drives large bills on the user's own key.
- **Privacy:** the **unpaid Gemini API tier** uses submitted content "to provide, improve, and develop products". Human reviewers may read it, and Google explicitly says **"Do not submit sensitive, confidential, or personal information to the Unpaid Services."** The paid tier does not use prompts for training (ai.google.dev/gemini-api/terms, HIGH). A user who pastes a free AI Studio key is therefore sending their entire inbox to a training pipeline. **OpenRouter** doesn't log prompts by default, but routes to providers with varying retention and training policies. ZDR must be enforced via account settings or the per-request `zdr` flag, and "no training" is not the same as "no retention" (MEDIUM-HIGH, openrouter.ai docs).
- **Keys:** plaintext keys in config, JSON or localStorage; keys logged in error traces; keys accessible to webview JS.
- Credential Manager blobs are capped at **2560 bytes** per credential, so large OAuth token bundles fail to save (MEDIUM, multiple library sources). DPAPI and Credential Manager protect against *other users* and offline disk theft, **not** against malware running as the same user. Set expectations accordingly.

**How to avoid:**
1. Pipeline: HTML to clean text, strip quoted reply chains, signatures and tracking. Truncate per item (e.g. 2-4K chars). Cache summaries by message ID or content hash. Use a cheap model for per-item triage and a better model only for the final digest. Set max_tokens.
2. Track **per-provider token usage and estimated cost locally**, and enforce a user-set daily or monthly cap. Show "Bu ayki AI maliyeti tahmini: $x".
3. Detect the key or tier where possible. In provider settings, show a clear notice that free-tier Gemini may be used for training, and recommend paid keys or OpenRouter ZDR. Offer a "redact before sending" option (email addresses, phone numbers, IBAN/TCKN patterns), which matters for Turkish personal data.
4. Store secrets in Credential Manager or DPAPI from the native side only. Store only the refresh token, not the full token JSON, or chunk it. The UI never receives raw keys after entry (show only the last 4 characters). Redact secrets in logs.

**Warning signs:**
- A summary run exceeding ~50K input tokens for a normal morning.
- Secrets appearing in the SQLite file, log files or devtools.

**Phase to address:** P1 (secret store abstraction, log redaction). P3 (pipeline, cost tracking, provider privacy notices).

---

## Technical Debt Patterns

| Shortcut | Immediate Benefit | Long-term Cost | When Acceptable |
|----------|-------------------|----------------|-----------------|
| Leaving the Google consent screen in "Testing" | Zero setup | Re-auth every 7 days, dead background sync | Only during the first days of P2 development |
| Hard-coding the owner's Google client ID | Faster P2 | Can't offer BYO-client distribution, and the shared 100-user cap gets consumed | Never. Read it from config, with a default |
| Polling in webview JS (setInterval) | Quick to write | Throttled when hidden, so missed alerts | Never for monitors. OK for UI clocks |
| Full-HTML snapshots for website diffs | Simple | Noise alerts, DB bloat | Never. Normalize to text or the selected region |
| One mega-prompt with all emails, tweets and pages | Simple summary code | Cost blow-up, injection blast radius, no caching | Prototype only, then replace in P3 |
| Spawning MCP servers without Job Objects | Less Win32 code | Orphan processes, locked files on update | Never |
| Unsigned owner builds | No cert cost | SmartScreen and Smart App Control friction | Owner-only v1. Never for other users |
| Updater key kept only on the dev box | Convenience | Permanent loss of the update channel | Never |
| Scraping X instead of the API | Saves money | ToS/legal exposure, breakage (see Nitter, Aug-Sep 2026) | Never in the shipped core |

## Integration Gotchas

| Integration | Common Mistake | Correct Approach |
|-------------|----------------|------------------|
| Gmail sync | Re-listing the inbox each poll (`messages.list` 5 units plus `messages.get` 20-60 units each) | Do one initial sync, then `history.list` (2 units) from the stored `historyId`. On 404 (expired historyId), fall back to a bounded full sync. Per-user quota is 6,000 units/min (official) |
| Gmail push | Planning `users.watch` (needs Cloud Pub/Sub, effectively server-shaped) | Poll `history.list` every 1-5 min. That is cheap and fits local-only |
| Gmail reply | Sending a new message that doesn't thread, or garbling Turkish characters | Set `threadId`, `In-Reply-To` and `References`. Build RFC 5322 MIME with UTF-8, and RFC 2047-encode the subject ("Re: Toplantı") |
| Gmail fetch | Using `format=full` for everything | Use `metadata` for lists. Fetch full content only for items going to the summary or opened by the user |
| Calendar | Ignoring all-day events (`date` vs `dateTime`), recurring events, time zones | Use `singleEvents=true&orderBy=startTime`, render the event's tz explicitly (Europe/Istanbul is UTC+3 with no DST, but invitees may be elsewhere) |
| Google OAuth | Custom URI scheme or OOB redirect | Loopback `127.0.0.1:<random>` plus PKCE S256, with a Desktop-app client type |
| X API | Fetching user objects repeatedly | Resolve handle → ID once ($0.01) and cache it. Use `since_id`. Request minimal fields |
| X API | Trusting the console spending limit alone | Also use an in-app budget and per-item cost estimate. The console limit blocks *all* requests once hit |
| MCP | Hand-rolled JSON-RPC client | Official SDK with 2026-07-28 plus legacy handshake support |
| Custom API cards | Fetching from the webview (CORS failures, key exposure) | Fetch in the native core. The webview gets only the extracted fields |
| Custom API cards | Keys in query strings stored in plain config | Template placeholders (`{{secret:name}}`) resolved from the secret store at request time |
| LLM providers | Assuming one API shape | A provider adapter with per-provider limits, errors (429/overloaded) and retry-after handling. OpenRouter model IDs differ from native IDs |

## Performance Traps

| Trap | Symptoms | Prevention | When It Breaks |
|------|----------|------------|----------------|
| Resume thundering herd | CPU and network spike plus a toast burst after wake | Staggered catch-up with a concurrency limit (e.g. 3) | Any user with more than 10 monitors and a laptop that sleeps |
| Browser-rendered monitors running concurrently | High RAM (each WebView2 instance is 100MB+), fan noise | A queue with max 1-2 concurrent renders. Prefer the HTTP path | More than 5 JS-rendered monitors |
| Unbounded snapshot/diff history | SQLite grows to GBs, slow startup | Keep the last N snapshots plus compressed diffs. VACUUM periodically | Months of use at 15-min intervals |
| Summary over the full inbox | Slow mornings (30s+), big bills | Cap to the last 24h, preference-ranked, cached per item | More than ~100 emails/day |
| X keyword pagination | Credits burned quickly | `max_results` cap, page cap per poll, `since_id` | Any popular term |
| Heavy glass effects on Win10 | Laggy drag and resize | Mica on Win11 only. Solid fallback on Win10 | Win10 machines, Acrylic on resizable windows |

## Security Mistakes

| Mistake | Risk | Prevention |
|---------|------|------------|
| Giving the summary LLM call tools/MCP access | Injected email triggers actions or exfiltration | A no-tools summary call. Tool use only in user-initiated flows with confirmation |
| Rendering model output as markdown with images or links | Data exfiltration via image URLs, phishing links | Plain text rendering, neutralized links |
| Approving an MCP server once, forever | Rug-pull tool poisoning | Hash-pin tool definitions. Re-approve on change |
| Loading monitored sites in an IPC-enabled webview | Remote page invokes native commands | Isolated capability-less window with a separate session |
| Opening URLs via `cmd /c start` or PowerShell | Command injection from server-provided URLs | OS shell-execute API with an http(s) scheme allowlist |
| Binding the OAuth listener on 0.0.0.0 | Firewall prompt, LAN exposure | 127.0.0.1 only, single-use, short timeout, `state` validation |
| Secrets in logs or crash reports | Token leakage | Central redaction. Never log request headers or bodies for auth endpoints |
| Free-tier Gemini key used for inbox data | User data used for training, human review | Provider notice plus recommend a paid key or ZDR routing |
| Shipping a scraper for X | ToS liquidated damages, "facilitation" exposure | Official API only |

## UX Pitfalls

| Pitfall | User Impact | Better Approach |
|---------|-------------|-----------------|
| Silent failure when Google, X or AI auth expires | "The app stopped telling me things" | A persistent status card per connector, with one-click repair |
| The scary unverified-app Google screen with no warning | Users abandon onboarding | The wizard pre-explains it with a screenshot ("Gelişmiş → Devam et") |
| BYO GCP project setup as a wall of text | Non-developers give up | A step-by-step wizard with deep links into the Cloud Console, and validation of the pasted client ID |
| "Add MCP server" assumes Node or Python is installed | Cryptic spawn errors | Prerequisite detection plus a friendly message, and favor remote MCP servers for non-devs |
| No cost visibility for X and AI | Bill shock, which then gets blamed on the app | Per-feature monthly estimate plus hard caps |
| Alerts on every trivial change | Users disable notifications OS-wide | Baselines, selectors, digests, quiet hours |
| A design-first build where data is fake for long | A beautiful but useless shell | Real data in the shell from P2 onward. Polish iteratively |
| Turkish UI hard-coded in strings | Blocks the later multi-language goal | Use an i18n key system from P1, even with only `tr` present |

## "Looks Done But Isn't" Checklist

- [ ] **Google connector:** survives 8+ days without re-login (not in Testing mode). Handles `invalid_grant`. Survives a password change by prompting re-auth.
- [ ] **Gmail reply:** the reply threads correctly in Gmail web, and Turkish characters in the subject and body render correctly on the recipient's side.
- [ ] **Toasts:** appear from the *installed* build with the correct app name and icon (AUMID). Clicking one opens the right item. Behavior under Do Not Disturb is understood.
- [ ] **Tray/background:** keeps checking after close-to-tray, minimize, lock, and sleep/resume. Autostart works after reboot *and* after an update. Only one instance runs.
- [ ] **Website monitor:** a page with a timestamp or ad does *not* alert every poll. A JS-rendered page actually yields content. A Cloudflare challenge yields a "blocked" state, not a diff.
- [ ] **X monitor:** the first add sends no toasts (baseline). The monthly cost estimate is visible. Restarting the app doesn't re-alert old posts.
- [ ] **AI summary:** the hidden-text injection fixture does not alter the summary's behavior. No images or links are rendered. Cost and token count are logged per run. Consent was given before the first email was sent to the provider.
- [ ] **MCP:** the add-server dialog shows the full command. A changed tool description disables the server. Killing Mornfold leaves no orphan `node.exe`.
- [ ] **Secrets:** grep the SQLite DB, logs and webview storage for key prefixes (`sk-ant-`, `AIza`, `sk-or-`) and find nothing.
- [ ] **Installer:** installs and upgrades on a clean Windows 11 VM with Smart App Control in evaluation/on, and on Windows 10.
- [ ] **Updater:** a real v0.1 → v0.2 update is tested end to end with the signing key restored from backup.

## Recovery Strategies

| Pitfall | Recovery Cost | Recovery Steps |
|---------|---------------|----------------|
| X pricing or policy changes again | LOW if X is an isolated connector; HIGH if woven through | Keep X behind the connector interface. Disable the feature remotely-free (it's local) via an update. Keep a migration path to an MCP-based X server |
| Google refuses verification or CASA is too costly | MEDIUM | Fall back to BYO-GCP-project onboarding (each user is personal use). Documented in the wizard |
| A Google client ID gets suspended or flagged | MEDIUM | BYO client config already exists. Users paste their own |
| Updater signing key lost | HIGH | Ship a manually downloaded "bridge" release with a new key. Communicate to all users. Prevent this with offline backups |
| A notification-spam reputation hit | MEDIUM | Ship a digest-by-default update. Re-baseline all monitors on upgrade |
| A Credential Manager size failure | LOW | Store only the refresh token, or chunk it, or use a DPAPI-encrypted file fallback |
| A malicious MCP server incident | HIGH (trust) | A kill switch per server, hash-pinning already in place, and an audit log of tool calls to explain what happened |

## Pitfall-to-Phase Mapping

| Pitfall | Prevention Phase | Verification |
|---------|------------------|--------------|
| 6. Linux-dev / Windows-reality gap | P1 Foundation | CI produces an installer every push. The owner's smoke-test checklist passes on a real PC |
| 8. Background scheduler death | P1 Foundation | "Last checked" stays fresh across sleep/resume. No resume burst |
| 9. Notification spam (primitives) | P1 Foundation, then P4/P7 | Throttle, digest and quiet hours exist before any monitor ships |
| 12. Secret storage and log redaction | P1 Foundation | A grep for key prefixes finds nothing outside Credential Manager |
| 7. Channel and updater key decision | Stack decision / P1 | Channel chosen (Store MSIX vs OV+NSIS). Updater key backed up offline |
| 2. Google OAuth model | P2 Connector + Google | Personal project In production. Loopback+PKCE. BYO client config. 8-day token survival |
| 4. Permission/consent model (shared) | P2 framework design | Built-in and MCP connectors use the same permission levels |
| 3. Prompt injection | P3 AI summary | Injection fixtures pass. No tools in the summary call. Plain-text rendering |
| 12. AI cost and PII | P3 AI summary | Per-run token and cost logs, cap enforcement, provider privacy notice, AI-transfer consent |
| 10. Website fetch, isolation, bot walls | P4 Website monitoring | Isolated render window. Challenge detection. Noise fixtures |
| Custom API cards (CORS, keys) | P5 API cards | Fetches run natively. Secrets come from placeholders |
| 4./5. MCP security and Windows processes | P6 MCP client | Consent dialog, hash pinning, Job Objects, no orphans |
| 1. X cost and legality | P7 X monitoring | Official API only. Cost meter. since_id. Baseline. Re-verified pricing |
| 7. Signing, SmartScreen, Store | P8 Distribution | A clean-VM install without a block. Update path tested |
| 2. Verification/CASA go/no-go | P8 Distribution | A written decision: verify vs BYO-project onboarding |
| 11. Scope creep | Every phase transition | The owner uses the app daily from P2/P3 onward. At most 2 connectors in progress |

## Sources

**Official (HIGH):**
- X API pricing (pay-per-usage, rates, 3M cap, dedup): https://docs.x.com/x-api/getting-started/pricing
- X API rate limits: https://docs.x.com/x-api/fundamentals/rate-limits
- X API overview (pay-per-usage only): https://docs.x.com/x-api/getting-started/about-x-api
- Google restricted scope verification and CASA: https://developers.google.com/identity/protocols/oauth2/production-readiness/restricted-scope-verification
- Google security assessment FAQ (cost, tiers, annual): https://support.google.com/cloud/answer/13463817
- Google "when verification is not needed" (personal use fewer than 100): https://support.google.com/cloud/answer/13464323
- Google unverified apps (100-user cap): https://support.google.com/cloud/answer/7454865
- Google OAuth refresh token expiration (7-day Testing, Gmail password change, 100-token limit): https://developers.google.com/identity/protocols/oauth2
- Google OAuth for desktop apps (loopback, no custom schemes, OOB removed, PKCE): https://developers.google.com/identity/protocols/oauth2/native-app
- Google API Services User Data Policy (Limited Use, transfers): https://developers.google.com/terms/api-services-user-data-policy
- Gmail API quotas: https://developers.google.com/workspace/gmail/api/reference/quota
- MCP security best practices: https://modelcontextprotocol.io/specification/draft/basic/security_best_practices
- MCP versioning (current 2026-07-28): https://modelcontextprotocol.io/specification/versioning
- MCP 2026-07-28 changelog: https://modelcontextprotocol.io/specification/2026-07-28/changelog
- Microsoft code signing options (Artifact Signing geography, EV change, Store MSIX free): https://learn.microsoft.com/en-us/windows/apps/package-and-deploy/code-signing-options
- Microsoft SmartScreen reputation (and Smart App Control): https://learn.microsoft.com/en-us/windows/apps/package-and-deploy/smartscreen-reputation
- Modern Standby and desktop apps: https://learn.microsoft.com/en-us/windows-hardware/design/device-experiences/integrating-apps-with-modern-standby
- Tauri Windows installer and cross-compilation caveats: https://v2.tauri.app/distribute/windows-installer/
- Tauri updater (signature mandatory): https://v2.tauri.app/plugin/updater/
- window-vibrancy platform and performance notes: https://github.com/tauri-apps/window-vibrancy
- Electron notifications (AUMID / Start Menu shortcut): https://www.electronjs.org/docs/latest/tutorial/notifications
- Node.js CVE-2024-27980 (.cmd/.bat spawn EINVAL): https://nodejs.org/en/blog/vulnerability/april-2024-security-releases-2
- Gemini API terms (unpaid vs paid data use): https://ai.google.dev/gemini-api/terms
- OpenRouter ZDR: https://openrouter.ai/docs/guides/features/zdr

**Secondary (MEDIUM/LOW, flagged inline):**
- Code signing 460-day validity (CA/B Forum CSC-31): https://www.digicert.com/blog/understanding-the-new-code-signing-certificate-validity-change
- X ToS liquidated damages: https://knightcolumbia.org/content/knight-institute-says-xs-new-terms-of-service-will-stifle-independent-research
- Nitter C&D and shutdown (Aug 2026): https://gigazine.net/gsc_news/en/20260826-nitter-end/ ; https://wwwhatsnew.com/2026/09/16/nitter-xcancel-muertos-x-legal-acciones-cierre-definitivo-2026
- X v. Bright Data dismissal: https://proxyway.com/news/twitter-sues-bright-data
- X pricing third-party summaries (legacy tiers): https://postproxy.dev/blog/x-api-pricing-2026/ ; https://www.blotato.com/blog/twitter-api-pricing
- xAI x_search tool pricing: https://docs.x.ai/developers/pricing.md
- Gemini Gmail summary injection (0DIN): https://0din.ai/blog/phishing-for-gemini ; https://bleepingcomputer.com/news/security/google-gemini-flaw-hijacks-email-summaries-for-phishing
- EchoLeak CVE-2025-32711: https://thehackernews.com/2025/06/zero-click-ai-vulnerability-exposes.html
- MCP tool poisoning / rug pulls: https://arxiv.org/pdf/2506.02040 ; https://labs.cloudsecurityalliance.org/research/csa-research-note-mcp-tool-poisoning-ai-agent-exfiltration-2/ ; https://www.cyberark.com/resources/threat-research/poison-everywhere-no-output-from-your-mcp-server-is-safe
- Windows MCP orphan processes / Job Objects: https://claudeissues.com/issue/57740-bug-windows-mcp-child-processes-orphan-on-session-exit-suggest-jobobject-kill-on
- rclone personal OAuth client pattern (publish to avoid 7-day expiry): https://rclone.org/drive/
- Credential Manager 2560-byte limit: https://example-code.com/c/windows_credentials_manager_auto_split.asp
- Website monitoring false positives: https://pagecrawl.io/blog/reduce-website-monitoring-false-positives

---
*Pitfalls research for: local-first Windows command-center app (Mornfold)*
*Researched: 2026-10-18*
