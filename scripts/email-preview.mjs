import { fileURLToPath } from "node:url";
import { createServer } from "node:http";
import { readdirSync } from "node:fs";
import nunjucks from "nunjucks";
import { dirname, join } from "node:path";

const scriptDir = dirname(fileURLToPath(import.meta.url));
const templatesDir = join(scriptDir, "..", "templates");

nunjucks.configure(templatesDir, { autoescape: true });

const PORT = Number(process.env.PORT) || 5858;

const DASHBOARD_LINK = "http://example.com";

const SAMPLE_DATA = {
  "password_reset.html": {
    reset_link: "https://www.example.com/set-password?token=sample-token-123",
  },
  "otp_code.html": { code: "654321" },
};

const DEFAULT_SAMPLE = {
  name: "Paula",
  dashboard_link: DASHBOARD_LINK,
  reset_link: `${DASHBOARD_LINK}reset-password?token=sample-token-123`,
  code: "123456",
};

// account_confirmation.html uses Askama-specific syntax ({% match %}, {% when Some %})
// that Nunjucks cannot render, so it is excluded from the preview index.
const EXCLUDED = new Set(["base.html", "biller.html", "account_confirmation.html"]);

function baseName(file) {
  return file.replace(/\.html$/, "");
}

function slugify(file) {
  return baseName(file).replace(/_/g, "-");
}

function labelFor(file) {
  return baseName(file).replace(/_/g, " ").replace(/\b\w/g, (c) => c.toUpperCase());
}

function scanTemplates() {
  return readdirSync(templatesDir)
    .filter((file) => file.endsWith(".html") && !EXCLUDED.has(file) && !file.startsWith("invoice") && !file.startsWith("sample-"))
    .sort()
    .map((file) => ({ route: slugify(file), file, label: labelFor(file) }));
}

const TEMPLATES = scanTemplates();

function readIndexHtml() {
  const links = TEMPLATES.map(
    ({ route, label }) =>
      `<li style="margin-bottom:10px"><a href="/${route}" style="color:#00b87a;font-weight:600">${label}</a> <span style="color:#8a9490">— /${route}</span></li>`,
  ).join("\n    ");
  return `<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>Email template preview</title>
    <style>
      body {
        font-family: "Poppins", Arial, sans-serif;
        background: #ffffff;
        color: #0d1220;
        margin: 0;
        padding: 40px 24px;
      }
      h1 { font-size: 22px; margin: 0 0 4px; letter-spacing: -0.3px; }
      p.sub { color: #8a9490; font-size: 14px; margin: 0 0 28px; }
      ul { list-style: none; padding: 0; margin: 0 0 32px; }
      .scan {
        display: inline-block;
        background: #00b87a;
        color: #fff;
        text-decoration: none;
        font-weight: 600;
        font-size: 14px;
        padding: 12px 24px;
        border-radius: 999px;
      }
    </style>
  </head>
  <body>
    <h1>Email template preview</h1>
    <p class="sub">Rendered live from templates using nunjucks (askama-compatible). Auto-indexed from the templates directory.</p>
    <ul>
      ${links}
    </ul>
    <a class="scan" href="/all">Scan all templates</a>
  </body>
</html>`;
}

function readAllHtml() {
  const frames = TEMPLATES.map(
    ({ route, label }) =>
      `<div style="margin-bottom:24px;border:1px solid #e3e9e6;border-radius:12px;overflow:hidden;background:#fff">
        <div style="padding:10px 16px;background:#f7f9f8;font-size:12px;color:#8a9490;font-weight:600">${label} — /${route}</div>
        <iframe src="/${route}" style="width:100%;height:760px;border:0"></iframe>
      </div>`,
  ).join("\n    ");
  return `<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <title>All email templates</title>
    <style>
      body { background: #eef2f0; margin: 0; padding: 24px; }
    </style>
  </head>
  <body>
    ${frames}
  </body>
</html>`;
}

function buildContext(templateFile, url) {
  const overrides = {
    name: url.searchParams.get("name"),
    dashboard_link: url.searchParams.get("dashboardLink"),
    reset_link: url.searchParams.get("resetLink"),
    code: url.searchParams.get("code"),
  };
  const ctx = { ...DEFAULT_SAMPLE, ...(SAMPLE_DATA[templateFile] ?? {}) };
  for (const [key, value] of Object.entries(overrides)) {
    if (value !== null) ctx[key] = value;
  }
  return ctx;
}

const server = createServer((req, res) => {
  const url = new URL(req.url, `http://localhost:${PORT}`);
  const pathname = url.pathname;

  if (pathname === "/favicon.ico") {
    res.writeHead(204);
    res.end();
    return;
  }

  if (pathname === "/") {
    res.writeHead(200, { "Content-Type": "text/html; charset=utf-8" });
    res.end(readIndexHtml());
    return;
  }

  if (pathname === "/all") {
    res.writeHead(200, { "Content-Type": "text/html; charset=utf-8" });
    res.end(readAllHtml());
    return;
  }

  const template = TEMPLATES.find(({ route }) => route === pathname.slice(1));

  if (template === undefined) {
    res.writeHead(404, { "Content-Type": "text/plain; charset=utf-8" });
    res.end("Not found");
    return;
  }

  const html = nunjucks.render(template.file, buildContext(template.file, url));

  res.writeHead(200, { "Content-Type": "text/html; charset=utf-8" });
  res.end(html);
});

server.listen(PORT, () => {
  console.log(`Email template preview running at http://localhost:${PORT}`);
  console.log(`Templates directory: ${templatesDir}`);
  console.log(`Press Ctrl+C to stop.`);
});
