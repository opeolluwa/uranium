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
    "url": "assets/AllEmailView.5671b89b.css",
    "revision": null
  }, {
    "url": "assets/AllEmailView.cd3b5fe6.js",
    "revision": null
  }, {
    "url": "assets/AllNotesEntry.9fa3b05a.css",
    "revision": null
  }, {
    "url": "assets/AllNotesEntry.b90b3502.js",
    "revision": null
  }, {
    "url": "assets/AppEmptyState.390c9aa1.js",
    "revision": null
  }, {
    "url": "assets/AppEmptyState.c08edad6.css",
    "revision": null
  }, {
    "url": "assets/BaseTextarea.362c8c3c.css",
    "revision": null
  }, {
    "url": "assets/BaseTextarea.5c96b01b.js",
    "revision": null
  }, {
    "url": "assets/ConfirmOtpView.135d6ba0.css",
    "revision": null
  }, {
    "url": "assets/ConfirmOtpView.9069c5d9.js",
    "revision": null
  }, {
    "url": "assets/CreateEmailView.10bb73cf.js",
    "revision": null
  }, {
    "url": "assets/CreateEmailView.be01e759.css",
    "revision": null
  }, {
    "url": "assets/CreateNoteView.a11fa3db.js",
    "revision": null
  }, {
    "url": "assets/CreateNoteView.ff0ddcfe.css",
    "revision": null
  }, {
    "url": "assets/EditNoteView.9d44c0f0.css",
    "revision": null
  }, {
    "url": "assets/EditNoteView.d7aa324d.js",
    "revision": null
  }, {
    "url": "assets/EmailIndexView.5eb340c9.js",
    "revision": null
  }, {
    "url": "assets/ImportantEmailView.8f768f3e.js",
    "revision": null
  }, {
    "url": "assets/index.867c1d86.js",
    "revision": null
  }, {
    "url": "assets/index.bd597648.css",
    "revision": null
  }, {
    "url": "assets/index.f1396e21.js",
    "revision": null
  }, {
    "url": "assets/notes.eaa3a705.js",
    "revision": null
  }, {
    "url": "assets/NotesIndexView.276d6f45.js",
    "revision": null
  }, {
    "url": "assets/NotFoundView.7ba95500.js",
    "revision": null
  }, {
    "url": "assets/NotFoundView.a911ae56.css",
    "revision": null
  }, {
    "url": "assets/NotificationView.6842a7a8.js",
    "revision": null
  }, {
    "url": "assets/PasswordResetView.0a182570.css",
    "revision": null
  }, {
    "url": "assets/PasswordResetView.d7d211be.js",
    "revision": null
  }, {
    "url": "assets/ProfileView.51385863.js",
    "revision": null
  }, {
    "url": "assets/ProfileView.5b8e06e2.css",
    "revision": null
  }, {
    "url": "assets/ProjectsView.5d87ab3d.js",
    "revision": null
  }, {
    "url": "assets/ProjectsView.973ac42e.css",
    "revision": null
  }, {
    "url": "assets/SettingsView.76557c5a.js",
    "revision": null
  }, {
    "url": "assets/SignupView.a758e1f3.css",
    "revision": null
  }, {
    "url": "assets/SignupView.fb6396f5.js",
    "revision": null
  }, {
    "url": "assets/StarredEmailView.f2755a0d.js",
    "revision": null
  }, {
    "url": "assets/TodoIndexView.0d0e6ce0.css",
    "revision": null
  }, {
    "url": "assets/TodoIndexView.b006b32e.js",
    "revision": null
  }, {
    "url": "assets/web.6f6eab0c.js",
    "revision": null
  }, {
    "url": "index.html",
    "revision": "7c210416ff3573530815e97dda58a54e"
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
