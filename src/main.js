// Confirm modal system
function showConfirm(message) {
  return new Promise((resolve) => {
    let overlay = document.querySelector(".confirm-overlay");
    if (overlay) overlay.remove();
    overlay = document.createElement("div");
    overlay.className = "confirm-overlay";
    overlay.innerHTML = `
      <div class="confirm-card">
        <div class="confirm-message">${message.replace(/\n/g, "<br>")}</div>
        <div class="confirm-actions">
          <button class="confirm-btn confirm-cancel">Cancel</button>
          <button class="confirm-btn confirm-ok">Continue</button>
        </div>
      </div>
    `;
    document.body.appendChild(overlay);
    requestAnimationFrame(() => overlay.classList.add("visible"));
    const close = (result) => {
      overlay.classList.remove("visible");
      setTimeout(() => overlay.remove(), 200);
      resolve(result);
    };
    overlay.querySelector(".confirm-cancel").addEventListener("click", () => close(false));
    overlay.querySelector(".confirm-ok").addEventListener("click", () => close(true));
    overlay.addEventListener("click", (e) => { if (e.target === overlay) close(false); });
  });
}

// Toast notification system
function showToast(message, duration = 2000) {
  let container = document.querySelector(".toast-container");
  if (!container) {
    container = document.createElement("div");
    container.className = "toast-container";
    document.body.appendChild(container);
  }
  const toast = document.createElement("div");
  toast.className = "toast";
  toast.textContent = message;
  container.appendChild(toast);
  requestAnimationFrame(() => toast.classList.add("visible"));
  setTimeout(() => {
    toast.classList.remove("visible");
    setTimeout(() => toast.remove(), 300);
  }, duration);
}

// DOM Elements
const input = document.querySelector("#url-input");
const appRoot = document.querySelector(".app");
const button = document.querySelector("#download-btn");
const downloadList = document.querySelector("#download-list");
const clearBtn = document.querySelector(".clear-btn");
const settingsBtn = document.querySelector("#settings-toggle");
const settingsPanel = document.querySelector("#settings-panel");
const settingsPanelClose = document.querySelector("#settings-panel-close");
const browseBtn = document.querySelector("#browse-btn");
const downloadLocation = document.querySelector("#download-location");
const videoQuality = document.querySelector("#video-quality");
const audioOnlyEnabled = document.querySelector("#audio-only-enabled");
const autoStartClipboardEnabled = document.querySelector("#auto-start-clipboard-enabled");
const themeSelect = document.querySelector("#theme-select");
const settingsInfoBtn = document.querySelector("#settings-info-btn");
const settingsGuideModal = document.querySelector("#settings-guide-modal");
const settingsGuideClose = document.querySelector("#settings-guide-close");
const autoAddRibbon = document.querySelector("#auto-add-ribbon");
const clearListBtn = document.querySelector("#clear-list-btn");
const keepHistoryEnabled = document.querySelector("#keep-history-enabled");

// State
let downloads = [];
let downloadIdCounter = 0;
let saveDownloadsTimer = null;
let resizeTimer = null;
let lastRequestedWindowHeight = 0;
let resizeValidationTimer = null;
const transferStats = new Map();
const progressUiState = new Map();
const conversionState = new Map();

const DOWNLOADS_STORAGE_KEY = "downloadList";
const SAVE_DOWNLOADS_DEBOUNCE_MS = 200;
const RESIZE_DEBOUNCE_MS = 80;
const MIN_WINDOW_HEIGHT = 220;
const MAX_WINDOW_HEIGHT = 1200;
const MIN_VISIBLE_DOWNLOADS = 6;
const FALLBACK_DOWNLOAD_ROW_HEIGHT = 68;
const MAX_TERMINAL_HISTORY = 100;
const CONVERSION_PROGRESS_CAP = 93;
const SETTINGS_OVERLAY_BREAKPOINT = 620;
const CLIPBOARD_WATCH_INTERVAL_MS = 1200;
const FREE_DAILY_LIMIT = 5;
const DOWNLOAD_COUNTER_KEY = "dailyDownloadCounter";
const LICENSE_KEY_STORAGE = "proLicenseKey";
const LICENSE_INSTANCE_STORAGE = "proLicenseInstance";
const LICENSE_NAME_STORAGE = "proLicenseName";
const FIRST_RUN_KEY = "hasSeenOnboarding";
const LEMONSQUEEZY_CHECKOUT_URL = "https://superdownloads.lemonsqueezy.com/checkout/buy/21db1cfb-37f8-4371-8085-b5e30f89645f";
let lastRequestedMinWindowHeight = 0;
let lastAutoFilledClipboardText = "";
let clipboardWatchTimer = null;

// Freemium / license logic
function isProUser() {
  const key = localStorage.getItem(LICENSE_KEY_STORAGE);
  return key && key.trim().length > 0;
}

function getTodayKey() {
  return new Date().toISOString().slice(0, 10);
}

function getDailyCounter() {
  try {
    const data = JSON.parse(localStorage.getItem(DOWNLOAD_COUNTER_KEY) || "{}");
    if (data.date === getTodayKey()) {
      return data.count || 0;
    }
  } catch {}
  return 0;
}

function incrementDailyCounter() {
  const today = getTodayKey();
  const current = getDailyCounter();
  localStorage.setItem(DOWNLOAD_COUNTER_KEY, JSON.stringify({ date: today, count: current + 1 }));
  updateDownloadCounterUI();
}

function getRemainingDownloads() {
  if (isProUser()) return Infinity;
  return Math.max(0, FREE_DAILY_LIMIT - getDailyCounter());
}

function canDownload() {
  return isProUser() || getDailyCounter() < FREE_DAILY_LIMIT;
}

function updateDownloadCounterUI() {
  const counterEl = document.querySelector("#download-counter");
  if (!counterEl) return;
  if (isProUser()) {
    counterEl.textContent = "Pro";
    counterEl.className = "download-counter pro";
  } else {
    const remaining = getRemainingDownloads();
    counterEl.textContent = `${remaining}/${FREE_DAILY_LIMIT}`;
    counterEl.className = `download-counter ${remaining === 0 ? "exhausted" : remaining <= 2 ? "low" : ""}`;
  }
}

// License UI
function updateLicenseUI() {
  const statusEl = document.querySelector("#license-status");
  const inputRow = document.querySelector("#license-input-row");
  const activeRow = document.querySelector("#license-active-row");
  const keyDisplay = document.querySelector("#license-key-display");
  const upgradeLink = document.querySelector("#upgrade-pro-link");
  if (!statusEl) return;

  if (isProUser()) {
    const key = localStorage.getItem(LICENSE_KEY_STORAGE) || "";
    const name = localStorage.getItem(LICENSE_NAME_STORAGE) || "";
    const maskedKey = key.length > 8 ? key.slice(0, 4) + "..." + key.slice(-4) : key;
    statusEl.innerHTML = `<span class="license-badge pro">Pro${name ? " — " + name : ""}</span>`;
    inputRow.style.display = "none";
    activeRow.style.display = "flex";
    keyDisplay.textContent = maskedKey;
    upgradeLink.classList.add("hidden");
  } else {
    statusEl.innerHTML = `<span class="license-badge free">Free — ${FREE_DAILY_LIMIT}/day</span>`;
    inputRow.style.display = "flex";
    activeRow.style.display = "none";
    upgradeLink.classList.remove("hidden");
    upgradeLink.href = LEMONSQUEEZY_CHECKOUT_URL;
  }
  updateDownloadCounterUI();
}

async function handleActivateLicense() {
  const licenseInput = document.querySelector("#license-key-input");
  const btn = document.querySelector("#activate-license-btn");
  const key = licenseInput.value.trim();
  if (!key) { showToast("Paste your license key first"); return; }

  btn.textContent = "Activating...";
  btn.disabled = true;
  try {
    const result = await window.__TAURI__.core.invoke("activate_license", { key });
    if (result.valid) {
      localStorage.setItem(LICENSE_KEY_STORAGE, key);
      if (result.instance_id) localStorage.setItem(LICENSE_INSTANCE_STORAGE, result.instance_id);
      if (result.customer_name) localStorage.setItem(LICENSE_NAME_STORAGE, result.customer_name);
      updateLicenseUI();
      showToast("Pro license activated!");
    } else {
      showToast(result.error || "Invalid license key");
    }
  } catch (e) {
    showToast("Activation failed — check your connection");
  }
  btn.textContent = "Activate";
  btn.disabled = false;
}

async function handleDeactivateLicense() {
  const key = localStorage.getItem(LICENSE_KEY_STORAGE);
  const instanceId = localStorage.getItem(LICENSE_INSTANCE_STORAGE);
  if (!key) return;

  const confirmed = await showConfirm("Deactivate your Pro license on this device?\n\nYou can reactivate it later.");
  if (!confirmed) return;

  if (instanceId) {
    try {
      await window.__TAURI__.core.invoke("deactivate_license", { key, instanceId });
    } catch {}
  }
  localStorage.removeItem(LICENSE_KEY_STORAGE);
  localStorage.removeItem(LICENSE_INSTANCE_STORAGE);
  localStorage.removeItem(LICENSE_NAME_STORAGE);
  updateLicenseUI();
  showToast("License deactivated");
}

// First-run onboarding
function hasSeenOnboarding() {
  return localStorage.getItem(FIRST_RUN_KEY) === "true";
}

function markOnboardingSeen() {
  localStorage.setItem(FIRST_RUN_KEY, "true");
}

function showOnboarding() {
  let overlay = document.querySelector(".onboarding-overlay");
  if (overlay) return;
  overlay = document.createElement("div");
  overlay.className = "onboarding-overlay";
  overlay.innerHTML = `
    <div class="onboarding-card">
      <svg class="onboarding-icon" width="56" height="56" viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg">
        <defs>
          <linearGradient id="ob-bg" x1="0" y1="0" x2="0" y2="1">
            <stop offset="0%" stop-color="#162b73"/>
            <stop offset="100%" stop-color="#0a163f"/>
          </linearGradient>
          <linearGradient id="ob-arrow" x1="0.18" y1="0.05" x2="0.84" y2="0.96">
            <stop offset="0%" stop-color="#d5ebff"/>
            <stop offset="36%" stop-color="#8cd0ff"/>
            <stop offset="100%" stop-color="#30a7ff"/>
          </linearGradient>
        </defs>
        <rect width="1024" height="1024" rx="240" fill="url(#ob-bg)"/>
        <path d="M512 256c-33 0-58 25-58 58v248l-69-69c-22-22-58-22-80 0s-22 58 0 80l170 170c22 22 58 22 80 0l170-170c22-22 22-58 0-80s-58-22-80 0l-69 69V314c0-33-25-58-58-58z" fill="url(#ob-arrow)"/>
      </svg>
      <h2 class="onboarding-title">Welcome to Super Downloads</h2>
      <p class="onboarding-tagline">The video downloader built for editors.</p>
      <div class="onboarding-features">
        <div class="onboarding-feature">Download from YouTube, TikTok, X, Vimeo, Instagram, Facebook, LinkedIn</div>
        <div class="onboarding-feature">Every video is Premiere Pro ready — H.264/MP4</div>
        <div class="onboarding-feature">Paste a link, drag from browser, or enable clipboard auto-add</div>
      </div>
      <div class="onboarding-free-note">${FREE_DAILY_LIMIT} free downloads per day — upgrade to Pro for unlimited</div>
      <button class="onboarding-start-btn">Get Started</button>
    </div>
  `;
  document.body.appendChild(overlay);
  requestAnimationFrame(() => overlay.classList.add("visible"));
  overlay.querySelector(".onboarding-start-btn").addEventListener("click", () => {
    markOnboardingSeen();
    overlay.classList.remove("visible");
    setTimeout(() => overlay.remove(), 300);
    input.focus();
  });
}

// About modal
function showAbout() {
  let overlay = document.querySelector(".about-overlay");
  if (overlay) overlay.remove();
  overlay = document.createElement("div");
  overlay.className = "confirm-overlay";
  const version = document.querySelector("#version-label")?.textContent || "v1.1.0";
  overlay.innerHTML = `
    <div class="confirm-card about-card">
      <svg width="48" height="48" viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg">
        <defs>
          <linearGradient id="ab-bg" x1="0" y1="0" x2="0" y2="1"><stop offset="0%" stop-color="#162b73"/><stop offset="100%" stop-color="#0a163f"/></linearGradient>
          <linearGradient id="ab-ar" x1="0.18" y1="0.05" x2="0.84" y2="0.96"><stop offset="0%" stop-color="#d5ebff"/><stop offset="36%" stop-color="#8cd0ff"/><stop offset="100%" stop-color="#30a7ff"/></linearGradient>
        </defs>
        <rect width="1024" height="1024" rx="240" fill="url(#ab-bg)"/>
        <path d="M512 256c-33 0-58 25-58 58v248l-69-69c-22-22-58-22-80 0s-22 58 0 80l170 170c22 22 58 22 80 0l170-170c22-22 22-58 0-80s-58-22-80 0l-69 69V314c0-33-25-58-58-58z" fill="url(#ab-ar)"/>
      </svg>
      <h3 style="margin:8px 0 2px;font-size:13px;">Super Downloads</h3>
      <p style="color:var(--text-secondary);font-size:10px;margin-bottom:8px;">${version} — The video downloader built for editors.</p>
      <p style="color:var(--text-secondary);font-size:10px;margin-bottom:4px;">support@superdownloads.app</p>
      <p style="color:var(--text-secondary);font-size:10px;margin-bottom:12px;">superdownloads.app</p>
      <button class="confirm-btn confirm-ok" style="width:100%;">Close</button>
    </div>
  `;
  document.body.appendChild(overlay);
  requestAnimationFrame(() => overlay.classList.add("visible"));
  const close = () => { overlay.classList.remove("visible"); setTimeout(() => overlay.remove(), 200); };
  overlay.querySelector(".confirm-ok").addEventListener("click", close);
  overlay.addEventListener("click", (e) => { if (e.target === overlay) close(); });
}

// Initialize
loadSettings();
loadDownloads().then(() => {
  renderDownloadList();
  scheduleWindowResize();
});

// URL Validation
function isValidURL(url) {
  try {
    const urlObj = new URL(url);
    return urlObj.protocol === "http:" || urlObj.protocol === "https:";
  } catch {
    return false;
  }
}

function isVideoURL(url) {
  const videoDomains = [
    "youtube.com",
    "youtu.be",
    "vimeo.com",
    "vimeocdn.com",
    "instagram.com",
    "instagr.am",
    "tiktok.com",
    "twitter.com",
    "x.com",
    "facebook.com",
    "fb.watch",
    "fb.com",
    "linkedin.com",
    "lnkd.in"
  ];
  try {
    const urlObj = new URL(url);
    const hostname = urlObj.hostname.toLowerCase();
    return videoDomains.some(domain =>
      hostname === domain || hostname.endsWith("." + domain)
    );
  } catch {
    return false;
  }
}

function isPotentialBulkDownloadUrl(url) {
  try {
    const parsed = new URL(url.trim());
    const host = parsed.hostname.toLowerCase();
    const path = parsed.pathname.toLowerCase();
    const hasPlaylistQuery = parsed.searchParams.has("list");
    const hasVideoQuery = parsed.searchParams.has("v");

    const isYoutubeHost = (
      host.includes("youtube.com") ||
      host.includes("youtu.be") ||
      host.includes("music.youtube.com")
    );
    if (!isYoutubeHost) {
      return false;
    }

    if (
      path.startsWith("/@") ||
      path.startsWith("/channel/") ||
      path.startsWith("/c/") ||
      path.startsWith("/user/") ||
      path.startsWith("/playlist")
    ) {
      return true;
    }

    if (hasPlaylistQuery && (!hasVideoQuery || host.includes("youtu.be"))) {
      return true;
    }

    return false;
  } catch {
    return false;
  }
}

// Generate unique download ID
function generateDownloadId() {
  return `download-${Date.now()}-${++downloadIdCounter}`;
}

function isTerminalStatus(status) {
  return status === "completed" || status === "error" || status === "cancelled";
}

function normalizeVideoUrl(url) {
  try {
    const parsed = new URL(url.trim());
    parsed.hash = "";
    if (parsed.pathname !== "/") {
      parsed.pathname = parsed.pathname.replace(/\/+$/, "");
    }
    return parsed.toString();
  } catch {
    return url.trim();
  }
}

function trimHistory() {
  let terminalCount = 0;
  const removed = [];
  downloads = downloads.filter((download) => {
    if (!isTerminalStatus(download.status)) {
      return true;
    }
    terminalCount += 1;
    const keep = terminalCount <= MAX_TERMINAL_HISTORY;
    if (!keep) {
      removed.push(download);
    }
    return keep;
  });
  queueThumbnailCleanup(removed.map(download => download.thumbnail));
}

function isLocalThumbnailPath(thumbnail) {
  if (!thumbnail || typeof thumbnail !== "string") {
    return false;
  }
  if (thumbnail.startsWith("http://") || thumbnail.startsWith("https://") || thumbnail.startsWith("data:")) {
    return false;
  }
  return thumbnail.startsWith("/") || thumbnail.startsWith("file://");
}

function getThumbnailSrc(thumbnail) {
  if (!thumbnail) {
    return "";
  }
  if (!isLocalThumbnailPath(thumbnail)) {
    return thumbnail;
  }
  const normalizedPath = thumbnail.startsWith("file://")
    ? thumbnail.slice("file://".length)
    : thumbnail;
  if (window.__TAURI__?.core?.convertFileSrc) {
    return window.__TAURI__.core.convertFileSrc(normalizedPath);
  }
  return `file://${normalizedPath}`;
}

async function deleteCachedThumbnail(path) {
  if (!window.__TAURI__?.core?.invoke || !isLocalThumbnailPath(path)) {
    return;
  }
  try {
    const normalizedPath = path.startsWith("file://")
      ? path.slice("file://".length)
      : path;
    await window.__TAURI__.core.invoke("delete_cached_thumbnail", { path: normalizedPath });
  } catch (err) {
    console.warn("Failed to delete cached thumbnail:", path, err);
  }
}

function queueThumbnailCleanup(paths) {
  const uniquePaths = [...new Set((paths || []).filter(isLocalThumbnailPath))];
  if (uniquePaths.length === 0) {
    return;
  }
  for (const path of uniquePaths) {
    deleteCachedThumbnail(path);
  }
}

// Add new download to list
function addDownload(url) {
  const id = generateDownloadId();
  const download = {
    id,
    url,
    title: url, // Will be updated when download-started event fires
    thumbnail: "", // Will be updated from metadata
    progress: 0,
    conversionProgress: 0,
    speed: "",
    eta: "",
    resolution: "",
    duration: "", // "28:30"
    size: "", // "349.7 MB"
    format: "", // "MP4"
    fps: "", // "30fps"
    status: "queued",
    completed: false,
    filePath: "",
    error: null
  };
  
  downloads.unshift(download);
  trimHistory();
  renderDownloadList();
  scheduleSaveDownloads();
  
  return id;
}

// Update download by ID
function updateDownload(id, updates) {
  const index = downloads.findIndex(d => d.id === id);
  if (index !== -1) {
    const currentDownload = downloads[index];
    const nextDownload = { ...currentDownload, ...updates };
    downloads[index] = nextDownload;
    const statusChanged = updates.status !== undefined && updates.status !== currentDownload.status;
    trimHistory();
    
    // Try to update just the specific element instead of re-rendering everything
    const existingItem = document.querySelector(`[data-download-id="${id}"]`);
    if (
      existingItem &&
      !updates.title &&
      !updates.thumbnail &&
      !statusChanged &&
      updates.completed === undefined &&
      updates.error === undefined &&
      updates.filePath === undefined
    ) {
      // Just update progress bar and text for smoother updates
      const progressFill = existingItem.querySelector('.progress-bar-fill');
      const percentSpan = existingItem.querySelector('.download-item-percent');
      const speedSpan = existingItem.querySelector('.download-item-speed');
      const etaSpan = existingItem.querySelector('.download-item-eta');
      const metadataDiv = existingItem.querySelector('.download-item-metadata');
      const isConverting = downloads[index].status === "converting";
      const visualProgress = isConverting
        ? Math.max(1, Math.min(99, Math.round(downloads[index].conversionProgress || 0)))
        : downloads[index].progress;
      
      if (progressFill && (updates.progress !== undefined || updates.conversionProgress !== undefined)) {
        progressFill.style.width = `${visualProgress}%`;
      }
      if (percentSpan && (updates.progress !== undefined || updates.conversionProgress !== undefined)) {
        percentSpan.textContent = `${visualProgress}%`;
      }
      if (speedSpan && updates.speed !== undefined) {
        speedSpan.textContent = updates.speed;
      }
      if (etaSpan && updates.eta !== undefined) {
        etaSpan.textContent = updates.eta || "";
      }
      if (
        metadataDiv &&
        (
          updates.resolution !== undefined ||
          updates.duration !== undefined ||
          updates.size !== undefined ||
          updates.format !== undefined ||
          updates.fps !== undefined
        )
      ) {
        // Rebuild metadata line
        const d = downloads[index];
        const parts = [];
        if (d.duration) parts.push(d.duration);
        if (d.size) parts.push(d.size);
        if (d.format) parts.push(d.format);
        if (d.resolution) parts.push(d.resolution);
        if (d.fps) parts.push(d.fps);
        metadataDiv.textContent = parts.join(" · ");
      }
    } else {
      // Full re-render for major changes (status, title, thumbnail, completion)
      renderDownloadList();
    }
    scheduleSaveDownloads();
  }
}

// Remove download from list
function removeDownload(id) {
  const download = downloads.find(d => d.id === id);
  if (download?.thumbnail) {
    queueThumbnailCleanup([download.thumbnail]);
  }
  downloads = downloads.filter(d => d.id !== id);
  transferStats.delete(id);
  progressUiState.delete(id);
  stopConversionProgress(id);
  renderDownloadList();
  scheduleSaveDownloads();
}

async function clearDownloadList() {
  if (downloads.length === 0) {
    return;
  }

  const hasActiveDownloads = downloads.some(download => (
    download.status !== "completed" &&
    download.status !== "error" &&
    download.status !== "cancelled"
  ));

  if (hasActiveDownloads) {
    const shouldContinue = await showConfirm("Some downloads are still active. Clear the list anyway?\n\nActive downloads will continue in the background.");
    if (!shouldContinue) {
      return;
    }
  }

  const thumbnailPaths = downloads.map(download => download.thumbnail);
  downloads = [];
  queueThumbnailCleanup(thumbnailPaths);
  transferStats.clear();
  progressUiState.clear();
  for (const [downloadId] of conversionState) {
    stopConversionProgress(downloadId);
  }
  renderDownloadList();
  scheduleSaveDownloads();
}

// Render download list
function renderDownloadList() {
  syncMinimumWindowHeight();
  if (clearListBtn) {
    clearListBtn.disabled = downloads.length === 0;
  }

  if (downloads.length === 0) {
    downloadList.innerHTML = `
      <div class="empty-state">
        <svg class="empty-state-icon" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
          <path d="M12 5v10"></path>
          <path d="M7 12l5 5 5-5"></path>
          <path d="M5 19h14"></path>
        </svg>
        <div class="empty-state-title">Paste a video link to get started</div>
        <div class="empty-state-hint">YouTube, TikTok, X, Vimeo, Instagram, Facebook, LinkedIn</div>
      </div>
    `;
    scheduleWindowResize();
    return;
  }

  downloadList.innerHTML = downloads.map(download => createDownloadItemHTML(download)).join("");
  attachThumbnailFallbackHandlers();
  
  // Add event listeners for menu buttons
  downloadList.querySelectorAll(".download-item-menu-btn").forEach(btn => {
    btn.addEventListener("click", (e) => {
      e.stopPropagation();
      const id = btn.dataset.downloadId;
      toggleContextMenu(id);
    });
  });
  
  // Add event listeners for menu items
  downloadList.querySelectorAll(".menu-item").forEach(item => {
    item.addEventListener("click", (e) => {
      e.stopPropagation();
      const action = item.dataset.action;
      const id = item.dataset.downloadId;
      handleMenuAction(action, id);
      closeAllContextMenus();
    });
  });

  scheduleWindowResize();
}

function attachThumbnailFallbackHandlers() {
  const images = downloadList.querySelectorAll(".download-item-thumbnail img");
  images.forEach((img) => {
    if (img.dataset.fallbackBound === "true") {
      return;
    }
    img.dataset.fallbackBound = "true";
    img.addEventListener("error", () => {
      const container = img.closest(".download-item-thumbnail");
      if (!container) return;
      container.classList.add("placeholder");
      container.innerHTML = `
        <svg width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
          <path d="M12 7v7"></path>
          <path d="M8.5 11.5L12 15l3.5-3.5"></path>
        </svg>
      `;
    }, { once: true });
  });
}

// Toggle context menu
function toggleContextMenu(downloadId) {
  closeAllContextMenus();
  const menu = document.getElementById(`menu-${downloadId}`);
  if (menu) {
    menu.classList.add("visible");
  }
}

// Close all context menus
function closeAllContextMenus() {
  document.querySelectorAll(".download-item-menu").forEach(menu => {
    menu.classList.remove("visible");
  });
}

// Handle menu actions
async function handleMenuAction(action, downloadId) {
  const download = downloads.find(d => d.id === downloadId);
  if (!download) return;
  
  switch (action) {
    case "open-file":
      if (download.filePath) {
        try {
          const openerModule = await import("@tauri-apps/plugin-opener");
          const openPath = openerModule.openPath || openerModule.default?.openPath;
          if (openPath) {
            await openPath(download.filePath.trim());
          } else {
            const open = openerModule.open || openerModule.default?.open;
            if (open) await open(download.filePath.trim());
          }
        } catch (err) {
          try {
            if (window.__TAURI__?.core?.invoke) {
              await window.__TAURI__.core.invoke("reveal_in_finder", { path: download.filePath.trim() });
            }
          } catch {
            showToast("Could not open file");
          }
        }
      }
      break;

    case "show-in-finder":
      if (download.filePath) {

        const filePath = download.filePath.trim();
        
        // Method 1: Try using our custom Rust command (most reliable on macOS)
        if (window.__TAURI__?.core?.invoke) {
          try {

            await window.__TAURI__.core.invoke("reveal_in_finder", { path: filePath });

            return;
          } catch (rustErr) {
            console.warn("Rust command failed, trying opener plugin:", rustErr);
          }
        }
        
        // Method 2: Try opener plugin
        try {
          const openerModule = await import("@tauri-apps/plugin-opener");
          const revealItemInDir = openerModule.revealItemInDir || openerModule.default?.revealItemInDir;
          
          if (revealItemInDir) {

            await revealItemInDir(filePath);

            return;
          } else {
            throw new Error("revealItemInDir function not found");
          }
        } catch (openerErr) {
          console.warn("Opener plugin failed, trying directory fallback:", openerErr);
          
          // Method 3: Fallback - open the directory
          try {
            const openerModule = await import("@tauri-apps/plugin-opener");
            const open = openerModule.open || openerModule.default?.open;
            const dir = filePath.substring(0, filePath.lastIndexOf("/"));
            if (dir) {

              await open(dir);

              return;
            }
          } catch (dirErr) {
            console.error("Directory fallback also failed:", dirErr);
          }
        }
        
        showToast("Could not show file in Finder");
      } else {
        showToast("No file path available");
      }
      break;
      
    case "copy-link":
      try {
        if (download.url) {
          await navigator.clipboard.writeText(download.url);
          showToast("Link copied");
        }
      } catch (err) {
        try {
          if (window.__TAURI__?.core?.invoke) {
            await window.__TAURI__.core.invoke("copy_to_clipboard", { text: download.url });
            showToast("Link copied");
          }
        } catch (err2) {
          showToast("Failed to copy link");
        }
      }
      break;
      
    case "remove":
      removeDownload(downloadId);
      break;

    case "retry":
      retryDownload(downloadId);
      break;

    case "cancel":
      await cancelDownload(downloadId);
      break;
      
    default:
      console.warn("Unknown menu action:", action);
  }
}

async function cancelDownload(downloadId) {
  try {
    if (!window.__TAURI__?.core?.invoke) {
      throw new Error("Tauri API unavailable");
    }

    await window.__TAURI__.core.invoke("cancel_download", { downloadId });
    updateDownload(downloadId, {
      status: "cancelled",
      error: null,
      completed: false,
      eta: ""
    });
    transferStats.delete(downloadId);
    progressUiState.delete(downloadId);
    stopConversionProgress(downloadId);
  } catch (err) {
    console.error("Failed to cancel download:", err);
    showToast("Could not cancel download");
  }
}

async function retryDownload(downloadId) {
  const existing = downloads.find(download => download.id === downloadId);
  if (!existing || !existing.url) {
    return;
  }

  const newDownloadId = addDownload(existing.url);
  updateDownload(newDownloadId, {
    title: existing.title || existing.url,
    thumbnail: existing.thumbnail || ""
  });

  try {
    const settings = JSON.parse(localStorage.getItem("appSettings") || "{}");
    await window.__TAURI__.core.invoke("download_video", {
      url: existing.url,
      downloadId: newDownloadId,
      downloadLocation: settings.downloadLocation || null,
      quality: settings.videoQuality || "best",
      format: settings.outputFormat || "mp4"
    });
    return true;
  } catch (err) {
    console.error("Retry failed to start:", err);
    updateDownload(newDownloadId, {
      status: "error",
      error: getFriendlyErrorMessage(err?.toString() || err?.message || "Failed to retry download.")
    });
    return false;
  }
}

async function retryFailedDownloads() {
  const failedDownloads = downloads.filter(download => download.status === "error");
  if (failedDownloads.length === 0) {
    return;
  }

  for (const failedDownload of failedDownloads) {
    await retryDownload(failedDownload.id);
    await new Promise(resolve => setTimeout(resolve, 120));
  }
}

// Create HTML for a single download item
function createDownloadItemHTML(download) {
  const statusClass = download.status === "error" ? "error" : 
                     download.status === "completed" ? "completed" : 
                     download.status === "cancelled" ? "cancelled" :
                     download.status === "merging" ? "merging" : 
                     download.status === "converting" ? "converting" : "";
  const isTerminal = isTerminalStatus(download.status);
  
  // Build metadata line
  const metadataParts = [];
  if (download.duration) metadataParts.push(download.duration);
  if (download.size) metadataParts.push(download.size);
  if (download.format) metadataParts.push(download.format);
  if (download.resolution) metadataParts.push(download.resolution);
  if (download.fps) metadataParts.push(download.fps);
  const metadataLine = metadataParts.length > 0 ? metadataParts.join(" · ") : "";
  const terminalLabel = download.status === "completed" ? "Completed ✓" :
    download.status === "cancelled" ? "Cancelled" :
    download.status === "error" ? "Failed" : "";
  const terminalStatusClass = download.status === "completed"
    ? "status-completed"
    : download.status === "cancelled"
      ? "status-cancelled"
      : download.status === "error"
        ? "status-error"
        : "";
  
  const canRetry = download.status === "error" || download.status === "completed" || download.status === "cancelled";
  const canCancel = !["completed", "error", "cancelled"].includes(download.status);

  const statusPill = getFinalStatusPill(download);
  const progressValue = download.status === "converting"
    ? Math.max(1, Math.min(99, Math.round(download.conversionProgress || 0)))
    : (download.status === "queued" || download.status === "starting")
      ? 8
    : download.status === "completed"
      ? 100
      : download.progress;

  return `
    <div class="download-item ${statusClass}" data-download-id="${download.id}">
      <div class="download-item-content">
        ${download.thumbnail ? `
          <div class="download-item-thumbnail">
            <img src="${escapeHtml(getThumbnailSrc(download.thumbnail))}" alt="${escapeHtml(download.title)}" />
          </div>
        ` : `
          <div class="download-item-thumbnail placeholder">
            <svg width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
              <path d="M12 7v7"></path>
              <path d="M8.5 11.5L12 15l3.5-3.5"></path>
            </svg>
          </div>
        `}
        <div class="download-item-info">
          <div class="download-item-header">
            <div class="download-item-title" title="${escapeHtml(download.title)}">${escapeHtml(download.title)}</div>
          </div>
          <div class="download-item-metadata ${metadataLine ? "" : "is-empty"}">${metadataLine ? escapeHtml(metadataLine) : "&nbsp;"}</div>
            <div class="download-item-progress ${isTerminal ? "is-terminal" : ""}">
              <div class="progress-bar-thin">
                <div class="progress-bar-fill ${download.status === 'converting' ? 'converting-pulse' : ''} ${(download.status === 'queued' || download.status === 'starting') ? 'starting-pulse' : ''}" style="width: ${progressValue}%"></div>
              </div>
            </div>
            <div class="download-item-progress-meta ${isTerminal ? "is-terminal" : ""}">
              ${download.status === 'converting' ? `
                <span class="download-item-converting">Converting to H.264...</span>
                <span class="download-item-percent converting">${progressValue}%</span>
              ` : (download.status === 'queued' || download.status === 'starting') ? `
                <span class="download-item-starting">Starting download...</span>
              ` : isTerminal ? `
                ${terminalLabel ? `<span class="download-item-terminal ${terminalStatusClass}">${terminalLabel}</span>` : ""}
              ` : `
                <span class="download-item-percent">${download.progress}%</span>
                ${download.speed ? `<span class="download-item-speed">${download.speed}</span>` : ''}
                ${download.eta ? `<span class="download-item-eta">${download.eta}</span>` : ''}
              `}
            </div>
          ${download.error && download.status !== "cancelled" ? `<div class="download-item-error">${escapeHtml(download.error)}</div>` : ''}
        </div>
        <div class="download-item-actions">
          ${statusPill}
          <button class="download-item-menu-btn" aria-label="More options" title="More options" data-download-id="${download.id}">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <circle cx="12" cy="5" r="1"></circle>
              <circle cx="12" cy="12" r="1"></circle>
              <circle cx="12" cy="19" r="1"></circle>
            </svg>
          </button>
          <div class="download-item-menu" id="menu-${download.id}">
            ${download.filePath ? `<button class="menu-item" data-action="open-file" data-download-id="${download.id}">Open File</button>` : ""}
            <button class="menu-item" data-action="show-in-finder" data-download-id="${download.id}">Show in Finder</button>
            <button class="menu-item" data-action="copy-link" data-download-id="${download.id}">Copy Link Address</button>
            ${canRetry ? `<button class="menu-item" data-action="retry" data-download-id="${download.id}">Retry Download</button>` : ""}
            ${canCancel ? `<button class="menu-item" data-action="cancel" data-download-id="${download.id}">Cancel Download</button>` : ""}
            <button class="menu-item" data-action="remove" data-download-id="${download.id}">Remove from List</button>
          </div>
        </div>
      </div>
    </div>
  `;
}

function escapeHtml(text) {
  const div = document.createElement("div");
  div.textContent = text;
  return div.innerHTML;
}

function getStatusText(status) {
  const statusMap = {
    "queued": "Queued",
    "starting": "Starting",
    "downloading": "Downloading",
    "merging": "Merging",
    "converting": "Converting to H.264...",
    "completed": "Completed",
    "error": "Error"
  };
  return statusMap[status] || status;
}

function getFinalStatusPill(download) {
  if (download.status === "completed") {
    return "";
  }
  if (download.status === "error") {
    return '<span class="download-item-status-pill status-failed" aria-label="Failed">Failed</span>';
  }
  if (download.status === "cancelled") {
    return "";
  }
  return "";
}

function parseSpeedToKBps(speedText) {
  if (!speedText) return null;
  const match = String(speedText).trim().match(/^([\d.]+)\s*(KiB\/s|MiB\/s|KB\/s|MB\/s)$/i);
  if (!match) return null;
  const value = parseFloat(match[1]);
  const unit = match[2].toLowerCase();
  if (!Number.isFinite(value)) return null;
  if (unit === "mib/s" || unit === "mb/s") return value * 1024;
  return value;
}

function formatSpeedFromKBps(kbps) {
  if (!Number.isFinite(kbps) || kbps <= 0) return "";
  if (kbps >= 1024) {
    return `${(kbps / 1024).toFixed(2)}MiB/s`;
  }
  return `${kbps.toFixed(0)}KiB/s`;
}

function formatEtaFromSeconds(seconds) {
  if (!Number.isFinite(seconds) || seconds <= 0 || seconds > 86400) return "";
  const rounded = Math.round(seconds);
  const mins = Math.floor(rounded / 60);
  const secs = rounded % 60;
  return `ETA ${mins}:${String(secs).padStart(2, "0")}`;
}

function getSmoothedProgressMetrics(downloadId, percent, rawSpeed) {
  const now = performance.now();
  const parsedSpeed = parseSpeedToKBps(rawSpeed);
  const existing = transferStats.get(downloadId) || {
    lastProgress: percent,
    lastTimestamp: now,
    speedSamples: []
  };

  const deltaPercent = Math.max(0, percent - existing.lastProgress);
  const deltaSeconds = Math.max(0.001, (now - existing.lastTimestamp) / 1000);
  const percentPerSecond = deltaPercent / deltaSeconds;

  if (parsedSpeed) {
    existing.speedSamples.push(parsedSpeed);
    if (existing.speedSamples.length > 6) {
      existing.speedSamples.shift();
    }
  }

  existing.lastProgress = percent;
  existing.lastTimestamp = now;
  transferStats.set(downloadId, existing);

  const averageSpeed = existing.speedSamples.length > 0
    ? existing.speedSamples.reduce((sum, value) => sum + value, 0) / existing.speedSamples.length
    : null;
  const etaSeconds = percentPerSecond > 0 ? (100 - percent) / percentPerSecond : null;

  return {
    speed: averageSpeed ? formatSpeedFromKBps(averageSpeed) : (rawSpeed || ""),
    eta: formatEtaFromSeconds(etaSeconds)
  };
}

function shouldRenderProgressUpdate(downloadId, percent, status) {
  const now = performance.now();
  const previous = progressUiState.get(downloadId);
  if (!previous) {
    progressUiState.set(downloadId, { ts: now, percent, status });
    return true;
  }

  const statusChanged = previous.status !== status;
  const progressJumped = Math.abs(percent - previous.percent) >= 1;
  const intervalElapsed = now - previous.ts >= 120;

  if (statusChanged || progressJumped || intervalElapsed) {
    progressUiState.set(downloadId, { ts: now, percent, status });
    return true;
  }

  return false;
}

function stopConversionProgress(downloadId) {
  const state = conversionState.get(downloadId);
  if (state?.timer) {
    clearInterval(state.timer);
  }
  conversionState.delete(downloadId);
}

function startConversionProgress(downloadId) {
  const existing = conversionState.get(downloadId);
  if (existing?.timer) {
    return;
  }

  const timer = setInterval(() => {
    const download = downloads.find(d => d.id === downloadId);
    if (!download || download.status !== "converting") {
      stopConversionProgress(downloadId);
      return;
    }

    const current = Number.isFinite(download.conversionProgress) ? download.conversionProgress : 0;
    if (current >= CONVERSION_PROGRESS_CAP) {
      return;
    }

    // Keep conversion progress believable: smooth and slower near the end,
    // and never pretend it is "almost done" for too long.
    const bump = current < 25
      ? 0.95
      : current < 55
        ? 0.65
        : current < 78
          ? 0.4
          : 0.2;
    const next = Math.min(CONVERSION_PROGRESS_CAP, current + bump);
    updateDownload(downloadId, { conversionProgress: next });
  }, 420);

  conversionState.set(downloadId, { timer });
}

function isHistoryEnabled() {
  return keepHistoryEnabled?.checked === true;
}

function scheduleSaveDownloads() {
  if (!isHistoryEnabled()) return;
  clearTimeout(saveDownloadsTimer);
  saveDownloadsTimer = setTimeout(() => saveDownloads(), SAVE_DOWNLOADS_DEBOUNCE_MS);
}

async function loadDownloads() {
  if (!isHistoryEnabled()) {
    downloads = [];
    return;
  }
  try {
    if (!window.__TAURI__?.core?.invoke) return;
    const json = await window.__TAURI__.core.invoke("load_download_history");
    const parsed = JSON.parse(json || "[]");
    if (Array.isArray(parsed) && parsed.length > 0) {
      downloads = parsed;
      downloadIdCounter = downloads.length;
    }
  } catch {
    downloads = [];
  }
}

async function saveDownloads() {
  if (!isHistoryEnabled()) return;
  try {
    if (!window.__TAURI__?.core?.invoke) return;
    const serializable = downloads.filter(d => isTerminalStatus(d.status));
    await window.__TAURI__.core.invoke("save_download_history", {
      data: JSON.stringify(serializable)
    });
  } catch {}
}

function scheduleWindowResize() {
  clearTimeout(resizeTimer);
  clearTimeout(resizeValidationTimer);
  resizeTimer = setTimeout(() => {
    resizeWindowToFitDownloads();
  }, RESIZE_DEBOUNCE_MS);
}

function getDownloadListContentHeight() {
  const styles = window.getComputedStyle(downloadList);
  const paddingTop = parseFloat(styles.paddingTop || "0");
  const paddingBottom = parseFloat(styles.paddingBottom || "0");
  const listRect = downloadList.getBoundingClientRect();
  const items = downloadList.querySelectorAll(".download-item, .empty-state");

  if (items.length === 0) {
    return Math.ceil(90 + paddingTop + paddingBottom);
  }

  const lastItemRect = items[items.length - 1].getBoundingClientRect();
  return Math.ceil((lastItemRect.bottom - listRect.top) + paddingBottom);
}

function getSingleDownloadRowHeight() {
  const firstItem = downloadList.querySelector(".download-item");
  if (!firstItem) {
    return FALLBACK_DOWNLOAD_ROW_HEIGHT;
  }
  return Math.ceil(firstItem.getBoundingClientRect().height);
}

function getThreeDownloadsMinimumInnerHeight() {
  const topBarHeight = document.querySelector(".top-bar")?.offsetHeight || 0;
  const listStyles = window.getComputedStyle(downloadList);
  const paddingTop = parseFloat(listStyles.paddingTop || "0");
  const paddingBottom = parseFloat(listStyles.paddingBottom || "0");
  const rowGap = parseFloat(listStyles.gap || "6");
  const rowHeight = getSingleDownloadRowHeight();
  const listNeededHeight = paddingTop + paddingBottom + (rowHeight * MIN_VISIBLE_DOWNLOADS) + (rowGap * (MIN_VISIBLE_DOWNLOADS - 1));
  const reservePadding = 18;
  return Math.ceil(topBarHeight + listNeededHeight + reservePadding);
}

async function syncMinimumWindowHeight() {
  if (!window.__TAURI__?.core?.invoke) {
    return;
  }

  const minimumHeight = Math.max(MIN_WINDOW_HEIGHT, getThreeDownloadsMinimumInnerHeight());
  if (Math.abs(minimumHeight - lastRequestedMinWindowHeight) < 2) {
    return;
  }

  try {
    await window.__TAURI__.core.invoke("set_min_window_height", { height: minimumHeight });
    lastRequestedMinWindowHeight = minimumHeight;
  } catch (err) {
    console.warn("Failed to set minimum window height:", err);
  }
}

async function resizeWindowToFitDownloads() {
  if (!window.__TAURI__?.core?.invoke) {
    return;
  }

  await syncMinimumWindowHeight();
  const topBarHeight = document.querySelector(".top-bar")?.offsetHeight || 0;
  const listHeight = getDownloadListContentHeight();
  const chromePadding = 34;
  const dynamicMinimumHeight = Math.max(MIN_WINDOW_HEIGHT, getThreeDownloadsMinimumInnerHeight());
  const screenLimit = Math.floor((window.screen?.availHeight || MAX_WINDOW_HEIGHT) * 0.9);
  const clampedHeight = Math.max(
    dynamicMinimumHeight,
    Math.min(topBarHeight + listHeight + chromePadding, MAX_WINDOW_HEIGHT, screenLimit)
  );

  const shrinkDelta = lastRequestedWindowHeight - clampedHeight;
  if (shrinkDelta > 0 && shrinkDelta < 28) {
    return;
  }

  if (Math.abs(clampedHeight - lastRequestedWindowHeight) < 2) {
    const currentInnerHeight = window.innerHeight || 0;
    if (Math.abs(clampedHeight - currentInnerHeight) < 2) {
      return;
    }
  }

  try {
    await window.__TAURI__.core.invoke("resize_window_height", { height: clampedHeight });
    lastRequestedWindowHeight = clampedHeight;

    resizeValidationTimer = setTimeout(async () => {
      const overflow = downloadList.scrollHeight - downloadList.clientHeight;
      if (overflow > 1) {
        const adjustedHeight = Math.max(
          dynamicMinimumHeight,
          Math.min(clampedHeight + Math.ceil(overflow + 14), MAX_WINDOW_HEIGHT, screenLimit)
        );
        if (adjustedHeight > lastRequestedWindowHeight) {
          await window.__TAURI__.core.invoke("resize_window_height", { height: adjustedHeight });
          lastRequestedWindowHeight = adjustedHeight;
        }
      }
    }, 90);
  } catch (err) {
    console.warn("Auto resize skipped:", err);
  }
}

function getFriendlyErrorMessage(errorMessage) {
  const rawMessage = String(errorMessage || "Download failed");
  const lowerMessage = rawMessage.toLowerCase();

  if (lowerMessage.includes("yt-dlp") && (lowerMessage.includes("not found") || lowerMessage.includes("no such file"))) {
    return "yt-dlp is not available. Install it with `brew install yt-dlp` and try again.";
  }

  if (lowerMessage.includes("ffmpeg") && (lowerMessage.includes("not found") || lowerMessage.includes("no such file"))) {
    return "ffmpeg is not available. Install it with `brew install ffmpeg` and try again.";
  }

  if (lowerMessage.includes("permission denied")) {
    return `${rawMessage}\n\nCheck folder permissions and try a different download location.`;
  }

  if (lowerMessage.includes("timed out") || lowerMessage.includes("network")) {
    return `${rawMessage}\n\nCheck your internet connection and try again.`;
  }

  if (lowerMessage.includes("age-restricted") || lowerMessage.includes("sign in to confirm your age")) {
    return "This video is age-restricted. Make sure you're logged into YouTube in Chrome and try again.";
  }

  if (lowerMessage.includes("tiktok") && (lowerMessage.includes("private") || lowerMessage.includes("unavailable"))) {
    return "This TikTok video is private or unavailable. Only public TikTok videos can be downloaded.";
  }

  if ((lowerMessage.includes("facebook") || lowerMessage.includes("fb")) && (lowerMessage.includes("login") || lowerMessage.includes("auth"))) {
    return "This Facebook video requires login. Log into Facebook in Chrome and try again.";
  }

  if (lowerMessage.includes("linkedin") && (lowerMessage.includes("login") || lowerMessage.includes("auth"))) {
    return "LinkedIn videos require authentication. Make sure you're logged into LinkedIn in Chrome.";
  }

  if ((lowerMessage.includes("twitter") || lowerMessage.includes("x.com")) && lowerMessage.includes("unavailable")) {
    return "This post is unavailable. It may have been deleted or the account may be suspended.";
  }

  return rawMessage;
}


// Load Settings
function loadSettings() {
  const settings = JSON.parse(localStorage.getItem("appSettings") || "{}");
  downloadLocation.value = settings.downloadLocation || getDefaultDownloadPath();
  videoQuality.value = settings.videoQuality || "best";
  const themePref = settings.theme || "dark";
  if (themeSelect) themeSelect.value = themePref;
  applyTheme(themePref);
  if (audioOnlyEnabled) {
    audioOnlyEnabled.checked = false;
  }
  if (autoStartClipboardEnabled) {
    autoStartClipboardEnabled.checked = false;
  }
  if (keepHistoryEnabled) {
    keepHistoryEnabled.checked = settings.keepHistory === true;
  }
  // Keep user preferences but force safety defaults OFF on each launch.
  saveSettings();
}

// Save Settings (auto-save on change)
function saveSettings() {
  const settings = {
    downloadLocation: downloadLocation.value,
    videoQuality: videoQuality.value,
    audioOnlyEnabled: audioOnlyEnabled?.checked === true,
    autoStartClipboardEnabled: autoStartClipboardEnabled?.checked === true,
    theme: themeSelect?.value || "dark",
    keepHistory: keepHistoryEnabled?.checked === true,
    outputFormat: "mp4" // Always use MP4 for best compatibility
  };
  localStorage.setItem("appSettings", JSON.stringify(settings));
}

function resolveTheme(pref) {
  if (pref === "system") {
    return window.matchMedia("(prefers-color-scheme: light)").matches ? "light" : "dark";
  }
  return pref === "light" ? "light" : "dark";
}

function applyTheme(pref) {
  const resolved = resolveTheme(pref);
  document.body.classList.toggle("theme-light", resolved === "light");
}

// Get Default Download Path
function getDefaultDownloadPath() {
  return "~/Downloads";
}

// Browse for Download Location - using custom Rust command
async function browseForLocation() {

  
  try {
    // Use our custom Rust command that opens a native folder picker
    const selected = await window.__TAURI__.core.invoke("pick_folder");
    

    
    if (selected) {
      downloadLocation.value = selected;
      saveSettings();

    }
  } catch (err) {
    console.error("Folder picker failed:", err);
    showToast("Could not open folder picker — type the path manually");
    downloadLocation.focus();
    downloadLocation.select();
  }
}

// Download Video
async function startDownloadForUrl(url, options = {}) {
  const { silent = false, focusAfterStart = false } = options;
  if (!url) {
    if (!silent) {
      showToast("Paste a URL first");
    }
    return "empty";
  }

  if (!isValidURL(url)) {
    if (!silent) {
      showToast("Invalid URL");
    }
    return "invalid";
  }

  if (!isVideoURL(url)) {
    if (!silent) {
      showToast("Not a supported video URL");
    }
    return "unsupported";
  }

  if (!canDownload()) {
    if (!silent) {
      showToast("Daily limit reached — upgrade to Pro for unlimited downloads");
    }
    return "limit-reached";
  }

  if (isPotentialBulkDownloadUrl(url)) {
    if (silent) {
      return "bulk-blocked";
    }
    const shouldContinue = await showConfirm("This link may start a bulk download (playlist/channel with many videos).\n\nDo you want to continue?");
    if (!shouldContinue) {
      return "cancelled-by-user";
    }
  }

  // Add download to list
  const downloadId = addDownload(url);
  updateDownload(downloadId, { status: "starting" });
  
  // Clear input
  input.value = "";
  if (focusAfterStart) {
    input.focus();
  }

  try {
    const settings = JSON.parse(localStorage.getItem("appSettings") || "{}");
    
    // The command returns immediately with download_id, actual download happens in background
    await window.__TAURI__.core.invoke("download_video", {
      url,
      downloadId: downloadId,
      downloadLocation: settings.downloadLocation || null,
      quality: settings.videoQuality || "best",
      format: settings.audioOnlyEnabled === true ? "mp3" : "mp4"
    });
    incrementDailyCounter();
    return "started";
  } catch (err) {
    console.error("Failed to invoke download_video:", err);
    // Only set error if invoke itself fails (not download process)
    updateDownload(downloadId, {
      status: "error",
      error: getFriendlyErrorMessage(err.toString() || err.message || "Failed to start download. Please check if yt-dlp is installed.")
    });
    return "invoke-error";
  }
}

async function downloadVideo() {
  const url = input.value.trim();
  await startDownloadForUrl(url, { focusAfterStart: true });
}

// Clear Input
function clearInput() {
  input.value = "";
  input.focus();
}

async function tryAutofillFromClipboard() {
  const autoAddEnabled = autoStartClipboardEnabled?.checked === true;
  if (!autoAddEnabled) {
    return;
  }

  const isFocused = document.hasFocus();
  if (isFocused && input.value.trim()) {
    return;
  }

  try {
    const text = (await getClipboardText())?.trim();
    if (!text || text === lastAutoFilledClipboardText) {
      return;
    }

    if (!isValidURL(text) || !isVideoURL(text)) {
      return;
    }

    if (isFocused) {
      input.value = text;
      input.dispatchEvent(new Event("input", { bubbles: true }));
    }

    const result = await startDownloadForUrl(text, { silent: true, focusAfterStart: false });
    if (result !== "empty") {
      lastAutoFilledClipboardText = text;
    }
  } catch {
    // Ignore clipboard permission failures and continue without autofill.
  }
}

function syncSettingsLayoutMode() {
  if (!appRoot) return;
  appRoot.classList.add("settings-overlay");
}

function startClipboardWatcher() {
  if (clipboardWatchTimer) {
    clearInterval(clipboardWatchTimer);
  }
  if (autoStartClipboardEnabled?.checked !== true) {
    clipboardWatchTimer = null;
    return;
  }
  clipboardWatchTimer = setInterval(() => {
    tryAutofillFromClipboard();
  }, CLIPBOARD_WATCH_INTERVAL_MS);
}

function syncAutoAddUiState() {
  const enabled = autoStartClipboardEnabled?.checked === true;
  appRoot?.classList.toggle("auto-add-active", enabled);
  if (autoAddRibbon) {
    autoAddRibbon.setAttribute("aria-hidden", enabled ? "false" : "true");
  }
}

async function getClipboardText() {
  if (window.__TAURI__?.core?.invoke) {
    try {
      const tauriText = await window.__TAURI__.core.invoke("read_clipboard_text");
      if (typeof tauriText === "string" && tauriText.trim()) {
        return tauriText;
      }
    } catch {
      // Fallback to navigator clipboard below.
    }
  }

  if (!navigator.clipboard?.readText) {
    return "";
  }

  try {
    return await navigator.clipboard.readText();
  } catch {
    return "";
  }
}

// Toggle Settings Panel
function toggleSettings() {
  setSettingsOpen(!settingsPanel.classList.contains("open"));
}

// Close Settings Panel
function closeSettings() {
  setSettingsOpen(false);
}

function setSettingsGuideOpen(isOpen) {
  if (!settingsGuideModal) return;
  settingsGuideModal.classList.toggle("open", isOpen);
  settingsGuideModal.setAttribute("aria-hidden", isOpen ? "false" : "true");
}

function setSettingsOpen(isOpen) {
  settingsPanel.classList.toggle("open", isOpen);
  appRoot?.classList.toggle("settings-open", isOpen);
  if (!isOpen) {
    setSettingsGuideOpen(false);
  }
  if (settingsBtn) {
    settingsBtn.classList.toggle("open", isOpen);
    settingsBtn.setAttribute("aria-pressed", isOpen ? "true" : "false");
  }
}

// Event Listeners
button.addEventListener("click", downloadVideo);
if (clearListBtn) {
  clearListBtn.addEventListener("click", clearDownloadList);
}

document.addEventListener("click", (e) => {
  if (!e.target.closest(".download-item-menu") && !e.target.closest(".download-item-menu-btn")) {
    closeAllContextMenus();
  }
  if (
    settingsGuideModal?.classList.contains("open") &&
    e.target === settingsGuideModal
  ) {
    setSettingsGuideOpen(false);
    return;
  }
  if (settingsGuideModal?.classList.contains("open")) {
    return;
  }
  if (
    settingsPanel?.classList.contains("open") &&
    !e.target.closest("#settings-panel") &&
    !e.target.closest("#settings-toggle") &&
    !e.target.closest("#settings-guide-modal")
  ) {
    closeSettings();
  }
});

input.addEventListener("input", () => {
  if (input.value.trim() && !isValidURL(input.value.trim())) {
    input.classList.add("error");
  } else {
    input.classList.remove("error");
  }
});

input.addEventListener("keydown", (e) => {
  if (e.key === "Enter") {
    e.preventDefault();
    downloadVideo();
  } else if (e.key === "Escape") {
    clearInput();
  }
});

function isEditableTarget(target) {
  if (!target) return false;
  const tagName = target.tagName?.toLowerCase();
  return target.isContentEditable || tagName === "input" || tagName === "textarea" || tagName === "select";
}

document.addEventListener("keydown", async (e) => {
  const shortcutKey = e.metaKey || e.ctrlKey;

  if (shortcutKey && e.key.toLowerCase() === "k") {
    e.preventDefault();
    clearInput();
    return;
  }

  if (e.key === "Escape") {
    closeAllContextMenus();
    closeSettings();
    return;
  }

  if (shortcutKey && e.key.toLowerCase() === "v" && !isEditableTarget(document.activeElement)) {
    e.preventDefault();
    input.focus();
    try {
      const text = await getClipboardText();
      if (text) {
        input.value = text;
        input.dispatchEvent(new Event("input", { bubbles: true }));
        lastAutoFilledClipboardText = text.trim();
      }
    } catch {
      // Ignore clipboard permission failures; focus is still useful.
    }
    return;
  }

  if (!shortcutKey && e.key === "Enter" && !isEditableTarget(document.activeElement)) {
    e.preventDefault();
    downloadVideo();
  }
});

// Drag & drop support
document.addEventListener("dragover", (e) => {
  e.preventDefault();
  e.dataTransfer.dropEffect = "copy";
  appRoot?.classList.add("drag-over");
});

document.addEventListener("dragleave", (e) => {
  if (!e.relatedTarget || e.relatedTarget === document.documentElement) {
    appRoot?.classList.remove("drag-over");
  }
});

document.addEventListener("drop", async (e) => {
  e.preventDefault();
  appRoot?.classList.remove("drag-over");
  const text = e.dataTransfer.getData("text/plain") || e.dataTransfer.getData("text/uri-list");
  if (text) {
    const urls = text.split(/[\r\n]+/).map(u => u.trim()).filter(u => u && !u.startsWith("#"));
    for (const url of urls) {
      if (isValidURL(url) && isVideoURL(url)) {
        await startDownloadForUrl(url, { silent: true });
        await new Promise(resolve => setTimeout(resolve, 120));
      }
    }
    if (urls.length > 0 && !urls.some(u => isVideoURL(u))) {
      showToast("Not a supported video URL");
    }
  }
});

window.addEventListener("focus", () => {
  tryAutofillFromClipboard();
});

window.addEventListener("resize", () => {
  syncSettingsLayoutMode();
});

document.addEventListener("visibilitychange", () => {
  if (!document.hidden) {
    tryAutofillFromClipboard();
  }
});

setTimeout(() => {
  tryAutofillFromClipboard();
}, 250);
syncSettingsLayoutMode();
startClipboardWatcher();
syncAutoAddUiState();
setSettingsOpen(false);

// First-run onboarding
if (!hasSeenOnboarding()) {
  setTimeout(() => showOnboarding(), 300);
}

// Initialize download counter & license UI
updateDownloadCounterUI();
updateLicenseUI();

// License button listeners
document.querySelector("#activate-license-btn")?.addEventListener("click", handleActivateLicense);
document.querySelector("#deactivate-license-btn")?.addEventListener("click", handleDeactivateLicense);
document.querySelector("#upgrade-pro-link")?.addEventListener("click", (e) => {
  e.preventDefault();
  window.__TAURI__.opener.openUrl(LEMONSQUEEZY_CHECKOUT_URL);
});

// Dynamic version from Tauri
(async () => {
  try {
    const version = await window.__TAURI__.app.getVersion();
    const versionLabel = document.querySelector("#version-label");
    if (versionLabel && version) {
      versionLabel.textContent = `v${version}`;
    }
  } catch {}
})();

// ── Auto-update (one-click) ─────────────────────────────────────────
let sdUpdateInProgress = false;

function showUpdateBanner(info) {
  const banner = document.getElementById("update-banner");
  const text = document.getElementById("update-banner-text");
  if (!banner || !text) return;
  text.textContent = `Version ${info.version} is available`;
  banner.hidden = false;
}

async function checkForUpdate({ silent = true } = {}) {
  if (!window.__TAURI__?.core?.invoke) return;
  try {
    const info = await window.__TAURI__.core.invoke("check_for_update");
    if (info) {
      showUpdateBanner(info);
    } else if (!silent) {
      showToast("You're on the latest version");
    }
  } catch (e) {
    console.error("Update check failed:", e);
    if (!silent) showToast("Could not check for updates");
  }
}

async function startUpdate() {
  if (sdUpdateInProgress || !window.__TAURI__?.core?.invoke) return;
  sdUpdateInProgress = true;
  const btn = document.getElementById("update-banner-btn");
  const text = document.getElementById("update-banner-text");
  if (btn) {
    btn.disabled = true;
    btn.textContent = "Updating…";
  }
  if (text) text.textContent = "Starting update…";
  try {
    // Downloads, installs, and relaunches the app — no further user action.
    await window.__TAURI__.core.invoke("install_update");
  } catch (e) {
    console.error("Update failed:", e);
    sdUpdateInProgress = false;
    if (btn) {
      btn.disabled = false;
      btn.textContent = "Retry";
    }
    if (text) text.textContent = "Update failed";
    showToast("Update failed: " + e);
  }
}

// Live download progress in the banner.
window.__TAURI__.event.listen("update-progress", (event) => {
  const text = document.getElementById("update-banner-text");
  if (!text) return;
  const [downloaded, total] = event.payload || [];
  if (total) {
    const pct = Math.min(100, Math.round((downloaded / total) * 100));
    text.textContent = `Downloading update… ${pct}%`;
  } else {
    text.textContent = "Downloading update…";
  }
});

document.getElementById("update-banner-btn")?.addEventListener("click", startUpdate);

// Silent check on startup — only reveals the banner if an update exists.
setTimeout(() => checkForUpdate({ silent: true }), 1500);


clearBtn.addEventListener("click", clearInput);
settingsBtn.addEventListener("click", toggleSettings);
if (settingsPanelClose) {
  settingsPanelClose.addEventListener("click", closeSettings);
}
browseBtn.addEventListener("click", browseForLocation);
if (settingsInfoBtn) {
  settingsInfoBtn.addEventListener("click", (e) => {
    e.preventDefault();
    setSettingsGuideOpen(true);
  });
}
if (settingsGuideClose) {
  settingsGuideClose.addEventListener("click", () => setSettingsGuideOpen(false));
}

// About screen trigger
const versionLabel = document.querySelector("#version-label");
if (versionLabel) {
  versionLabel.style.cursor = "pointer";
  versionLabel.addEventListener("click", showAbout);
}

// Auto-save settings on change
downloadLocation.addEventListener("change", saveSettings);
downloadLocation.addEventListener("blur", saveSettings);
videoQuality.addEventListener("change", saveSettings);
if (audioOnlyEnabled) {
  audioOnlyEnabled.addEventListener("change", saveSettings);
}
if (autoStartClipboardEnabled) {
  autoStartClipboardEnabled.addEventListener("change", () => {
    saveSettings();
    syncAutoAddUiState();
    startClipboardWatcher();
  });
}
if (themeSelect) {
  themeSelect.addEventListener("change", () => {
    applyTheme(themeSelect.value);
    saveSettings();
  });
}

// Listen for system theme changes (when "System" is selected)
window.matchMedia("(prefers-color-scheme: light)").addEventListener("change", () => {
  const settings = JSON.parse(localStorage.getItem("appSettings") || "{}");
  if (settings.theme === "system") {
    applyTheme("system");
  }
});
if (keepHistoryEnabled) {
  keepHistoryEnabled.addEventListener("change", () => {
    saveSettings();
    if (keepHistoryEnabled.checked) {
      scheduleSaveDownloads();
    }
  });
}

// Tauri Event Listeners
window.__TAURI__.event.listen("download-started", (event) => {
  const [downloadId, title, thumbnail] = event.payload;
  transferStats.delete(downloadId);
  progressUiState.delete(downloadId);
  stopConversionProgress(downloadId);
  updateDownload(downloadId, {
    title: title || "Unknown Video",
    thumbnail: thumbnail || "",
    status: "downloading"
  });
});

window.__TAURI__.event.listen("download-progress", (event) => {
  const [downloadId, percent, resolution, speed, status] = event.payload;
  const currentDownload = downloads.find(d => d.id === downloadId);
  const nextStatus = status || "downloading";
  if (!shouldRenderProgressUpdate(downloadId, percent, nextStatus)) {
    return;
  }
  const smoothedMetrics = getSmoothedProgressMetrics(downloadId, percent, speed || "");
  const isConverting = nextStatus === "converting";
  if (isConverting) {
    startConversionProgress(downloadId);
  } else {
    stopConversionProgress(downloadId);
  }
  // Always update resolution if provided (even if empty, to clear old values)
  const updates = {
    progress: percent,
    speed: smoothedMetrics.speed,
    eta: smoothedMetrics.eta,
    conversionProgress: isConverting
      ? Math.max((currentDownload?.conversionProgress || 0), 5)
      : 0
  };
  if (!currentDownload || currentDownload.status !== nextStatus) {
    updates.status = nextStatus;
  }
  // Update resolution if provided (it might be detected during download or from metadata)
  // For YouTube, resolution will be empty during download - only show it after completion
  if (resolution !== undefined && resolution !== null) {
    updates.resolution = resolution.trim();
  } else {
    // Explicitly clear resolution if not provided (for YouTube during download)
    updates.resolution = "";
  }
  updateDownload(downloadId, updates);
});

window.__TAURI__.event.listen("download-metadata", (event) => {
  const [downloadId, duration, size, format, fps, thumbnail] = event.payload;
  updateDownload(downloadId, {
    duration: duration || "",
    size: size || "",
    format: format || "",
    fps: fps || "",
    thumbnail: thumbnail || ""
    // Note: resolution is sent via download-progress event
  });
});

window.__TAURI__.event.listen("download-finished", (event) => {
  const [downloadId, title, filePath, duration, size, format, resolution, fps, thumbnail] = event.payload;
  transferStats.delete(downloadId);
  progressUiState.delete(downloadId);
  stopConversionProgress(downloadId);
  const displayTitle = title || "Downloaded Video";
  updateDownload(downloadId, {
    title: displayTitle,
    progress: 100,
    status: "completed",
    completed: true,
    filePath: filePath || "",
    duration: duration || "",
    size: size || "",
    format: format || "",
    resolution: (resolution && resolution.trim()) ? resolution.trim() : "",
    fps: fps || "",
    thumbnail: thumbnail || "",
    eta: "",
    conversionProgress: 100
  });

  // Native macOS notification (only when app is not focused)
  if (!document.hasFocus() && window.__TAURI__?.core?.invoke) {
    window.__TAURI__.core.invoke("show_notification", {
      title: "Download Complete",
      body: displayTitle
    }).catch(() => {});
  }
});

window.__TAURI__.event.listen("download-error", (event) => {
  const [downloadId, errorMessage] = event.payload;
  
  // Add helpful message for Vimeo impersonation errors
  let displayError = getFriendlyErrorMessage(errorMessage || "Download failed");
  if (errorMessage && (errorMessage.includes("impersonation") || errorMessage.includes("impersonate") || errorMessage.includes("unauthentic"))) {
    if (errorMessage.includes("unauthenticated request") || errorMessage.includes("mature content")) {
      displayError = errorMessage + "\n\nThis Vimeo video may require:\n- Authentication (cookies/login)\n- VPN if you're in Europe\n- Or the video may be age-restricted\n\nTry: yt-dlp --cookies-from-browser chrome <URL>";
    } else {
      displayError = errorMessage + "\n\nNote: Impersonation support is installed, but this video may require authentication or have location restrictions.";
    }
  }
  
  updateDownload(downloadId, {
    status: "error",
    error: displayError,
    completed: false,
    eta: ""
  });
  transferStats.delete(downloadId);
  progressUiState.delete(downloadId);
  stopConversionProgress(downloadId);
});
