import { readFile, writeFile } from "node:fs/promises";

const files = process.argv.slice(2);

const targets = files.length > 0
  ? files
  : [".env", ".env.example"];

const ENV_PATTERN = /^(\s*(?:export\s+)?)([A-Za-z_][A-Za-z0-9_]*)(\s*=\s*.*)$/;

function getGroup(key) {
  return key.includes("_")
    ? key.split("_", 1)[0].toUpperCase()
    : key.toUpperCase();
}

function parse(content) {
  const lines = content.split(/\r?\n/);

  const entries = [];
  let comments = [];

  for (const line of lines) {
    const trimmed = line.trim();

    if (!trimmed) {
      continue;
    }

    if (trimmed.startsWith("#")) {
      comments.push(line);
      continue;
    }

    const match = line.match(ENV_PATTERN);

    if (!match) {
      entries.push({
        key: null,
        group: null,
        lines: [...comments, line],
      });

      comments = [];
      continue;
    }

    const [, prefix, key, value] = match;

    entries.push({
      key,
      group: getGroup(key),
      lines: [...comments, `${prefix}${key}${value}`],
    });

    comments = [];
  }

  // Preserve trailing comments.
  if (comments.length > 0) {
    entries.push({
      key: null,
      group: null,
      lines: comments,
    });
  }

  return entries;
}

function sortEntries(entries) {
  const variables = entries.filter((entry) => entry.key !== null);
  const other = entries.filter((entry) => entry.key === null);

  variables.sort((a, b) => {
    const group = a.group.localeCompare(b.group);

    if (group !== 0) {
      return group;
    }

    return a.key.localeCompare(b.key);
  });

  return [...variables, ...other];
}

function format(entries) {
  const output = [];
  let previousGroup = null;

  for (const entry of entries) {
    if (
      previousGroup !== null &&
      entry.group !== null &&
      entry.group !== previousGroup
    ) {
      output.push("");
    }

    output.push(...entry.lines);

    if (entry.group !== null) {
      previousGroup = entry.group;
    }
  }

  return `${output.join("\n").trimEnd()}\n`;
}

async function formatFile(file) {
  let content;

  try {
    content = await readFile(file, "utf8");
  } catch (error) {
    if (error.code === "ENOENT") {
      console.warn(`Skipping ${file}: file does not exist`);
      return;
    }

    throw error;
  }

  const entries = parse(content);
  const sorted = sortEntries(entries);
  const formatted = format(sorted);

  if (formatted === content) {
    console.log(`Already formatted: ${file}`);
    return;
  }

  await writeFile(file, formatted);

  console.log(`Formatted ${file}`);
}

for (const file of targets) {
  await formatFile(file);
}