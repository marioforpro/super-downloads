# Support macros — Super Downloads

> Reply templates for support@superdownloads.app, used by `/comms-draft`. Brand voice (`DESIGN.md` § 1):
> plain, short, no exclamation marks, signed by the team, never by a person. Fill the `{…}` slots; delete
> lines that don't apply. This repo is public: no names, addresses or real cases in this file.
>
> Facts to keep current: version on `/changelog`, requirements (Apple Silicon macOS 12+, Intel macOS 10.15+),
> known issues on `/changelog#known-issues`. The price line follows the live site (currently «free»; change it here when the site changes).

---

## 1. macOS won't open the app («can't verify», «Apple could not check it»)

Subject: Opening Super Downloads for the first time

Hi,

macOS shows that message because Apple notarization is still in progress. The app is safe to open; you only need to allow it once:

1. Open Super Downloads and click **Done** on the warning.
2. Go to System Settings › Privacy & Security, scroll down and click **Open Anyway**.

On macOS 14 or earlier: right-click the app in Applications, choose Open, then Open again.

From then on it opens like any other app. If it still won't start, reply with your macOS version and whether your Mac is Apple Silicon or Intel (Apple menu › About This Mac).

The Super Downloads team

---

## 2. «The app is damaged and can't be opened»

Subject: Fixing the «damaged» message

Hi,

That message comes from the download quarantine flag, not from a broken file. Paste this into Terminal (Applications › Utilities › Terminal) and press Return:

`xattr -dr com.apple.quarantine "/Applications/Super Downloads.app"`

Then open the app normally. If you'd like to check the file first, the SHA-256 fingerprint of each download is on superdownloads.app under «Verify the download».

The Super Downloads team

---

## 3. A site stopped working («it worked yesterday on {site}»)

Subject: {Site} downloads

Hi,

Thanks for the link. Video sites change how they deliver video often, and the app's download engine updates every week to keep up. To check for a new engine right away: Settings › **Update** next to Engine, then try the link again.

{If fixed by a newer engine:} A new engine that fixes this is out; the Update button installs it.
{If known and pending:} We can reproduce it and are working on it. It's listed under Known issues on superdownloads.app/changelog, and we'll write back when it's fixed.
{If login site:} Instagram, Facebook, X and LinkedIn use the session in your browser: make sure you're signed in there in your default browser, then retry.

The Super Downloads team

---

## 4. The file won't import / wrong codec

Subject: Importing your download

Hi,

Every download should be an H.264 MP4 with AAC audio. Could you reply with:

- the app version (top of Settings),
- the quality you chose (Best, 1080p, 720p or audio only),
- the link, if it's public,
- the exact message your editing app shows.

In the meantime, 1080p usually comes straight as H.264 and imports everywhere.

The Super Downloads team

---

## 5. Activation didn't work / didn't get an email

Subject: Activating Super Downloads

Hi,

Activation happens inside the app: once you enter your email, the app is unlocked on that Mac right away, even if the connection is down at that moment (it confirms with us on the next launch). {If welcome email is on:} We also send one welcome email; if it's not in your inbox, check Spam or Promotions.

If the app still asks you to activate, reply with your app version and macOS version and we'll look into it.

The Super Downloads team

---

## 6. Delete my data / stop emails

Subject: Your data

Hi,

Done: we've deleted your email and activation details from our records{, and you won't receive any more emails from us}. The app keeps working on your Mac. Your download history and settings never left your computer; to remove them too, clear the list in the app or delete the app.

The Super Downloads team

> Internal: delete the row(s) for that email in Airtable `SD · Users` before sending. Reply within 30 days (GDPR).

---

## 7. Is it legal? What can I download?

Subject: What Super Downloads is for

Hi,

Super Downloads is for video you own, have licensed, that's shared under Creative Commons, or that you're otherwise authorized to use: your own uploads, client footage, approved references. It doesn't host or distribute anything, and it stops at copy-protected (DRM) video. Whether a specific download is allowed depends on the video's licence and the site's terms.

The Super Downloads team

---

## 8. Windows or Linux?

Subject: Super Downloads on {Windows / Linux}

Hi,

Not yet: Super Downloads is Mac only for now (Apple Silicon with macOS 12+, Intel with macOS 10.15+). We've noted your interest{; reply «yes» if you'd like one email when that changes}.

The Super Downloads team

---

## 9. Is it really free? Will you charge later?

Subject: Price

Hi,

Yes: it's free, with unlimited downloads and every feature, and no card. If that ever changes, everyone who activated gets an email before anything happens; nothing changes for you without notice.

The Super Downloads team

---

## 10. First download is slow on an Intel Mac

Subject: Slow first download

Hi,

On Intel Macs the very first download after installing can take about 10 seconds to start while macOS prepares the download engine. Later downloads start right away. If every download is slow, reply with the link and your connection type and we'll take a look.

The Super Downloads team

---

## 11. Feature request

Subject: Your idea

Hi,

Thanks, that's useful. We've added it to our list{; it's close to something already planned}. We don't promise dates, but requests from editors are what decide what comes next.

The Super Downloads team
