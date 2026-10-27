# Docs index — Super Downloads

> One line per document: what it answers and when to read it. Updated 2026-10-27.
> This repo is **public**: plans, strategy, metrics and pricing work live in LifeOS (`08_Growth/super-downloads/`), not here.
> When two documents disagree, the newest dated entry in `docs/DECISIONS.md` wins.

## Current

| Document | Answers | Read it when |
|---|---|---|
| `../NEXT.md` | What happened last and what the founder has to do next | Starting a session |
| `../ROADMAP.md` | Tracked work items (R-SD-…), status and dates | Picking what to build |
| `DECISIONS.md` | Every product decision, newest first, with what it supersedes | Before changing anything the product promises |
| `ARCHITECTURE.md` | How the Tauri app, the bundled engine and the website fit together | Touching Rust or the download pipeline |
| `DEVELOPMENT.md` | Setup, dev mode, checks, tests | First build on a machine |
| `OPS.md` | Release pipeline, health checks, live tests | Cutting a release or after an incident |
| `PLATFORM-HEALTH.md` | Per-platform download protocol (YouTube, Vimeo, Instagram, X, TikTok…) | A platform starts failing |
| `SIGNING.md` | Apple signing + notarization, step by step | The Apple Developer account exists (January 2027) |
| `LAUNCH.md` | Technical launch gates (gates 1–3; gate 4, payments, void since 2026-10-03) | Before the public launch |
| `SECURITY-NOTES.md` | Threat notes, CSP, secrets, update signing | Security review or a dependency alert |
| `BRAND.md` | Name, voice, colours, what we never say. §3 has a pending founder edit (Super Apps family) | Writing any copy |
| `../DESIGN.md` | Tokens, type, components, motion, copy patterns (web + app + STUDIO pieces) | Designing UI or a piece |
| `support/MACROS.md` | Ready answers for the usual support emails | Answering support@ |
| `support/SUPPORT.md` | Where users can write to us, where each channel lands, top questions | Setting up or auditing support |
| `ANNOUNCEMENT-DRAFT.md` | Verified technical facts for the announcement (texts paused) | Writing launch copy — use the facts, not the old texts |
| `repo-intel-references.md` | Other repos studied and what we took from them | Considering a new approach |
| `qa/` | Dated QA passes (what was tested, what broke, what was fixed) | Before a release |

## Historical (`archive/`, kept, not current)

| Document | Why it is archived | Replaced by |
|---|---|---|
| `archive/LAUNCH-PLAN-2026-03.md` | Paid launch (€29, LemonSqueezy, LAUNCH30) — the app went free | Private go-to-market plan (LifeOS) |
| `archive/MARKETING-2026-03.md` | Old prices and a personal-account announcement (no person on public surfaces since 2026-10-25) | Private go-to-market plan (LifeOS) |
| `archive/APP-AUDIT-v1.1.0.md` | UX audit of v1.1.0; items closed or reconciled 2026-10-23 | `DECISIONS.md`, `../CHANGELOG.md` |
| `archive/DIAGNOSTIC-v1.1.0.md` | Product gaps of v1.1.0; reconciled 2026-10-23 | `DECISIONS.md`, `../CHANGELOG.md` |
| `archive/2026-07-16-relaunch-hardening-design.md` | Design spec of the July relaunch hardening, shipped | `../CHANGELOG.md` |
| `archive/NEXT-history-2026.md` | Older NEXT breadcrumbs | `../NEXT.md` |
| `../PROGRESS.md` | Pre-launch progress log, last entry 2026-08-10 | `../CHANGELOG.md`, `../NEXT.md` |

## Contradictions found on 2026-10-27 (resolved in favour of the newest decision)

- `LAUNCH.md` gate 4 (payments) vs `DECISIONS.md` 2026-10-03 «No payments provider» → gate 4 is void. Note: the founder reopened the pricing question on 2026-10-27; any change will be a new `DECISIONS.md` entry.
- `CONTEXT.md` said v1.5.4 → current is v1.5.7 (`CLAUDE.md`, `NEXT.md`).
- `BRAND.md` §3 «not part of a brand family (yet)» vs `DECISIONS.md` 2026-10-26 (Super Guides P.S. by email) → the decision wins; the §3 text change waits for the founder.
