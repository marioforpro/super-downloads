# DESIGN.md — Super Downloads

> The single design reference for Super Downloads: the website (`web/`), the Mac app (`src/`), and any
> video, graphic or share card made for it. Built from the live code on 2026-10-25 and checked on
> superdownloads.app. When this file and the code disagree, the code wins and this file gets fixed.
> Brand rules (name, tagline, positioning) live in `docs/BRAND.md`; this file is how they look and move.

---

## 1. Brand in one paragraph

Super Downloads is a native Mac app that saves online video as edit-ready H.264 MP4 files, for video
editors and content creators who live in Premiere Pro, DaVinci Resolve and Final Cut. It should feel
like a professional tool that stays out of the way: dark navy, one lit blue, real screenshots, short
sentences. Quiet confidence — it does one thing and does it well.

**Voice.** The brand speaks, never a person: «we» on legal pages, the product name or the imperative
everywhere else. No founder name, face or voice anywhere (site, app, emails, videos).
- Plain, concrete, editor vocabulary: *NLE, timeline, H.264, save folder, reference board*.
- Short declaratives. One idea per sentence. Numbers over adjectives («up to 4K», «90 seconds»).
- Honest limits stated next to the claim («on a best-effort basis», «stops at DRM»).
- Never: piracy-adjacent words («rip», «grab anything», «bypass»), AI hype, memes, exclamation marks,
  urgency tricks, crossed-out prices, «beta», «founder».
- Tagline (locked): **The video downloader built for editors.**

---

## 2. Color

### Website (`web/src/layouts/Layout.astro`, `:root`)

Light page (`color-scheme: light only`) with navy bands for the hero, demo and download sections.

| Token | Hex | Role |
|---|---|---|
| `--navy-950` (= `--navy-dark`) | `#0a163f` | Dark bands (nav, hero, demo, download), button text on blue, `theme-color` |
| `--navy-800` (= `--navy-mid`) | `#162b73` | Secondary navy: small text on blue buttons, keycap glyphs, ghost-button hover |
| `--blue` (= `--blue-bright`) | `#30a7ff` | The one accent: button fills, focus ring on navy, glows |
| `--blue-200` (= `--blue-light`) | `#8cd0ff` | Links and highlights on navy, LED edges |
| `--blue-50` (= `--blue-pale`) | `#d5ebff` | Body text on navy (at 72–84 % opacity) |
| `--ink` (= `--gray-900`) | `#0f172a` | Headings and strong text on light |
| `--slate` (= `--gray-600`) | `#475569` | Body text on light |
| `--mist` (= `--gray-100`) | `#f4f7fc` | Alternate light band (Features), keycap fill |
| `--line` (= `--gray-200`) | `#e2e8f0` | Hairlines, dividers, FAQ rules |
| `--link` | `#0b6bcb` | Links and focus ring on white |
| `--white` | `#ffffff` | Page background, text on navy |
| `--gray-400` | `#94a3b8` | **Decorative only** (fails as text, 2.56:1) |
| footer fine print | `#5b6b80` | Legal fine print on white (literal in `index.astro`) |

The two token sets (`--navy-dark…` and `--navy-950…`) are aliases of the same brand palette
(`docs/BRAND.md` § 7); new code uses the second set.

Button gradient (literal in `.btn-blue`): `#7cc8ff → #30a7ff → #1f93ef`, top to bottom.
macOS window chrome (screenshots): dark `#1c1c1e` / bar `#2a2a2c`; light `#fff` / bar `#ececec`;
traffic lights `#ff5f57 #febc2e #28c840`.

### Contrast pairs (WCAG 2.x, measured)

| Pair | Ratio | Use |
|---|---|---|
| `--ink` on white | 17.85 | AAA |
| `--slate` on white / on `--mist` | 7.58 / 7.06 | AAA body |
| `--link` on white | 5.28 | AA links |
| `#5b6b80` on white | 5.44 | AA fine print |
| `--navy-950` on `--blue` | 6.75 | AA button labels |
| `--navy-800` on `--blue` | 4.98 | AA small button text |
| white on `--navy-950` | 17.51 | AAA |
| `--blue-50` @ 84 % / 72 % on navy | 10.33 / 7.83 | AAA body on navy |
| `--blue-200` on navy | 10.49 | links on navy |
| **white on `--blue`** | **2.59** | **fails — never** |
| **`--gray-400` on white** | **2.56** | **fails — decoration only** |

### App (`src/styles.css`, `:root` = dark, `body.theme-light` = light)

| Token | Dark | Light | Role |
|---|---|---|---|
| `--bg-primary` | `#1a1a1a` | `#edf2f8` | Window background |
| `--bg-secondary` | `#242424` | `#ffffff` | Rows, cards, inputs |
| `--bg-tertiary` | `#2d2d2d` | `#f4f7fc` | Hover, chips |
| `--text-primary` | `#e0e0e0` | `#1a2433` | Text |
| `--text-secondary` | `#999999` (6.11:1) | `#5f6d7f` (4.69:1) | Metadata |
| `--text-accent` | `#b8c0ff` | `#4269b6` | Accent text |
| `--accent-blue` | `#4c8dff` | — | Download button, primary actions |
| `--status-active` | `#2a9dff` | | Downloading |
| `--status-converting` | `#4c8dff` | | Converting to H.264 |
| `--status-completed` | `#1fc980` | | Done |
| `--status-failed` | `#ff5d5d` | | Error |
| `--status-cancelled` | `#d98a2b` | | Cancelled |

The app's blue (`#4c8dff`) is slightly cooler than the web's `#30a7ff` on purpose: it sits on neutral
macOS greys, not on navy. Don't «fix» one to match the other.

---

## 3. Typography

- **Web:** Geist Variable + Geist Mono Variable, self-hosted via `@fontsource-variable/*` (imported in
  `Layout.astro`; no Google Fonts request). Fallback `-apple-system, BlinkMacSystemFont, "SF Pro Text"`.
- **App:** the macOS system font (SF) at 11 px base; mono `ui-monospace, SFMono-Regular, Menlo`.
  Screenshots must look like the real app, so the app never uses Geist.

| Role (web) | Size | Line height | Weight | Tracking |
|---|---|---|---|---|
| Hero H1 | `clamp(40px, 5.2vw, 64px)` | 1.02 | 600 | −0.04em, `text-wrap: balance` |
| Section H2 | `clamp(30px, 3.6vw, 44px)` | 1.08 | 600 | −0.035em, balanced |
| Platform names | `clamp(22px, 2.2vw, 28px)` | — | 600 | −0.025em |
| Install H3 | 19 px | — | 600 | −0.015em |
| Lede (hero) | 19 px (17 px ≤ 640) | 1.55 | 400 | — |
| Body / row copy | 17 px | 1.55–1.6 | 400 | — |
| Specs, facts, FAQ answers | 15 px | 1.5–1.65 | 400 | — |
| Meta, fine print | 13 px | 1.45–1.6 | 400–500 | — |
| Keycaps, code | Geist Mono 12.5–13 px | — | 500 | — |
| Legal pages H1 / H2 / body | 28 / 16 / 14 px | — / — / 1.7 | 800 / 700 / 400 | — |

Numerals: default proportional; versions and sizes are short enough not to need tabular figures.
Max line length ~30–32 em for copy, 40 em for FAQ answers.

---

## 4. Space, shape, depth, grid

- **Container:** `.wrap` max 1120 px, side padding 24 px (16 px ≤ 640 px).
- **Vertical rhythm:** sections 120 px (88 px ≤ 900); feature rows 128 px apart (96 px ≤ 900); hero
  80 px top / 150 px bottom so the window's glow fades inside the band.
- **Grids:** hero 6fr/5fr, rows 5fr/6fr (flipped rows 6fr/5fr), FAQ 4fr/7fr, facts 4 columns
  (2 ≤ 1024, 1 ≤ 640). Gaps 56–64 px desktop, 40 px stacked.
- **Breakpoints:** 1100 (hero tightens) · 1024 (facts 2-up) · 900 (everything stacks) · 640 (phone:
  nav keeps only Download, buttons full width) · checked at **390 px** (iPhone) and 1440 px.
- **Radius:** buttons 10 px (small 8), windows 12, demo frame 16, install card 14, codebox/keycaps
  6–8, pills 999. App: 6 px controls, 12 px window edge, 999 px pills.
- **Borders:** 1 px `--line` on light; `rgba(213,235,255,.12–.16)` on navy.
- **Depth:** shadows on light are navy-tinted (`rgba(10,22,63,…)`), never neutral grey. On navy, depth
  is light, not shadow: a 1 px LED edge (`rgba(140,208,255,…)`) plus a soft blue glow
  (`rgba(48,167,255,.18–.55)`). One glow per element; glows never pulse on the web.

---

## 5. Components (web)

| Component | Spec | States |
|---|---|---|
| **Primary button** `.btn.btn-blue` | Navy text on the blue gradient, 1 px pale-blue ring, inner top highlight, blue glow. Sizes: `-sm` 13 px/8×14, `-lg` 17 px/15×28, `-dl` two-line 240 px min | Hover: −1 px lift, brightness 1.06, stronger glow. Focus: 2 px ring (`--blue` on navy). Reduced motion: no lift |
| **Ghost button** `.btn-ghost` | White text on navy, gradient hairline border | Hover: navy-800 fill, brighter border |
| **Nav** `.nav` | Sticky, 60 px, navy at 96 % with blur, LED hairline under it | Links: 78 % → 100 % opacity with an underline that grows from the centre |
| **Screenshot window** `.window` | Real app capture in a drawn macOS title bar; `-dark` / `-light`; `.on-light` for navy-tinted shadow; `.hero-window` adds the LED edge | Static |
| **Keycap** `kbd` | Geist Mono, white, 1 px `--line`, 2 px bottom border | — |
| **Spec list** `.specs` | `dt` 104 px column, hairlines between rows | — |
| **FAQ** `details` | 17 px/600 summary, chevron rotates 45° → −135° | Opens from `#hash` links |
| **Install card** `.install` | Translucent pale-blue card on navy, numbered steps, codebox with Copy | Copy → «Copied» for 2 s |
| **Status line** `.install-started` | `role="status"`, blue tint, appears after a DMG click | Rises in (300 ms) |
| **Verify disclosure** `.verify` | `details` under the download buttons: SHA-256 per DMG from the GitHub release digest; VirusTotal links behind `VT_SCANNED` | Summary underlined, ≥ 29 px target |
| **Family footer** `.family` | «More from Super»: three siblings, name + literal tagline, UTM `utm_source=superdownloads&utm_medium=footer&utm_campaign=family` | 3 columns → 1 at 640 px |
| **Known issues** `.known` (changelog) | Mist card above the releases, data in `knownIssues` (`releases.js`) | — |
| **Footer** | Brand, links (≥32 px tap height), legal fine print | Links underline on hover |
| **Badges** | Changelog «Latest» pill: green tint `rgba(31,201,128,.14)`, text `#0b7a4e` | — |
| **Launch-price badge** | **None.** Super Downloads is free; no price, seat or countdown badges | — |
| **Tables / forms** | The site has no tables or forms (activation happens in the app) | — |

**App components** (`src/styles.css`): top bar with URL field + Download (`.download-btn`, shows ↵
when a copied link is waiting) + Batch + Settings; download rows with thumbnail, status pill, thin
progress bar and ⋯ menu; slide-in Settings panel; welcome/activation card (`.onboarding-*`); toasts.

---

## 6. Motion

| What | Duration | Easing |
|---|---|---|
| Button hover (lift, glow) | 250 ms | `ease` |
| Nav underline grow | 250 ms | `ease` |
| FAQ chevron | 200 ms | `ease` |
| Install-step scroll after a download click | smooth (instant with reduced motion) | browser |
| Hero: one pass of LED light across «editors.» on load | 1.4 s, 0.5 s delay, once | `cubic-bezier(.4,0,.2,1)` |
| How it works: ⌘C → ⌘V → ✓ keys pressed in order when in view; ✓ stays lit | 420 ms each, 380 ms apart, once | `ease` |
| «Your download has started» status rises in | 300 ms | `ease` |
| Demo film | plays muted only while ≥35 % on screen; never with reduced motion | — |
| App: row enter | 280 ms | `ease-out` |
| App: progress fill | 300 ms | `cubic-bezier(.4,0,.2,1)` |
| App: converting / starting pulse | 1.2–1.5 s loop | `ease-in-out` |
| App: edge pulse (Batch on) | 1.95 s loop | `ease-in-out` |

Rules:
- Motion explains a state change (a download starting, a link waiting, a panel opening). Nothing
  moves just to decorate. Short: 150–300 ms for UI, loops only for «work in progress».
- Never move: body text, prices/version numbers, the screenshots themselves, anything that would
  cause layout shift.
- `prefers-reduced-motion: reduce` → no lifts, no smooth scroll, no autoplay, transitions off
  (web: per-rule; app: global 0.01 ms override at the end of `styles.css`).

---

## 7. Imagery and icons

- **Screenshots:** always the real app, rendered by `scripts/screenshots/capture.cjs` into
  `web/public/shots/*.webp` (1120 px + 560 px). Never mock-ups. Content shown = Blender Studio open
  movies (CC BY 4.0), credited in the footer.
- **Demo film:** `web/public/demo/demo-1080-v8.{webm,mp4}` + poster; versioned filenames (bump the
  suffix on every change — caches hold old files). Masters and the workshop live in STUDIO
  `clients/super-downloads/`.
- **App icon:** `icono.svg` (master) / `web/public/icon.svg`: download arrow, navy gradient
  `#162b73 → #0a163f`, arrow `#d5ebff → #8cd0ff → #30a7ff`.
- **Icons in UI:** 16 px stroke icons, `currentColor`, stroke 1.8, round caps (see `ONBOARDING_ICONS`
  in `src/main.js`). No icon font, no emoji in UI.
- **Share card:** `web/public/og/sd-og-home.jpg` (1200×630, < 300 KB), set once in `OG_IMAGE` in
  `Layout.astro`. A new card gets a new filename. Cards are made in STUDIO.

---

## 8. Copy patterns

- **Headlines:** a claim an editor can check, ≤ 8 words, ends with a full stop.
  «Files your NLE opens on the first try.» · «Seven platforms, one paste field.» · «Copy. Paste. Done.»
- **CTAs:** verb + object, no «now»/«today»: «Download for Mac», «Download», «Send this page to your Mac».
- **Price line:** «Free». No «limited time» without a date. Never a crossed-out price, a countdown or «only N left». Launch-price
  rules for the paid products (`08_Growth/LAUNCH-PRICE-2026-10-25.md`) do not apply here.
- **Platform claims:** YouTube, Vimeo, TikTok «no login»; Instagram, Facebook, X, LinkedIn «with your
  browser login», best-effort.
- **Legal lines (required):** «Super Downloads does not host, store, or distribute any copyrighted
  content.» + the authorized-use sentence, in the landing footer; the CC BY credit for screenshots
  next to it.
- **Shortcuts:** written as keycaps (`⌘C`, `⌘V`, `⌘B`, `↵`) with a screen-reader label.

---

## 9. Do / Don't

| Do | Don't |
|---|---|
| «Paste a link from YouTube, Vimeo, TikTok and more.» | «Download anything from anywhere!» |
| «Stops at DRM. If a video is copy-protected, the app tells you and stops.» | Imply it gets around protection |
| Navy text on the blue button | White text on `#30a7ff` (2.59:1) |
| Real screenshot in a window frame | A mock-up that differs from the shipped app |
| Navy-tinted shadow on light bands, light glow on navy | Grey drop shadows on navy |
| «Apple notarization is still in progress, so macOS blocks the first launch.» | «It's totally safe, ignore the warning» |
| One accent blue per view | A second accent colour (green is status-only, in the app and the «Latest» pill) |
| Versioned asset filenames (`*-v8.*`, `sd-og-home.jpg`) | Replacing a cached file in place |

---

## 10. Where things live — and how to change a token safely

| Thing | File |
|---|---|
| Web tokens, fonts, focus ring, base styles, OG card, analytics | `web/src/layouts/Layout.astro` |
| Landing sections, buttons, nav, windows, FAQ, install guide | `web/src/pages/index.astro` (scoped `<style>`) |
| Release notes page + known issues | `web/src/pages/changelog.astro` + `web/src/data/releases.js` |
| Legal pages | `web/src/pages/{privacy,terms,copyright}.astro` |
| Press kit (noindex) + logos | `web/src/pages/press.astro`, `web/public/press/` (`node scripts/press/render-logos.cjs`) |
| Support reply templates | `docs/support/MACROS.md` |
| Security / cache headers | `web/vercel.json` |
| App tokens, themes, components, motion | `src/styles.css` |
| App markup / UI copy | `src/index.html`, `src/main.js` |
| Screenshots pipeline | `scripts/screenshots/` |
| Brand rules | `docs/BRAND.md` |

**Changing a token:**
1. Change the value in one place (`:root` in `Layout.astro` or `src/styles.css`). Prefer the token over
   a literal; if you meet a literal, replace it with the token in the same change.
2. Re-check contrast for every pair in § 2 that uses it (AA: 4.5:1 text, 3:1 large text and UI).
3. App token? Re-render the screenshots (`node scripts/screenshots/capture.cjs`) so the site matches
   the app.
4. `cd web && npm run build`, look at `/`, `/changelog`, `/privacy` at 1440 and 390 px, then push
   (Vercel deploys `main`). App changes ship only with a release (`docs/OPS.md`).
5. Update this file in the same commit.
