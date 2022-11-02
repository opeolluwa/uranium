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
    "url": "assets/AllEmailView.279d9ae2.js",
    "revision": null
  }, {
    "url": "assets/AllEmailView.5671b89b.css",
    "revision": null
  }, {
    "url": "assets/AllNotesEntry.59e28cb8.css",
    "revision": null
  }, {
    "url": "assets/AllNotesEntry.a0169c08.js",
    "revision": null
  }, {
    "url": "assets/AllTodoViews.5a52f447.css",
    "revision": null
  }, {
    "url": "assets/AllTodoViews.e0fb4aa7.js",
    "revision": null
  }, {
    "url": "assets/AppEmptyState.52e2987e.css",
    "revision": null
  }, {
    "url": "assets/AppEmptyState.5ca869cc.js",
    "revision": null
  }, {
    "url": "assets/AppModal.8343d420.css",
    "revision": null
  }, {
    "url": "assets/AppModal.ee24828f.js",
    "revision": null
  }, {
    "url": "assets/BaseTextarea.0273d02e.js",
    "revision": null
  }, {
    "url": "assets/BaseTextarea.25744ddf.css",
    "revision": null
  }, {
    "url": "assets/ConfirmOtpView.1669263f.js",
    "revision": null
  }, {
    "url": "assets/ConfirmOtpView.88e00644.css",
    "revision": null
  }, {
    "url": "assets/CreateEmailView.6d1077d6.js",
    "revision": null
  }, {
    "url": "assets/CreateEmailView.be01e759.css",
    "revision": null
  }, {
    "url": "assets/CreateNoteView.5d201fe5.js",
    "revision": null
  }, {
    "url": "assets/CreateNoteView.6f919239.css",
    "revision": null
  }, {
    "url": "assets/CreateTodoView.034e2b60.css",
    "revision": null
  }, {
    "url": "assets/CreateTodoView.b2eba222.js",
    "revision": null
  }, {
    "url": "assets/EmailIndexView.726853d0.js",
    "revision": null
  }, {
    "url": "assets/ImportantEmailView.8f768f3e.js",
    "revision": null
  }, {
    "url": "assets/index.1e245a18.js",
    "revision": null
  }, {
    "url": "assets/index.2e27ba4a.css",
    "revision": null
  }, {
    "url": "assets/index.867c1d86.js",
    "revision": null
  }, {
    "url": "assets/notes.1e5caa52.js",
    "revision": null
  }, {
    "url": "assets/NotesIndexView.d47a21de.js",
    "revision": null
  }, {
    "url": "assets/NotFoundView.7ef7b581.js",
    "revision": null
  }, {
    "url": "assets/NotFoundView.a763002d.css",
    "revision": null
  }, {
    "url": "assets/NotificationView.6d26c5a3.js",
    "revision": null
  }, {
    "url": "assets/PasswordResetView.27468696.css",
    "revision": null
  }, {
    "url": "assets/PasswordResetView.f0098990.js",
    "revision": null
  }, {
    "url": "assets/ProfileView.45383fae.js",
    "revision": null
  }, {
    "url": "assets/ProfileView.5197a868.css",
    "revision": null
  }, {
    "url": "assets/ProjectsView.973ac42e.css",
    "revision": null
  }, {
    "url": "assets/ProjectsView.ce77efee.js",
    "revision": null
  }, {
    "url": "assets/SettingsView.dbd549fb.js",
    "revision": null
  }, {
    "url": "assets/SignupView.bdde3149.css",
    "revision": null
  }, {
    "url": "assets/SignupView.cfdb7b98.js",
    "revision": null
  }, {
    "url": "assets/StarredEmailView.f2755a0d.js",
    "revision": null
  }, {
    "url": "assets/style.3349eec2.js",
    "revision": null
  }, {
    "url": "assets/style.8b0c9dff.css",
    "revision": null
  }, {
    "url": "assets/todo.4bfc2f9a.js",
    "revision": null
  }, {
    "url": "assets/TodoIndexView.24618b5e.js",
    "revision": null
  }, {
    "url": "assets/UpdateNote.0ca43064.js",
    "revision": null
  }, {
    "url": "assets/UpdateNote.d1a38d89.css",
    "revision": null
  }, {
    "url": "assets/ViewNote.80672ba2.js",
    "revision": null
  }, {
    "url": "assets/ViewNote.adc0ce13.css",
    "revision": null
  }, {
    "url": "assets/web.e57133bc.js",
    "revision": null
  }, {
    "url": "index.html",
    "revision": "22480929fed2c8864e0fe6ab474511ed"
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
