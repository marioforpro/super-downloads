// /llms.txt (llmstxt.org): a plain summary of the site for AI assistants and
// answer engines. Built from the same data as the pages, so it never drifts.
import { faqs, SUPPORT } from '../data/faqs.js';
import { releases } from '../data/releases.js';

const SITE = 'https://superdownloads.app';

export function GET() {
  const latest = releases[0];
  const body = `# Super Downloads

> Super Downloads is a free Mac app for video editors. Paste a link from YouTube, Vimeo, TikTok, Instagram, Facebook, X or LinkedIn and get an H.264 MP4 with AAC audio that opens in Premiere Pro, DaVinci Resolve and Final Cut Pro without transcoding. Everything runs on the Mac.

- Price: free, unlimited downloads, every feature; activated once with an email inside the app. No card.
- Platforms: macOS 12 Monterey or later on Apple Silicon; macOS 10.15 Catalina or later on Intel. No Windows or Linux version.
- Output: H.264 + AAC in MP4, Best (up to 4K), 1080p or 720p; audio only as MP3.
- Sites: YouTube, Vimeo and TikTok without a login; Instagram, Facebook, X and LinkedIn through the browser session, on a best-effort basis. Stops at DRM.
- Latest version: ${latest.version} (${latest.date}). The app updates itself; its download engine (yt-dlp) updates weekly.
- For video the user owns, has licensed, that is shared under Creative Commons, or is otherwise authorized to use.
- Not notarized by Apple yet: the first launch needs System Settings › Privacy & Security › Open Anyway.
- Support: ${SUPPORT}

## Pages

- [Home and download](${SITE}/): what the app does, screenshots, the 24-second demo, download for Apple Silicon or Intel, SHA-256 of each file
- [Help and FAQ](${SITE}/faq): installing, updates, quality, sites that need a login, imports, privacy, support
- [Release notes](${SITE}/changelog): every version in plain language, plus known issues
- [Press kit](${SITE}/press): facts, descriptions, logos and screenshots
- [Privacy Policy](${SITE}/privacy): what is collected (activation email, one anonymous daily usage ping) and how to opt out
- [Source code](https://github.com/marioforpro/super-downloads)

## FAQ

${faqs.map((f) => `### ${f.q}\n\n${f.a}`).join('\n\n')}

## Optional

- [Terms of Use](${SITE}/terms)
- [Copyright and takedown](${SITE}/copyright)
`;
  return new Response(body, { headers: { 'Content-Type': 'text/plain; charset=utf-8' } });
}
