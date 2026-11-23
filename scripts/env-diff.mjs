import { readFile } from "node:fs/promises";

const ENV_FILE = ".env";
const ENV_EXAMPLE_FILE = ".env.example";

const ENV_PATTERN =
  /^\s*(?:export\s+)?([A-Za-z_][A-Za-z0-9_]*)\s*=/;

async function readKeys(file) {
  const content = await readFile(file, "utf8");

  return new Set(
    content
      .split(/\r?\n/)
      .map((line) => line.match(ENV_PATTERN)?.[1])
      .filter(Boolean),
  );
}

const envKeys = await readKeys(ENV_FILE);
const exampleKeys = await readKeys(ENV_EXAMPLE_FILE);

const onlyInEnv = [...envKeys]
  .filter((key) => !exampleKeys.has(key))
  .sort();

const onlyInExample = [...exampleKeys]
  .filter((key) => !envKeys.has(key))
  .sort();

if (onlyInEnv.length > 0) {
  console.log(".env > .env.example");

  for (const key of onlyInEnv) {
    console.log(key);
  }

  console.log();
}

if (onlyInExample.length > 0) {
  console.log(".env.example > .env");

  for (const key of onlyInExample) {
    console.log(key);
  }

  console.log();
}

if (onlyInEnv.length === 0 && onlyInExample.length === 0) {
  console.log("No differences.");
}