/**
 * Copyright 2018 Google Inc. All Rights Reserved.
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *     http://www.apache.org/licenses/LICENSE-2.0
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// If the loader is already loaded, just stop.
if (!self.define) {
  let registry = {};

  // Used for `eval` and `importScripts` where we can't get script URL by other means.
  // In both cases, it's safe to use a global var because those functions are synchronous.
  let nextDefineUri;

  const singleRequire = (uri, parentUri) => {
    uri = new URL(uri + ".js", parentUri).href;
    return registry[uri] || (
      
        new Promise(resolve => {
          if ("document" in self) {
            const script = document.createElement("script");
            script.src = uri;
            script.onload = resolve;
            document.head.appendChild(script);
          } else {
            nextDefineUri = uri;
            importScripts(uri);
            resolve();
          }
        })
      
      .then(() => {
        let promise = registry[uri];
        if (!promise) {
          throw new Error(`Module ${uri} didn’t register its module`);
        }
        return promise;
      })
    );
  };

  self.define = (depsNames, factory) => {
    const uri = nextDefineUri || ("document" in self ? document.currentScript.src : "") || location.href;
    if (registry[uri]) {
      // Module is already loading or loaded.
      return;
    }
    let exports = {};
    const require = depUri => singleRequire(depUri, uri);
    const specialDeps = {
      module: { uri },
      exports,
      require
    };
    registry[uri] = Promise.all(depsNames.map(
      depName => specialDeps[depName] || require(depName)
    )).then(deps => {
      factory(...deps);
      return exports;
    });
  };
}
define(['./workbox-cc8906f2'], (function (workbox) { 'use strict';

  self.addEventListener('message', (event) => {
    if (event.data && event.data.type === 'SKIP_WAITING') {
      self.skipWaiting();
    }
  });




  /**
   * The precacheAndRoute() method efficiently caches and responds to
   * requests for URLs in the manifest.
   * See https://goo.gl/S9QRab
   */
  workbox.precacheAndRoute([
    {
      "url": "assets/CreateEmailView.4bd8c11d.js",
      "revision": null
    },
    {
      "url": "assets/CreateEmailView.5d8ac838.css",
      "revision": null
    },
    {
      "url": "assets/EmailIndexView.cfff1652.js",
      "revision": null
    },
    {
      "url": "assets/EmailView.2c2ae286.js",
      "revision": null
    },
    {
      "url": "assets/EmailView.4480ec03.css",
      "revision": null
    },
    {
      "url": "assets/index.b28eb709.css",
      "revision": null
    },
    {
      "url": "assets/index.e08d3992.js",
      "revision": null
    },
    {
      "url": "assets/NotFoundView.6ff6b9d3.css",
      "revision": null
    },
    {
      "url": "assets/NotFoundView.75e655de.js",
      "revision": null
    },
    {
      "url": "assets/NotificationView.3b9163ba.js",
      "revision": null
    },
    {
      "url": "assets/ProjectsView.4d41a0c0.js",
      "revision": null
    },
    {
      "url": "assets/ProjectsView.6b1b442d.css",
      "revision": null
    },
    {
      "url": "assets/SettingsView.3de62e32.js",
      "revision": null
    },
    {
      "url": "assets/TodoView.2d5ae8dd.js",
      "revision": null
    },
    {
      "url": "index.html",
      "revision": "118ddd4cc8b91a2154efa3077569ea4e"
    },
    {
      "url": "registerSW.js",
      "revision": "e42e5a2c306df510d950e3c31c28dcbe"
    },
    {
      "url": "manifest.webmanifest",
      "revision": "a36b2d4c1ba674104a8943bc62d88842"
    }
  ], {});
  workbox.cleanupOutdatedCaches();
  workbox.registerRoute(new workbox.NavigationRoute(workbox.createHandlerBoundToURL("index.html")));

}));
