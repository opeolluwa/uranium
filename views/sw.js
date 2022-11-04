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
    "url": "assets/AllEmailView.6a397453.css",
    "revision": null
  }, {
    "url": "assets/AllEmailView.6eba6504.js",
    "revision": null
  }, {
    "url": "assets/AllNotesEntry.5c78dd7a.css",
    "revision": null
  }, {
    "url": "assets/AllNotesEntry.f41b9f5f.js",
    "revision": null
  }, {
    "url": "assets/AllTodoViews.3d41ccaa.css",
    "revision": null
  }, {
    "url": "assets/AllTodoViews.fc3a3aec.js",
    "revision": null
  }, {
    "url": "assets/AppEmptyState.52e2987e.css",
    "revision": null
  }, {
    "url": "assets/AppEmptyState.b9dd52af.js",
    "revision": null
  }, {
    "url": "assets/AppModal.8343d420.css",
    "revision": null
  }, {
    "url": "assets/AppModal.c9253f62.js",
    "revision": null
  }, {
    "url": "assets/BaseTextarea.2be742d1.css",
    "revision": null
  }, {
    "url": "assets/BaseTextarea.bcbae674.js",
    "revision": null
  }, {
    "url": "assets/ConfirmOtpView.88e00644.css",
    "revision": null
  }, {
    "url": "assets/ConfirmOtpView.b3ec19ee.js",
    "revision": null
  }, {
    "url": "assets/CreateEmailView.3b0afc6b.js",
    "revision": null
  }, {
    "url": "assets/CreateEmailView.be01e759.css",
    "revision": null
  }, {
    "url": "assets/CreateNoteView.6f919239.css",
    "revision": null
  }, {
    "url": "assets/CreateNoteView.7f2a8738.js",
    "revision": null
  }, {
    "url": "assets/CreateTodoView.7a84b4f2.css",
    "revision": null
  }, {
    "url": "assets/CreateTodoView.c29df576.js",
    "revision": null
  }, {
    "url": "assets/EmailIndexView.13686989.js",
    "revision": null
  }, {
    "url": "assets/ImportantEmailView.8f768f3e.js",
    "revision": null
  }, {
    "url": "assets/index.473591ae.css",
    "revision": null
  }, {
    "url": "assets/index.867c1d86.js",
    "revision": null
  }, {
    "url": "assets/index.b63b8e1c.js",
    "revision": null
  }, {
    "url": "assets/notes.f6ed24a3.js",
    "revision": null
  }, {
    "url": "assets/NotesIndexView.f0a45897.js",
    "revision": null
  }, {
    "url": "assets/NotFoundView.a763002d.css",
    "revision": null
  }, {
    "url": "assets/NotFoundView.de713132.js",
    "revision": null
  }, {
    "url": "assets/NotificationView.f33b865c.js",
    "revision": null
  }, {
    "url": "assets/PasswordResetView.27468696.css",
    "revision": null
  }, {
    "url": "assets/PasswordResetView.df70fbac.js",
    "revision": null
  }, {
    "url": "assets/ProfileView.266e1f52.js",
    "revision": null
  }, {
    "url": "assets/ProfileView.ed781e59.css",
    "revision": null
  }, {
    "url": "assets/ProjectsView.973ac42e.css",
    "revision": null
  }, {
    "url": "assets/ProjectsView.f9f31ae7.js",
    "revision": null
  }, {
    "url": "assets/SettingsView.474e6fb0.css",
    "revision": null
  }, {
    "url": "assets/SettingsView.884b150b.js",
    "revision": null
  }, {
    "url": "assets/SignupView.356e60d2.js",
    "revision": null
  }, {
    "url": "assets/SignupView.bdde3149.css",
    "revision": null
  }, {
    "url": "assets/StarredEmailView.f2755a0d.js",
    "revision": null
  }, {
    "url": "assets/style.8b0c9dff.css",
    "revision": null
  }, {
    "url": "assets/style.b8ec1b3c.js",
    "revision": null
  }, {
    "url": "assets/todo.e363e95c.js",
    "revision": null
  }, {
    "url": "assets/TodoIndexView.305de9cd.js",
    "revision": null
  }, {
    "url": "assets/twitter.b4c20ffe.js",
    "revision": null
  }, {
    "url": "assets/UpdateNote.94b08776.js",
    "revision": null
  }, {
    "url": "assets/UpdateNote.c006e7c5.css",
    "revision": null
  }, {
    "url": "assets/ViewNote.217e3269.css",
    "revision": null
  }, {
    "url": "assets/ViewNote.a50e63ff.js",
    "revision": null
  }, {
    "url": "assets/web.b7834e6a.js",
    "revision": null
  }, {
    "url": "index.html",
    "revision": "20119bb85d3925a4e46910ae1b11a7e9"
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
