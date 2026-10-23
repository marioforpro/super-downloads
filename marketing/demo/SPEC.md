# SPEC — «Links become clips» (storyline A)

GOAL:    an editor ends knowing the whole move — copy a link, switch to Super Downloads, press Enter, the video is in the timeline — and wants it (free, for Mac).
OUTPUT:  HyperFrames, 1920 × 1080, 30 fps (master; landing hero + X). Recomposed 1080 × 1920 for Instagram/TikTok after the master passes review.
TIME:    24.0 s × 30 = 720 frames. Grid 120 BPM → 15 frames a beat, 60 frames a bar (2.0 s). Picture changes land on beats.

LOOK:    four worlds, one palette (the app icon's own colours + the landing):
         - browser world: ground #edf2f8, white browser window, ink #0a163f;
         - app world: the icon gradient #162b73 → #0a163f (radial, light from upper left), the app window in its real dark theme;
         - timeline world: #0d1b4f, the landing hero's timeline (ruler, V1/A1, clips with 1 px #8cd0ff borders) + a program monitor;
         - end card: the icon gradient again.
         Display: Geist 700, titles 92 px (8.5 % of height), tracking −2 %, white (#ffffff) on blue, #0a163f on light.
         Accent #30a7ff → #8cd0ff (the icon arrow gradient): one word per title, the playhead, the end-card arrow. Nothing else blue-bright.
         Medium: product UI as objects — windows with a 1 px rim (rgba 213,235,255,.14), contact shadow 0 40px 120px −30px rgba(48,167,255,.35) + 0 30px 60px −30px rgba(0,0,0,.8);
         soft blue rim light behind the app window (Mario's «depth, glow, grain» — overrides the vendor no-glow default, MOTION-CRAFT §7); 3 % animated grain over every world; vignette 8 %.

OBJECTS: the real link (address bar, then a capsule with a link glyph); physical keycaps (110 px, radius 22 px, top #f7f9fc, 8 px depth #c5cfdd, press = 6 px down + shadow 8 → 2 px);
         a macOS arrow cursor 54 px (5 % of height); the app window = the real app DOM (assets/app, extract-ui.cjs) at 2.4× (11 px → 26 px text);
         thumbnails = Blender open movies (CC BY 4.0); the payoff footage = «Spring» 1:17–1:26 (Blender Studio, CC BY 4.0), credited on the end card.

MOTION:  arrivals expo.out 0.6 s (18 f); moves power3.inOut 0.5 s; exits power3.in, 40–50 % of the arrival; camera = one continuous move per world (power1.inOut segments that overlap, never stopping between targets);
         keycap press 4 f down (power2.in), 8 f up (expo.out); the capsule arcs (MotionPath-free: x and y on different eases) with 6–8 px blur at its fastest frame;
         titles: per word blur 6 → 0 + opacity in 4 f, 2 f apart, then dead still; exit = blur-out 5 f. Progress bars and counters linear (they are the point).

SHOTS:
  0–60    (bar 1)  black → the browser rises out of the dark (iris of light, 12 f); cursor enters bottom-right, clicks the address bar (f 36), selection sweeps (8 f); ⌘ down f 52 · C down f 60 (beat) — «Copy.» lands f 60. SFX: key-press ×2.
  60–120  (bar 2)  the selected URL lifts off as a capsule (f 60–78, scale 1 → 1.25, shadow grows); ⌘ Tab press f 90 — «Switch.»; whip: browser leaves left with 20 px blur f 100–112, colour flood to the app gradient f 105–117. SFX: key-press, whoosh-short.
  105–240 (bars 3–4) the app window rises (f 105–135, y +260 → 0, expo.out) while the capsule arcs into the URL field (lands f 132, the field shows the link, Download glows ready); ↵ press f 150 — «Enter.»; the real row enters (rowEnter, 8 f) and downloads 0 → 100 % f 153–210 with speed/ETA; Converting to H.264 f 210–240. Camera: slow push 1.0 → 1.06 toward the row across the shot. SFX: key-press, click-soft on ↵.
  240–300 (bar 5)  Completed ✓ f 240 (pop); ⌘ B press f 270 — Batch on (real button + ribbon) — «Batch mode.» SFX: pop, key-press.
  300–420 (bars 6–7) three links «rain» in: a mini address bar flashes in at f 300 / 315 / 330 (beat-locked inserts, 12 f each), each capsule drops into the list; three real rows download at their own speeds and complete f 378 / 390 / 402 — «Many at once.» SFX: click-soft per drop, pop per completion (two takes alternating).
  420–480 (bar 8)  the cursor grabs Spring's thumbnail (f 420), drags it down; the camera pans down with it and the timeline world rises (object becomes the world, f 432–462); the clip snaps onto V1 at f 462 (spring damping 0.75), the other three follow 6 f apart. SFX: whoosh-cinematic (cut), click-soft on snap.
  480–570 (bars 9–early 10) payoff: the program monitor plays «Spring» full width above the tracks, playhead runs; «Ready for your timeline.» f 486; small «H.264 · MP4» under it. SFX: riser f 440 → impact f 480.
  570–600          «From YouTube.» — the platform word rolls YouTube → TikTok → X → Vimeo → Instagram → Facebook → LinkedIn (3 f each, settles on LinkedIn).
  600–720 (bars 11–12) end card: the icon's arrow falls into the rounded square (f 600–618, impact), «Super Downloads» wipes in (f 618–636), «Free for Mac · superdownloads.app» (f 640), «Made by Mario Monteiro» (f 652); hold to f 720. Credit line 18 px: «Film: Spring © Blender Foundation · CC BY 4.0».

BANNED:  dark void with small floating UI; neon/lens flares/particles; purple gradients; three cards in a row; a phone frame; titles that drift after landing; a docked corner logo;
         invented UI (every app pixel is the real DOM); YouTube/Adobe chrome (the browser is a generic window; the timeline is our landing's design); white flashes between worlds; crossfades between worlds.
QA:      f 0 (black, nothing else), f 12 (browser risen), f 60 (keycaps + «Copy.»), f 110 (whip mid-blur), f 132 (link in field), f 180 (row ~45 %), f 225 (converting), f 245 (Completed), f 280 (Batch on + ribbon), f 350 (three rows downloading), f 450 (world change mid), f 520 (monitor playing + title), f 585 (platform roll), f 640 (end card full), f 719 (last frame held).
