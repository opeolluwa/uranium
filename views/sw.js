if(!self.define){let s,e={};const i=(i,l)=>(i=new URL(i+".js",l).href,e[i]||new Promise((e=>{if("document"in self){const s=document.createElement("script");s.src=i,s.onload=e,document.head.appendChild(s)}else s=i,importScripts(i),e()})).then((()=>{let s=e[i];if(!s)throw new Error(`Module ${i} didn’t register its module`);return s})));self.define=(l,n)=>{const r=s||("document"in self?document.currentScript.src:"")||location.href;if(e[r])return;let o={};const t=s=>i(s,r),u={module:{uri:r},exports:o,require:t};e[r]=Promise.all(l.map((s=>u[s]||t(s)))).then((s=>(n(...s),o)))}}define(["./workbox-3ea082d2"],(function(s){"use strict";self.skipWaiting(),s.clientsClaim(),s.precacheAndRoute([{url:"assets/CreateEmailView.447b4ea3.js",revision:null},{url:"assets/CreateEmailView.d97dd341.css",revision:null},{url:"assets/EmailIndexView.bda20bc8.js",revision:null},{url:"assets/EmailView.073fffbe.js",revision:null},{url:"assets/EmailView.8f1e4aaf.css",revision:null},{url:"assets/IconCommunity.9f7f924e.js",revision:null},{url:"assets/index.4a61f78d.css",revision:null},{url:"assets/index.838ec327.js",revision:null},{url:"assets/NotFoundView.6ff6b9d3.css",revision:null},{url:"assets/NotFoundView.7845904f.js",revision:null},{url:"assets/NotificationView.897296b0.js",revision:null},{url:"assets/ProjectsView.7c07ffc0.js",revision:null},{url:"assets/ProjectsView.846b4d05.css",revision:null},{url:"assets/SettingsView.650c3046.js",revision:null},{url:"assets/TodoView.20f69e7b.js",revision:null},{url:"assets/workbox-window.prod.es5.d2780aeb.js",revision:null},{url:"index.html",revision:"dfb75db70d828de4d22854d4a2667107"},{url:"manifest.webmanifest",revision:"a99b6381420cc0440cb6bb2899cfcc67"}],{}),s.cleanupOutdatedCaches(),s.registerRoute(new s.NavigationRoute(s.createHandlerBoundToURL("index.html")))}));
