# Progress — Super Downloads

> Implementation log. One entry per session.

---

## Session 1 — 2026-03-24 — Full Project Bootstrap

### Phase 0: Foundation — COMPLETE
- Full audit of Super Downloads + Super Prompts (transferable patterns)
- 8 documentation files created (CLAUDE.md, ROADMAP.md, BRAND.md, DECISIONS.md, DIAGNOSTIC.md, LAUNCH.md, MARKETING.md, CHANGELOG.md)
- All strategic decisions made: freemium €29 lifetime, superdownloads.app, LemonSqueezy, Astro, blue brand
- Domain registered (Hostinger), email support@superdownloads.app registered
- Competitive research: 4K Download (€30.25 Personal, €48.40 Pro)
- Tauri updater research: uses own Ed25519 signing, no Apple Dev ID needed
- Quick wins: Cargo.toml author, CSP, version in UI, git init

### Phase 1: App Polish — COMPLETE
**Must-haves (7/7):**
- Empty state with icon + platforms hint
- Toast notification system (replaced all 5 `alert()` calls)
- "Link copied" feedback toast
- "Open File" in context menu
- Dynamic version from Tauri API
- "Keep download history" toggle in settings (connected to Rust backend)
- Audio-only/auto-add reset confirmed intentional

**Should-haves (4/4):**
- Drag & drop URLs from browser (multi-URL, visual drop zone)
- Native macOS notifications on download complete (osascript, unfocused only)
- Theme selector: Dark / Light / System (follows macOS `prefers-color-scheme`)
- Cleaned 12 debug console.logs

**Could-haves (2/3):**
- Custom confirm modal (replaced both `window.confirm()` calls)
- Removed ~180 lines of dead `settings-compact` CSS
- Batch URL paste — deferred to post-launch

### Phase 2: Product Infrastructure — NEAR COMPLETE
- Freemium system: 5 downloads/day counter, midnight reset, localStorage
- Download counter badge in top bar (color-coded: normal/low/exhausted/pro)
- Pro user detection via license key in localStorage
- Limit enforcement: blocks download with toast when exhausted
- First-run onboarding: welcome screen with icon, features, free limit info
- About screen: click version label in settings (icon, version, email, website)
- Auto-updater: DEFERRED — key generation needs interactive terminal

### Phase 3: Landing Page — IN PROGRESS
- Astro v6 project created in `web/` subfolder
- Full landing page: hero, 6 feature cards, pricing (Free vs Pro €29), download section, FAQ (6 items)
- Terms of Service page
- Privacy Policy page
- Favicon + icon from app identity
- Build: 3 pages in 348ms
- Pending: Vercel deploy, DMG download URLs, OG image, analytics, domain DNS

### Commits
```
02e234a feat: initial commit — Super Downloads v1.1.0 with full project structure
fd89344 docs: add app audit and restructure roadmap with new Phase 1
f2a8c14 feat: Phase 1 must-haves — toast system, empty state, open file, history toggle
1084f94 feat: Phase 1 should-haves — drag & drop, notifications, system theme
626d873 feat: Phase 1 could-haves — custom confirm modal, dead code cleanup
2af8a38 feat: Phase 2 — freemium logic, onboarding, about screen
5919e5c feat: Phase 3 — Astro landing page with full content
```

### What needs founder action
1. **Test the app** — `npm run tauri dev` to see all changes
2. **Generate Tauri updater keys** — `! npx tauri signer generate -w src-tauri/.tauri-update-key`
3. **Deploy landing to Vercel** — connect web/ folder
4. **Point DNS** — superdownloads.app → Vercel (in Hostinger)
5. **Create LemonSqueezy account** — set up product + license keys
6. **Build fresh DMGs** — for the download section of the landing
7. **Take new screenshots** — for OG image and marketing

### Next session
- Phase 4: Billing (LemonSqueezy integration, license validation in app)
- Phase 5: Pre-Launch (screenshots, assets, beta testing)
- Phase 3 remaining: Vercel deploy, DNS, OG image
