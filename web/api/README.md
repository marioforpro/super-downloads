# /api/activate

Vercel Node function (root dir `web`) that appends an activation row to the Airtable "SD Users" table.
Env vars (Vercel, all required): `AIRTABLE_TOKEN`, `AIRTABLE_BASE_ID`, `AIRTABLE_SD_USERS_TABLE` (table id or name). Missing -> 503.

```
curl -X POST https://superdownloads.app/api/activate -H 'Content-Type: application/json' \
  -d '{"email":"test@example.com","appVersion":"1.3.0","arch":"aarch64","osVersion":"15.0","instance":"test","marketingOptIn":false,"source":"app"}'
```
