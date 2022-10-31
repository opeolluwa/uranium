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
    "url": "assets/AllEmailView.98c87d8e.css",
    "revision": null
  }, {
    "url": "assets/AllEmailView.cb4e1286.js",
    "revision": null
  }, {
    "url": "assets/AllNotesEntry.6ba1060e.css",
    "revision": null
  }, {
    "url": "assets/AllNotesEntry.d03233c2.js",
    "revision": null
  }, {
    "url": "assets/AppEmptyState.32bdad68.js",
    "revision": null
  }, {
    "url": "assets/AppEmptyState.73cc1daf.css",
    "revision": null
  }, {
    "url": "assets/BaseTextarea.4cce83bb.js",
    "revision": null
  }, {
    "url": "assets/BaseTextarea.5e69e4dc.css",
    "revision": null
  }, {
    "url": "assets/ConfirmOtpView.2a9a12b9.css",
    "revision": null
  }, {
    "url": "assets/ConfirmOtpView.d08608f4.js",
    "revision": null
  }, {
    "url": "assets/CreateEmailView.4c162db8.js",
    "revision": null
  }, {
    "url": "assets/CreateEmailView.be01e759.css",
    "revision": null
  }, {
    "url": "assets/CreateNoteView.0653717a.css",
    "revision": null
  }, {
    "url": "assets/CreateNoteView.fcbb1413.js",
    "revision": null
  }, {
    "url": "assets/EditNoteView.bdaa3ab2.css",
    "revision": null
  }, {
    "url": "assets/EditNoteView.c1f02035.js",
    "revision": null
  }, {
    "url": "assets/EmailIndexView.612b3993.js",
    "revision": null
  }, {
    "url": "assets/ImportantEmailView.8f768f3e.js",
    "revision": null
  }, {
    "url": "assets/index.b4f8f17b.css",
    "revision": null
  }, {
    "url": "assets/index.f8e3d68e.js",
    "revision": null
  }, {
    "url": "assets/notes.67e3bc46.js",
    "revision": null
  }, {
    "url": "assets/NotesIndexView.6cbd356e.js",
    "revision": null
  }, {
    "url": "assets/NotFoundView.24f8b654.js",
    "revision": null
  }, {
    "url": "assets/NotFoundView.c49cc0d1.css",
    "revision": null
  }, {
    "url": "assets/NotificationView.c68c53c4.js",
    "revision": null
  }, {
    "url": "assets/PasswordResetView.5a31dfa9.js",
    "revision": null
  }, {
    "url": "assets/PasswordResetView.a4eda510.css",
    "revision": null
  }, {
    "url": "assets/ProfileView.9e1a2c9c.js",
    "revision": null
  }, {
    "url": "assets/ProfileView.c73f6505.css",
    "revision": null
  }, {
    "url": "assets/ProjectsView.19472de3.js",
    "revision": null
  }, {
    "url": "assets/ProjectsView.973ac42e.css",
    "revision": null
  }, {
    "url": "assets/SettingsView.76557c5a.js",
    "revision": null
  }, {
    "url": "assets/SignupView.2d2a5f95.css",
    "revision": null
  }, {
    "url": "assets/SignupView.55db07e6.js",
    "revision": null
  }, {
    "url": "assets/StarredEmailView.f2755a0d.js",
    "revision": null
  }, {
    "url": "assets/TodoIndexView.644c76ed.css",
    "revision": null
  }, {
    "url": "assets/TodoIndexView.ba3bacd3.js",
    "revision": null
  }, {
    "url": "assets/web.eb987510.js",
    "revision": null
  }, {
    "url": "index.html",
    "revision": "0e6127c662f633992c97608625810a69"
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
