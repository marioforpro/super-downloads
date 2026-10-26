// User-facing release notes for /changelog, newest first. Plain language for
// editors; the technical detail lives in CHANGELOG.md. Add an entry with every
// release (docs/OPS.md § Release pipeline).
// Known issues shown at the top of /changelog (#known-issues). Plain language,
// each with what to do. Remove an entry in the same commit that fixes it.
export const knownIssues = [
  'The app isn’t notarized by Apple, so the first time you open it macOS says it can’t verify it. Click Done, then System Settings › Privacy & Security › Open Anyway, and confirm. You only do this once.',
  'On Intel Macs the very first download after installing can take about 10 seconds to start while macOS prepares the download engine. Later downloads start right away.',
  'Instagram, Facebook, X and LinkedIn change how they serve video often and work on a best-effort basis. When one stops working, the weekly engine update usually fixes it; Settings › Update next to Engine checks right away.',
  'Copy-protected (DRM) video can’t be downloaded. The app tells you and stops.',
];

export const releases = [
  {
    version: '1.5.6',
    date: '2026-10-26',
    notes: [
      'The box for emails about updates and other Super apps now starts unticked. Tick it only if you want them; every email has a one-click unsubscribe.',
      'After your tenth download, once, the app offers a link to send to an editor friend. Say no and it never asks again.',
      'Updated the app framework and its security libraries.',
    ],
  },
  {
    version: '1.5.5',
    date: '2026-10-25',
    notes: [
      'Vimeo videos hidden from vimeo.com but shared by link now download.',
      'The backup route for Instagram works again, for the days the main one fails.',
      'Copy a link, paste it, done — the app now explains it the same way as the website.',
      'Settings › Help › Watch demo shows the app in 24 seconds.',
      'New: one anonymous ping a day counts how many people use the app. It carries the app version, your Mac type and a random number renewed every year; no email, no IP, nothing about your downloads. Turn it off anytime in Settings › Share anonymous usage stats.',
    ],
  },
  {
    version: '1.5.4',
    date: '2026-10-23',
    notes: [
      'A download that loses its connection now stops after 90 seconds with a clear message instead of freezing. Press Retry and it continues where it left off.',
      'The ⋯ menu stays open while other downloads update.',
      'Settings works better with the keyboard and VoiceOver.',
      'New downloads slide into the list.',
      'The app now states what it needs: macOS 12 or later on Apple Silicon, macOS 10.15 or later on Intel.',
    ],
  },
  {
    version: '1.5.3',
    date: '2026-10-23',
    notes: [
      'Report a problem in one click — from the ⋯ menu of a failed download, or Settings › Help. The email arrives filled in with everything needed to fix it.',
      'A new welcome screen.',
    ],
  },
  {
    version: '1.5.2',
    date: '2026-10-23',
    notes: [
      'Fresh download engine.',
      'The engine’s weekly self-update now checks the file’s fingerprint before installing it.',
      'Dropbox links are no longer mistaken for X links.',
    ],
  },
  {
    version: '1.5.1',
    date: '2026-10-20',
    notes: ['Vimeo videos whose owner only allows embedding on certain websites now download.'],
  },
  {
    version: '1.5.0',
    date: '2026-10-04',
    notes: [
      'Copy & come back: copy a video link, switch to the app and press Enter.',
      'Paste a link and it starts downloading. Paste several and they all start.',
      'Batch mode (⌘B): every video link you copy downloads on its own, even with the app in the background.',
    ],
  },
  {
    version: '1.4.0',
    date: '2026-10-03',
    notes: [
      'Downloads start much faster — the engine used to take about 7 seconds to wake up.',
      'No more needless re-encoding at 4K when the source is already H.264.',
      'Redesigned Settings.',
      'Real conversion progress, a progress bar that never goes backwards, and errors in plain language.',
      'More Vimeo videos download, including ones whose owner restricts embedding.',
    ],
  },
  {
    version: '1.3.0',
    date: '2026-10-03',
    notes: ['Super Downloads is free, with no download limit. You activate it with your email.'],
  },
  {
    version: '1.2.0',
    date: '2026-07-16',
    notes: [
      'The download engine now updates itself every week, so sites that change keep working without a new app version.',
      'Facebook, Instagram and other sign-in sites download more reliably, using your browser’s login when needed.',
      'Settings shows the engine version, with an Update button.',
    ],
  },
  {
    version: '1.1.1',
    date: '2026-05-31',
    notes: [
      'One-click updates: a banner appears when a new version is out.',
      'YouTube downloads in full quality again (they were stuck at 360p).',
    ],
  },
  {
    version: '1.1.0',
    date: '2026-05-06',
    notes: ['First public release.'],
  },
];
