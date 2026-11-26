// PolyTabletop service worker: simple cache so the page works offline.
const CACHE = "polytabletop-2";
const FILES = ["./", "index.html", "jocuri/index.html", "manifest.webmanifest", "icon.svg", "icon-192.png", "icon-512.png", "icon-maskable-512.png"];
self.addEventListener("install", e => {
  e.waitUntil(caches.open(CACHE).then(c => Promise.all(FILES.map(f => c.add(f).catch(() => {})))).then(() => self.skipWaiting()));
});
self.addEventListener("activate", e => {
  e.waitUntil(caches.keys().then(ks => Promise.all(ks.filter(k => k !== CACHE).map(k => caches.delete(k)))).then(() => self.clients.claim()));
});
self.addEventListener("fetch", e => {
  const r = e.request;
  if (r.method !== "GET" || r.headers.has("range") || new URL(r.url).origin !== location.origin) return;
  const key = new URL(r.url).pathname;
  // capturile și clipurile (docs/media) nu se pun în cache: browserul le cere direct
  if (key.includes("/media/")) return;
  // network first (updates arrive), cache as the offline fallback; ?lang=xx shares the page entry
  e.respondWith(fetch(r).then(res => {
    if (res.status === 200 && res.type === "basic") {
      const copy = res.clone();
      e.waitUntil(caches.open(CACHE).then(c => c.put(key, copy)).catch(() => {}));
    }
    return res;
  }).catch(async () => (await caches.match(key)) || (r.mode === "navigate" && key.includes("/jocuri/") ? await caches.match("jocuri/index.html") : null) || (await caches.match("index.html")) || Response.error()));
});
