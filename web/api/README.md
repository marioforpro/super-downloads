# /api/activate

Vercel Node function (root dir `web`) that appends an activation row to the Airtable "SD Users" table.
Env vars (Vercel, all required): `AIRTABLE_TOKEN`, `AIRTABLE_BASE_ID`, `AIRTABLE_SD_USERS_TABLE` (table id or name). Missing -> 503.

```
curl -X POST https://superdownloads.app/api/activate -H 'Content-Type: application/json' \
  -d '{"email":"test@example.com","appVersion":"1.3.0","arch":"aarch64","osVersion":"15.0","instance":"test","marketingOptIn":false,"source":"app"}'
```

# /api/subscribe

News sign-up from the website (no app install): forms on the home hero (non-Mac visitors only), the download section
and `/faq` (`web/src/components/SubscribeForm.astro`, switch `SUBSCRIBE_ON`). Appends a row to the same "SD Users"
table with `Source = web`, `Instance = web:<place>`, no `Activated At`, and the consent wording + time. Stores nothing
unless the box was ticked (`consent: true` + `consentText`); hidden `website` field = honeypot. Same env vars as
`/api/activate`. Offline test: `node scripts/test-subscribe.mjs` (API + form in WebKit over `web/dist`).
