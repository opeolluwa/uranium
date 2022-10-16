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
define(['./workbox-3589c0c5'], (function (workbox) { 'use strict';

  /**
  * Welcome to your Workbox-powered service worker!
  *
  * You'll need to register this file in your web app.
  * See https://goo.gl/nhQhGp
  *
  * The rest of the code is auto-generated. Please don't update this file
  * directly; instead, make changes to your Workbox build configuration
  * and re-run your build process.
  * See https://goo.gl/2aRDsh
  */

  self.skipWaiting();
  workbox.clientsClaim();
  /**
   * The precacheAndRoute() method efficiently caches and responds to
   * requests for URLs in the manifest.
   * See https://goo.gl/S9QRab
   */

  workbox.precacheAndRoute([{
    "url": "assets/AppEmptyState.529521ab.js",
    "revision": null
  }, {
    "url": "assets/AppEmptyState.8f6b4102.css",
    "revision": null
  }, {
    "url": "assets/AppNetworkError.228aeec1.css",
    "revision": null
  }, {
    "url": "assets/AppNetworkError.d423b02d.js",
    "revision": null
  }, {
    "url": "assets/CreateEmailView.63077dd0.js",
    "revision": null
  }, {
    "url": "assets/CreateEmailView.632b2cbb.css",
    "revision": null
  }, {
    "url": "assets/EmailIndexView.f5516bd5.js",
    "revision": null
  }, {
    "url": "assets/EmailView.7d82402b.js",
    "revision": null
  }, {
    "url": "assets/EmailView.f015e947.css",
    "revision": null
  }, {
    "url": "assets/index.344cb491.css",
    "revision": null
  }, {
    "url": "assets/index.9718dd56.js",
    "revision": null
  }, {
    "url": "assets/NotFoundView.42d9b319.js",
    "revision": null
  }, {
    "url": "assets/NotFoundView.6ff6b9d3.css",
    "revision": null
  }, {
    "url": "assets/NotificationView.43755712.js",
    "revision": null
  }, {
    "url": "assets/ProjectsView.3eb714f5.css",
    "revision": null
  }, {
    "url": "assets/ProjectsView.c4a3859a.js",
    "revision": null
  }, {
    "url": "assets/SettingsView.4799f0d5.js",
    "revision": null
  }, {
    "url": "assets/SettingsView.75b548f1.css",
    "revision": null
  }, {
    "url": "assets/TodoView.3fc76809.js",
    "revision": null
  }, {
    "url": "assets/TodoView.e99b67a7.css",
    "revision": null
  }, {
    "url": "index.html",
    "revision": "f14fbe611ba5b3ce9249ec78d57a73fa"
  }, {
    "url": "registerSW.js",
    "revision": "1872c500de691dce40960bb85481de07"
  }, {
    "url": "manifest.webmanifest",
    "revision": "2278a49869c66e44212511dd80370d3c"
  }], {});
  workbox.cleanupOutdatedCaches();
  workbox.registerRoute(new workbox.NavigationRoute(workbox.createHandlerBoundToURL("index.html")));

}));
