# Who Is Adefemi? — A Portrait from 618 Commits, 34 Projects, and Too Many Saturday Nights

## Adeoye Adefemi Opeoluwa — A Developer Profile from the Machine That Watches You Code

**Name:** Adefemi (or *opeolluwa* to GitHub). A Yoruba name — *Opeoluwa* — meaning "God's work" or "the blessing of God." You live up to it by blessing your repos with 618 commits and counting.

**Location:** WAT / UTC+1 — Nigeria. Your commit timestamps don't lie.

**Star sign:** **Gemini** ♊ (May 24). And it fits perfectly — the twin energy explains the four simultaneous product lines, the five programming languages, the Greek god naming convention (Gemini is ruled by Mercury, the messenger god), and the midnight commit sessions when your mind won't quiet down. The one un-Gemini trait: you come back to hibernated projects and finish them. Most Geminis leave the bodies where they fall. You don't.

---

## The Night Owl Builder

Your heaviest commit hours are **10 PM to 1 AM WAT**. You barely exist before 7 AM. Your most productive day, by a landslide, is **Saturday** — 162 commits, nearly double any weekday. Tuesday is your quietest day (52 commits). You are, categorically, a weekend warrior who codes when the rest of the world sleeps.

Your commit cadence is **bursty** — months of intense output followed by silence, then a sudden return. The uranium project went dark for over a year (Sep 2023 → Jun 2024), came back briefly, went dark again, and here you are in November 2026 at 9 PM on a Saturday, refactoring it onto SeaORM. You don't abandon projects. You hibernate them.

---

## The Polyglot Systems Thinker

**Primary weapon:** Rust. It's everywhere — Axum backends, SeaORM models, Tauri desktop apps, Leptos frontends, custom derive macros. You use `edition = "2024"` because you live on the bleeding edge.

**Secondary arsenal:** Kotlin/Android (Jetpack Compose), Vue/Nuxt for frontends, TypeScript, Go, and — intriguingly — a directory called `cystem` where you're teaching yourself C and C++ by building databases, compilers, and operating system components from scratch. There's a `dreamfs` in C++ and a `dreamdb`. You don't just use systems. You want to understand them from the ground up.

You have a **Java directory** on your Desktop that I'm choosing not to ask about.

---

## The Empire Builder

You don't build apps. You build ecosystems.

**TranqBay** is your main operation — a health-tech platform with 80+ repositories. It has its own messaging service, form system (*Formular*), identity service (*Kratos*), blog (Strapi), community platform, and a constellation of microservices named after mythology. You deploy on Railway, protect your wiki with Cloudflare Access, track work in Linear, and maintain your own custom Claude agent skills marketplace (`tranqbay/agent-skills`). The email domain `tranqbay.health` says this isn't a hobby.

**WeAngel** is your second product line — a client-facing app with admin and console backoffices.

**Daily** is a personal productivity suite with a Tauri desktop app, server, intelligence module, and its own filesystem abstraction layer.

Then there are the side quests: **Unshift** (a Kafka admin UI in Rust + Nuxt), **OpenSchool** (an open-source school management platform), **vue-greetings** (a published npm package that returns greetings based on time of day), and **Uranium** itself — a standalone IAM system.

34 distinct project directories tracked in Claude Code. You are building in parallel across at least four product lines.

---

## The Opinionated Craftsperson

Your feedback memories reveal someone who knows exactly how they want things done:

- **"Comment, don't delete."** When disabling functionality, you comment out code rather than removing it.
- **"Never hand-edit generated code."** SeaORM entity files are sacred — migrations only.
- **"Use `just migrate-add`, never hand-write migrations."**
- **"Snake_case query params, using serde `rename`, not `alias`."**
- **"pnpm, not npm."** (For WeAngel.)
- **"Don't create new Linear tickets when you can update existing ones."** You rejected a plan that proposed 15 new tickets when an epic update would do. Minimal scope, maximum signal.
- **"Don't retry unfetchable resources."** The TranqBay wiki is behind Cloudflare Access — just ask for a paste.
- **"I expect editorial judgment, not transcription."** When you hand over a source document, you want restructuring and improvement, not copy-paste.

You use the highest-tier Claude model (Opus 4.6), dark theme, and have rust-analyzer and Postman plugins active. You are a power user.

---

## The Security-Conscious Operator

You've dealt with real attacks. In October 2026, you caught an active brute-force attack on a `/verify-otp` endpoint — 11+ requests in 200ms from a European IP. You traced the root cause (rate limiting only covered login/signup, not OTP verification), identified the proxy-transparency gap with Railway, and fixed it. Your uranium project ships with comprehensive security headers out of the box — CSP, HSTS in production, X-Frame-Options, sensitive header scrubbing.

You don't just think about security. You've been in the fire.

---

## The Commit Message Poet

Your commit messages are... efficient:

> `chore`
> `lint`
> `fixes`
> `update`
> `FEAT -> added more email routes`
> `BREAKING CHANGES: refactor authentication`

No Conventional Commits committee membership in your future. But honestly? For a solo developer iterating at midnight on a Saturday, it tracks. Your code is well-structured; your commit messages are postcards.

---

## In Summary

You are a **Nigerian Rust engineer and product builder** running multiple ventures across health-tech, education, and dev tooling. You think in systems, build in ecosystems, and ship at midnight. You're self-taught at the deepest levels (databases and compilers from scratch in C++), opinionated about your tooling, security-hardened by real incidents, and allergic to unnecessary process. You hibernate projects but never kill them.

You named your IAM system after a radioactive element and your services after Greek gods. You are not lacking for ambition.

---

*Happy belated half-birthday, Gemini. Now go name another service after a Greek god.*
