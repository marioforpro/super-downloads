// Screenshot harness: fake Tauri API + staged downloads (Blender open movies, CC-BY 4.0).
(function () {
  const q = new URLSearchParams(location.search);
  const theme = q.get("theme") || "dark";
  const scene = q.get("scene") || "list";

  localStorage.clear();
  if (scene !== "onboarding") localStorage.setItem("hasSeenOnboarding", "true");
  if (scene !== "onboarding" && scene !== "activation") {
    localStorage.setItem("activationEmail", "you@yourstudio.com");
    localStorage.setItem("activationSynced", "1");
  }
  localStorage.setItem("appSettings", JSON.stringify({
    theme, keepHistory: scene !== "empty", videoQuality: "best",
    downloadLocation: "/Users/editor/Movies/Super Downloads"
  }));

  const t = (id) => `thumbs/${id}.jpg`;
  const base = { conversionProgress: 0, speed: "", eta: "", completed: false, error: null, filePath: "" };
  const done = (o) => ({ ...base, progress: 100, status: "completed", completed: true, filePath: "/Users/editor/Movies/x.mp4", ...o });

  const lists = {
    list: [
      { ...base, id: "d1", url: "https://youtu.be/UXqq0ZvbOnk", title: "CHARGE — Blender Open Movie", thumbnail: t("UXqq0ZvbOnk"),
        progress: 64, speed: "18.42MiB/s", eta: "ETA 0:09", status: "downloading", duration: "4:23", size: "412.6 MB", format: "MP4", resolution: "2160p", fps: "24fps" },
      { ...base, id: "d2", url: "https://vimeo.com/1", title: "Sprite Fright — Blender Open Movie", thumbnail: t("_cMxraX_5RE"),
        progress: 100, conversionProgress: 41, eta: "ETA 0:38", status: "converting", duration: "10:30", size: "688.1 MB", format: "MP4", resolution: "1080p", fps: "24fps" },
      done({ id: "d3", url: "https://youtu.be/WhWc3b3KhnY", title: "Spring — Blender Open Movie", thumbnail: t("WhWc3b3KhnY"),
        duration: "7:44", size: "301.4 MB", format: "MP4", resolution: "1080p", fps: "24fps" }),
      done({ id: "d4", url: "https://youtu.be/u9lj-c29dxI", title: "WING IT! — Blender Open Movie", thumbnail: t("u9lj-c29dxI"),
        duration: "3:58", size: "162.9 MB", format: "MP4", resolution: "1080p", fps: "24fps" }),
    ],
    batch: [
      { ...base, id: "b1", url: "https://youtu.be/mN0zPOpADL4", title: "Agent 327: Operation Barbershop", thumbnail: t("mN0zPOpADL4"),
        progress: 0, status: "starting" },
      { ...base, id: "b2", url: "https://youtu.be/pKmSdY56VtY", title: "HERO — Blender Grease Pencil Showcase", thumbnail: t("pKmSdY56VtY"),
        progress: 37, speed: "11.07MiB/s", eta: "ETA 0:14", status: "downloading", duration: "3:57", size: "154.3 MB", format: "MP4", resolution: "1080p", fps: "24fps" },
      { ...base, id: "b3", url: "https://youtu.be/Y-rmzh0PI3c", title: "Cosmos Laundromat — First Cycle", thumbnail: t("Y-rmzh0PI3c"),
        progress: 82, speed: "21.88MiB/s", eta: "ETA 0:04", status: "downloading", duration: "12:11", size: "533.0 MB", format: "MP4", resolution: "1080p", fps: "24fps" },
    ],
  };
  // Proof of the edit-ready claim: one row converting, one finished MP4.
  lists.convert = [lists.list[1], lists.list[2]];
  lists.settings = lists.list;
  lists.menu = lists.list;
  const history = lists[scene] || [];

  const invoke = async (cmd) => {
    switch (cmd) {
      case "load_download_history": return JSON.stringify(history);
      case "get_ytdlp_version": return { version: "2026.10.21", source: "bundled" };
      case "check_for_update": return null;
      case "read_clipboard_text": return q.get("clip") || "";
      default: return null;
    }
  };
  window.__TAURI__ = {
    core: { invoke, convertFileSrc: (p) => p },
    event: { listen: async () => () => {} },
    app: { getVersion: async () => "__APP_VERSION__" }, // filled in by capture.cjs
    opener: { openUrl: async () => {}, openPath: async () => {} },
  };
})();
