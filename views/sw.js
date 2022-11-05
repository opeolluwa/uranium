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
    "url": "assets/AllEmailView.3f4b464f.js",
    "revision": null
  }, {
    "url": "assets/AllEmailView.6a397453.css",
    "revision": null
  }, {
    "url": "assets/AllNotesEntry.5c78dd7a.css",
    "revision": null
  }, {
    "url": "assets/AllNotesEntry.d875a1ef.js",
    "revision": null
  }, {
    "url": "assets/AllTodoViews.7069778d.js",
    "revision": null
  }, {
    "url": "assets/AllTodoViews.d8a000c5.css",
    "revision": null
  }, {
    "url": "assets/AppEmptyState.03179617.js",
    "revision": null
  }, {
    "url": "assets/AppEmptyState.52e2987e.css",
    "revision": null
  }, {
    "url": "assets/AppModal.439edf4c.js",
    "revision": null
  }, {
    "url": "assets/AppModal.8343d420.css",
    "revision": null
  }, {
    "url": "assets/BaseTextarea.21dd1aaa.js",
    "revision": null
  }, {
    "url": "assets/BaseTextarea.2be742d1.css",
    "revision": null
  }, {
    "url": "assets/ConfirmOtpView.564424cf.js",
    "revision": null
  }, {
    "url": "assets/ConfirmOtpView.88e00644.css",
    "revision": null
  }, {
    "url": "assets/CreateEmailView.973eb008.js",
    "revision": null
  }, {
    "url": "assets/CreateEmailView.be01e759.css",
    "revision": null
  }, {
    "url": "assets/CreateNoteView.641f0f92.js",
    "revision": null
  }, {
    "url": "assets/CreateNoteView.6f919239.css",
    "revision": null
  }, {
    "url": "assets/CreateTodoView.1c98f1e2.css",
    "revision": null
  }, {
    "url": "assets/CreateTodoView.bb24ee6c.js",
    "revision": null
  }, {
    "url": "assets/EmailIndexView.cdf332b4.js",
    "revision": null
  }, {
    "url": "assets/ImportantEmailView.8f768f3e.js",
    "revision": null
  }, {
    "url": "assets/index.679e4c1f.js",
    "revision": null
  }, {
    "url": "assets/index.867c1d86.js",
    "revision": null
  }, {
    "url": "assets/index.8c8cf985.css",
    "revision": null
  }, {
    "url": "assets/notes.bb6aebc9.js",
    "revision": null
  }, {
    "url": "assets/NotesIndexView.18d9ebab.js",
    "revision": null
  }, {
    "url": "assets/NotFoundView.278824c4.js",
    "revision": null
  }, {
    "url": "assets/NotFoundView.a763002d.css",
    "revision": null
  }, {
    "url": "assets/NotificationView.6ac85f0e.js",
    "revision": null
  }, {
    "url": "assets/PasswordResetView.27468696.css",
    "revision": null
  }, {
    "url": "assets/PasswordResetView.5ed8542a.js",
    "revision": null
  }, {
    "url": "assets/ProfileView.cd6d107d.js",
    "revision": null
  }, {
    "url": "assets/ProfileView.d7f09cec.css",
    "revision": null
  }, {
    "url": "assets/ProjectsView.6ef22711.js",
    "revision": null
  }, {
    "url": "assets/ProjectsView.973ac42e.css",
    "revision": null
  }, {
    "url": "assets/SettingsView.474e6fb0.css",
    "revision": null
  }, {
    "url": "assets/SettingsView.4e466c19.js",
    "revision": null
  }, {
    "url": "assets/SignupView.822aeb7f.js",
    "revision": null
  }, {
    "url": "assets/SignupView.a952ae31.css",
    "revision": null
  }, {
    "url": "assets/StarredEmailView.f2755a0d.js",
    "revision": null
  }, {
    "url": "assets/style.8b0c9dff.css",
    "revision": null
  }, {
    "url": "assets/style.f36e090d.js",
    "revision": null
  }, {
    "url": "assets/tasks.18199f5e.js",
    "revision": null
  }, {
    "url": "assets/TodoIndexView.7f9524c3.js",
    "revision": null
  }, {
    "url": "assets/twitter.b4c20ffe.js",
    "revision": null
  }, {
    "url": "assets/UpdateNote.765e5277.js",
    "revision": null
  }, {
    "url": "assets/UpdateNote.c006e7c5.css",
    "revision": null
  }, {
    "url": "assets/ViewNote.0151c3d6.js",
    "revision": null
  }, {
    "url": "assets/ViewNote.217e3269.css",
    "revision": null
  }, {
    "url": "assets/web.2a8349e4.js",
    "revision": null
  }, {
    "url": "index.html",
    "revision": "7f0aef031385378a54980a7205780961"
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
