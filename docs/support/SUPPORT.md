# Support inventory — Super Downloads

> Where a user can ask for help today, where it lands, what they ask and how we answer. Updated 2026-10-27.
> The family-wide routine (inbox, labels, `/support` drafts, weekly review) lives in LifeOS
> `07_Operations/support/SYSTEM.md`. Reply templates: `MACROS.md` (this folder). Public answers: `/faq`
> (source `web/src/data/faqs.js`). This repo is public: no personal addresses, names or real cases here.

## 1 · Channels a user can use today

| Channel | Where the user finds it | Where it lands | Notes |
|---|---|---|---|
| **support@superdownloads.app** | Footer «Contact», `/faq`, `/changelog`, `/privacy`, `/press`, release notes, the app | The support mailbox, then the family support inbox (see SYSTEM.md) | The only address published. Forwarding to the family inbox is pending confirmation: until then, check the mailbox itself |
| **App › Settings › Help › Report a problem** | In the app | Opens a prefilled email to support@ (app version, macOS, chip, engine version) and copies the same text to the clipboard | Webmail users without a mail app: the toast tells them to paste it |
| **App › failed download › ⋯ › Report** | On a failed row | Same email, plus the link, the error and the engine output | Best bug reports we get |
| **GitHub issues** (public repo) | Repo page | GitHub notifications of the repo owner | Blank issues are off; the chooser sends help requests to support@ and `/changelog#known-issues`; the bug form stays for reproducible bugs |
| **`/faq`** (`/help` and `/support` redirect there) | Footer «Help», home FAQ «All questions and help» | Self-serve | 20 answers in 5 groups, FAQPage JSON-LD |
| **`/changelog#known-issues`** | `/faq`, GitHub chooser, macros | Self-serve | Remove an entry in the commit that fixes it |
| One-click unsubscribe | Every marketing email | `/api/unsubscribe` → Airtable `Marketing Withdrawn At` | No reply needed |

Not a channel: hello@superdownloads.app (does not exist; never publish it), social profiles (none yet), app stores
(not distributed there), payments (no provider, the app is free).

**Reply time we promise:** 2 business days (same promise on every Super app). No auto-acknowledgement for now.

## 2 · The 15 questions we expect most

Answers are the `/faq` entries (one source, kept true for the live version) and the macros. «Data» is what we need
before we can resolve it; «Self-serve» says whether the user can fix it alone.

| # | Question | Answer (`/faq#…`) | Macro | Data we need | Self-serve |
|---|---|---|---|---|---|
| 1 | macOS won't open the app / «can't verify» | `faq-macos` | 1 | macOS version, chip | Yes (Open Anyway) |
| 2 | «The app is damaged» | `damaged` | 2 | — | Yes (Terminal line) |
| 3 | A site stopped working | `site-broken` | 3 | the link, app + engine version | Usually (engine update) |
| 4 | Instagram / Facebook / X / LinkedIn fails | `login` | 3 (login variant) | the link, default browser, signed in? | Usually |
| 5 | Premiere / Resolve won't import the file | `import` | 4 | app version, quality, link, the NLE's message | Often (try 1080p) |
| 6 | Activation didn't work / still asks | `activation` | 5 | app + macOS version | Rarely |
| 7 | Is it really free? Will you charge? | `free` | 9 | — | Yes |
| 8 | Delete my data / stop emails | `delete` | 6 | the email address (reply from it) | Emails: yes (unsubscribe link). Deletion: we do it in Airtable `SD · Users`, within 30 days |
| 9 | Is it legal? What can I download? | `what-can` | 7 | — | Yes |
| 10 | Windows / Linux? | `macs` | 8 | — | Yes |
| 11 | First download slow on Intel | `intel-slow` | 10 | — | Yes |
| 12 | How do I update? | `update` | — (link `/faq#update`) | — | Yes |
| 13 | Where did my download go? | `where` | — (link `/faq#where`) | — | Yes |
| 14 | Does the app track me / what is the daily ping? | `usage-stats` | — (link `/privacy#usage-stats`) | — | Yes (switch in Settings) |
| 15 | Can you add {feature / site}? | `contact` | 11 | — | No (goes to the ideas list) |

Escalate to the owner (Claude drafts, does not resolve): data deletion (macro 6, the Airtable delete is manual),
press and partnerships, anything legal (copyright complaints go to `/copyright`), and any report that a release
broke downloads for everyone (then run the live E2E: `cd src-tauri && cargo test e2e_downloads_live -- --ignored --nocapture`).

## 3 · Keeping it true

- A new known issue → `web/src/data/releases.js` `knownIssues` + macro 3 wording if it changes the advice.
- A change in how the app is offered (price, limits) → `/faq#free`, macro 9, `/press` facts and the welcome card,
  in the same release.
- New question seen twice in a month → add it to `faqs.js` and to the table above.
