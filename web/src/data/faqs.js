// One source for every FAQ answer on the site: /faq (all of it, with its
// FAQPage JSON-LD) and the short list on the home page (`home: true`).
// `a` is the plain-text answer (JSON-LD, llms.txt); `html` is the visible
// answer when it needs markup. Keep answers true for the live app version;
// support macros (docs/support/MACROS.md) say the same thing.

export const SUPPORT = 'support@superdownloads.app';
export const XATTR = 'xattr -dr com.apple.quarantine "/Applications/Super Downloads.app"';
const REPO = 'https://github.com/marioforpro/super-downloads';
const MAIL = `<a href="mailto:${SUPPORT}">${SUPPORT}</a>`;

export const faqGroups = [
  {
    title: 'The basics',
    items: [
      {
        id: 'what',
        home: true,
        q: 'What is Super Downloads?',
        a: 'A native Mac app that saves online video as edit-ready files. YouTube, Vimeo and TikTok work without a login; Instagram, Facebook, X and LinkedIn work through your browser login, on a best-effort basis. Every file is an H.264 MP4 that opens in Premiere Pro, DaVinci Resolve and Final Cut Pro.',
      },
      {
        id: 'free',
        home: true,
        q: 'Is it free?',
        a: 'Yes. Free, with unlimited downloads and every feature. You activate it once with your email inside the app. No card required. If that ever changes, everyone who activated hears from us by email first.',
      },
      {
        id: 'different',
        home: true,
        q: 'How is it different from other downloaders?',
        a: 'Most downloaders hand you whatever the site serves, often VP9 or AV1 in a WebM or MKV file that your editing app won’t import. Super Downloads always gives you H.264 in an MP4, picks up the link you just copied, and keeps its download engine up to date by itself. No Terminal, nothing to configure.',
      },
      {
        id: 'macs',
        q: 'Which Macs does it run on? Is there a Windows or Linux version?',
        a: 'Apple Silicon Macs with macOS 12 Monterey or later, and Intel Macs with macOS 10.15 Catalina or later. There is no Windows or Linux version yet.',
      },
    ],
  },
  {
    title: 'Installing and updating',
    items: [
      {
        id: 'faq-macos',
        home: true,
        q: 'macOS won’t open the app. Is it safe?',
        a: 'Yes. The app isn’t notarized by Apple, so macOS can’t verify it and blocks the first launch. You allow it once. On macOS 15 or later: open the app, click Done, then go to System Settings › Privacy & Security, scroll down to Security and click Open Anyway (it shows for about an hour after you try to open the app), confirm with your password or Touch ID and click Open Anyway again. On macOS 14 or earlier: right-click the app, choose Open, then click Open. The source code is public on GitHub, and you can check your download against the SHA-256 published on the download section of the home page.',
        html: `Yes. The app isn’t notarized by Apple, so macOS can’t verify it and blocks the first launch. You allow it once. On macOS 15 or later: open the app, click <strong>Done</strong>, then go to System Settings › Privacy &amp; Security, scroll down to Security and click <strong>Open Anyway</strong> (it shows for about an hour after you try to open the app), confirm with your password or Touch ID and click <strong>Open Anyway</strong> again. On macOS 14 or earlier: right-click the app, choose Open, then click Open. The <a href="${REPO}">source code</a> is public, and you can check your download against the SHA-256 under <a href="/#download">Verify the download</a>.`,
      },
      {
        id: 'damaged',
        q: 'macOS says the app is “damaged”. What do I do?',
        a: `The file isn’t broken: that message comes from the quarantine flag macOS puts on downloads. Paste this into Terminal (Applications › Utilities › Terminal) and press Return: ${XATTR}. Then open the app normally.`,
        html: `The file isn’t broken: that message comes from the quarantine flag macOS puts on downloads. Paste this into Terminal (Applications › Utilities › Terminal) and press Return:<br /><code>${XATTR.replace(/"/g, '&quot;')}</code><br />Then open the app normally.`,
      },
      {
        id: 'update',
        q: 'How do I update the app?',
        a: 'When a new version is out, a banner appears at the top of the app; click Update and it installs itself. The download engine, the part that talks to video sites, checks for updates every week and installs them on its own. What changed in each version is on the release notes page.',
        html: 'When a new version is out, a banner appears at the top of the app; click <strong>Update</strong> and it installs itself. The download engine, the part that talks to video sites, checks for updates every week and installs them on its own. What changed in each version is in the <a href="/changelog">release notes</a>.',
      },
      {
        id: 'intel-slow',
        q: 'The first download on my Intel Mac is slow to start.',
        a: 'On Intel Macs the very first download after installing can take about 10 seconds to start while macOS prepares the download engine. Later downloads start right away.',
      },
    ],
  },
  {
    title: 'Downloading',
    items: [
      {
        id: 'what-can',
        home: true,
        q: 'What can I download?',
        a: 'Video you own, video you have licensed, video shared under Creative Commons, and anything else you are authorized to use: your own uploads, client footage, approved references. The app does not get around DRM: if a video is copy-protected, it tells you and stops.',
      },
      {
        id: 'quality',
        home: true,
        q: 'What quality can I download?',
        a: 'Best (up to 4K), 1080p or 720p, plus audio only as MP3. 1080p and 720p are usually fastest, because most sites already offer them as H.264. When a source only has 4K as VP9 or AV1, Best converts it to H.264 with your Mac’s hardware encoder.',
      },
      {
        id: 'login',
        q: 'Why do some platforms need my browser login?',
        a: 'Instagram, Facebook, X and LinkedIn usually serve video only to signed-in users. Super Downloads uses the session already in your default browser (Safari, Chrome, Firefox, Brave, Edge or Vivaldi). It never asks for, sees or stores your password.',
      },
      {
        id: 'site-broken',
        home: true,
        q: 'A site stopped working. What now?',
        a: `Sites change how they deliver video all the time. The app checks for a new download engine every week and installs it on its own, which usually fixes it. To check right away, open Settings and click Update next to Engine. Still failing? Write to ${SUPPORT} with the link.`,
        html: `Sites change how they deliver video all the time. The app checks for a new download engine every week and installs it on its own, which usually fixes it. To check right away, open Settings and click <strong>Update</strong> next to Engine. Still failing? Write to ${MAIL} with the link.`,
      },
      {
        id: 'import',
        q: 'My editing app won’t import a file.',
        a: `Every download should be an H.264 MP4 with AAC audio, which Premiere Pro, Resolve and Final Cut read natively. Try the same link at 1080p, which usually comes straight as H.264. If it still fails, write to ${SUPPORT} with the app version (top of Settings), the quality you chose, the link and the exact message your editing app shows.`,
        html: `Every download should be an H.264 MP4 with AAC audio, which Premiere Pro, Resolve and Final Cut read natively. Try the same link at 1080p, which usually comes straight as H.264. If it still fails, write to ${MAIL} with the app version (top of Settings), the quality you chose, the link and the exact message your editing app shows.`,
      },
      {
        id: 'where',
        q: 'Where do my downloads go?',
        a: 'To your Downloads folder, unless you choose another one in Settings › Save folder. Hover a finished download and use its menu to show it in Finder.',
      },
      {
        id: 'servers',
        q: 'Do my downloads go through your servers?',
        a: 'No. Everything runs on your Mac. Video goes straight from the platform to your save folder.',
      },
    ],
  },
  {
    title: 'Your email and privacy',
    items: [
      {
        id: 'email',
        q: 'What do you do with my email?',
        a: 'It activates the app and lets us tell you about important changes, such as security fixes or any change to how the app is offered. Emails about updates and our other apps are optional: you get them only if you tick that box, and every one has a one-click unsubscribe.',
        html: 'It activates the app and lets us tell you about important changes, such as security fixes or any change to how the app is offered. Emails about updates and our other apps are optional: you get them only if you tick that box, and every one has a one-click unsubscribe. Details in the <a href="/privacy">Privacy Policy</a>.',
      },
      {
        id: 'activation',
        q: 'I entered my email but the app still asks me to activate.',
        a: `Activation happens inside the app: once you enter your email, the app unlocks on that Mac right away, even if your connection is down at that moment (it confirms with us on the next launch). If it still asks, write to ${SUPPORT} with your app version and macOS version.`,
        html: `Activation happens inside the app: once you enter your email, the app unlocks on that Mac right away, even if your connection is down at that moment (it confirms with us on the next launch). If it still asks, write to ${MAIL} with your app version and macOS version.`,
      },
      {
        id: 'usage-stats',
        q: 'Does the app track what I download?',
        a: 'No. The app sends one anonymous ping a day so we can count how many people use it: the app version, your Mac type and a random number renewed every year. No email, no IP address, nothing about your downloads. Turn it off in Settings › Share anonymous usage stats.',
        html: 'No. The app sends one anonymous ping a day so we can count how many people use it: the app version, your Mac type and a random number renewed every year. No email, no IP address, nothing about your downloads. Turn it off in Settings › Share anonymous usage stats. Details in the <a href="/privacy#usage-stats">Privacy Policy</a>.',
      },
      {
        id: 'delete',
        q: 'How do I delete my data or stop the emails?',
        a: `Every email has a one-click unsubscribe link. To have your email and activation record deleted, write to ${SUPPORT} from that address; we delete it and confirm. Your download history and settings never leave your Mac: clear the list in the app or delete the app to remove them.`,
        html: `Every email has a one-click unsubscribe link. To have your email and activation record deleted, write to ${MAIL} from that address; we delete it and confirm. Your download history and settings never leave your Mac: clear the list in the app or delete the app to remove them.`,
      },
    ],
  },
  {
    title: 'Getting help',
    items: [
      {
        id: 'contact',
        q: 'How do I report a problem or suggest a feature?',
        a: `In the app, open Settings › Help › Report a problem: it starts an email with your app and Mac details filled in. Or write to ${SUPPORT} directly. Include the link if a download failed. Feature ideas are welcome at the same address.`,
        html: `In the app, open Settings › Help › <strong>Report a problem</strong>: it starts an email with your app and Mac details filled in. Or write to ${MAIL} directly. Include the link if a download failed. Feature ideas are welcome at the same address. Known issues and their fixes are listed in the <a href="/changelog#known-issues">release notes</a>.`,
      },
    ],
  },
];

export const faqs = faqGroups.flatMap((g) => g.items);

export const faqJsonLd = {
  '@context': 'https://schema.org',
  '@type': 'FAQPage',
  mainEntity: faqs.map((f) => ({ '@type': 'Question', name: f.q, acceptedAnswer: { '@type': 'Answer', text: f.a } })),
};
