# Super Downloads — Landing (`web/`)

Astro v6 marketing site for Super Downloads. Deployed on Vercel (auto-deploy on push to `main`).

- **Live:** https://superdownloads.app · https://www.superdownloads.app (both resolve through Vercel)
- **Vercel project:** `superdownloads.vercel.app`
- **Analytics:** PostHog (integrated in the landing)

## Key locations

| What | Where |
|------|-------|
| Pages | `src/pages/` (`index.astro` = home) |
| "Get Pro" checkout link | `src/pages/index.astro` pricing section — LemonSqueezy product UUID `21db1cfb-37f8-4371-8085-b5e30f89645f` (must match `../src/main.js:99`) |
| Download buttons | point to `github.com/marioforpro/super-downloads/releases/latest/download/Super-Downloads_<arch>.dmg` |

## Dev

```sh
npm install
npm run dev      # local dev server
npm run build    # production build → dist/
```

> Project docs live one level up in `../docs/`. Brand rules: `../docs/BRAND.md`. Marketing: `../docs/MARKETING.md`.
