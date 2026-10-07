// 5BX Progressive Web App Service Worker
const CACHE_NAME = '5bx-cache-v1';

const STATIC_ASSETS = [
  '/',
  '/index.html',
  '/manifest.json',
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
  // Pass API requests directly to network, fallback to cache for static assets
  if (event.request.url.includes('/api/')) {
    event.respondWith(
      fetch(event.request).catch(() => {
        // Return cached response if available
        return caches.match(event.request);
      })
    );
    return;
  }

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
