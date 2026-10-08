// 5BX Progressive Web App Service Worker
const CACHE_NAME = '5bx-cache-v7';

const STATIC_ASSETS = [
  '/manifest.json',
  '/images/run.png',
  '/images/walk.png',
  '/images/c1_ex1.png',
  '/images/c1_ex2.png',
  '/images/c1_ex3.png',
  '/images/c1_ex4.png',
  '/images/c1_ex5.png',
  '/images/c2_ex1.png',
  '/images/c2_ex2.png',
  '/images/c2_ex3.png',
  '/images/c2_ex4.png',
  '/images/c2_ex5.png',
  '/images/c3_ex1.png',
  '/images/c3_ex2.png',
  '/images/c3_ex3.png',
  '/images/c3_ex4.png',
  '/images/c3_ex5.png'
];

self.addEventListener('install', (event) => {
  event.waitUntil(
    caches.open(CACHE_NAME).then((cache) => {
      return cache.addAll(STATIC_ASSETS);
    })
  );
  self.skipWaiting();
});

self.addEventListener('activate', (event) => {
  event.waitUntil(
    caches.keys().then((keys) => {
      return Promise.all(
        keys.filter((key) => key !== CACHE_NAME).map((key) => caches.delete(key))
      );
    })
  );
  self.clients.claim();
});

self.addEventListener('fetch', (event) => {
  const url = new URL(event.request.url);

  // Bypass API requests to network directly
  if (url.pathname.startsWith('/api/')) {
    event.respondWith(
      fetch(event.request).catch(() => caches.match(event.request))
    );
    return;
  }

  // Network-First for HTML navigation and root index: Always fetch fresh deploy, fall back to cache only when offline
  if (event.request.mode === 'navigate' || url.pathname === '/' || url.pathname === '/index.html') {
    event.respondWith(
      fetch(event.request)
        .then((response) => {
          if (response && response.status === 200) {
            const copy = response.clone();
            caches.open(CACHE_NAME).then((cache) => cache.put(event.request, copy));
          }
          return response;
        })
        .catch(() => {
          return caches.match(event.request) || caches.match('/index.html');
        })
    );
    return;
  }

  // Stale-While-Revalidate / Cache-First for versioned static assets (images, fonts, hashed assets)
  event.respondWith(
    caches.match(event.request).then((cachedResponse) => {
      if (cachedResponse) {
        return cachedResponse;
      }
      return fetch(event.request).then((response) => {
        if (!response || response.status !== 200 || response.type !== 'basic') {
          return response;
        }
        const responseToCache = response.clone();
        caches.open(CACHE_NAME).then((cache) => {
          cache.put(event.request, responseToCache);
        });
        return response;
      });
    })
  );
});
