// Service Worker
const CACHE_NAME = 'ferris8-v1';
const STATIC_ASSETS = [
    './',
    './index.html',
    './style.css',
    './main.js',
    './pkg/ferris8.js',
    './pkg/ferris8_bg.wasm'
];

// Installation : mise en cache des assets
self.addEventListener('install', (event) => {
    event.waitUntil(
        caches.open(CACHE_NAME)
            .then((cache) => cache.addAll(STATIC_ASSETS))
            .then(() => self.skipWaiting())
    );
});

// Activation : nettoyage des anciens caches
self.addEventListener('activate', (event) => {
    event.waitUntil(
        caches.keys()
            .then((cacheNames) => {
                return Promise.all(
                    cacheNames.map((cacheName) => {
                        if (cacheName !== CACHE_NAME) {
                            return caches.delete(cacheName);
                        }
                    })
                );
            })
            .then(() => self.clients.claim())
    );
});

// Stratégie cache-first pour les assets, network-first pour le reste
self.addEventListener('fetch', (event) => {
    if (STATIC_ASSETS.some(asset => event.request.url.includes(asset))) {
        // Cache-first pour les assets statiques
        event.respondWith(
            caches.match(event.request)
                .then((response) => response || fetch(event.request))
        );
    } else {
        // Network-first pour le reste
        event.respondWith(
            fetch(event.request)
                .catch(() => caches.match(event.request))
        );
    }
});