#!/usr/bin/env node
const { copyFileSync, chmodSync, mkdtempSync, mkdirSync, rmSync, writeFileSync } = require("node:fs");
const { tmpdir } = require("node:os");
const { basename, join, resolve } = require("node:path");
const { spawnSync } = require("node:child_process");

const launchers = {
  "ksp-cli": "plugins/israel-shopping/bin/ksp-cli",
  "reddit-cli": "plugins/web-platforms/bin/reddit-cli",
  "youtube-cli": "plugins/web-platforms/bin/youtube-cli",
};

const name = process.argv[2];
if (!Object.hasOwn(launchers, name)) {
  console.error(`Unknown launcher: ${name || "(missing)"}`);
  process.exit(1);
}

const repositoryRoot = resolve(__dirname, "..");
const source = resolve(repositoryRoot, launchers[name]);
const temporary = mkdtempSync(join(tmpdir(), `${name}-launcher-`));

try {
  const binDirectory = join(temporary, "bin");
  mkdirSync(binDirectory);
  const target = join(binDirectory, basename(source));
  copyFileSync(source, target);
  chmodSync(target, 0o755);
  writeFileSync(
    join(temporary, "package.json"),
    `${JSON.stringify(
      {
        name: `${name}-lazy-launcher`,
        version: "0.1.0",
        private: true,
        bin: { [name]: `bin/${basename(target)}` },
      },
      null,
      2,
    )}\n`,
  );

  const [npm, npmArgs] =
    process.platform === "win32"
      ? ["cmd", ["/c", "npm.cmd"]]
      : ["npm", []];
  const result = spawnSync(npm, [...npmArgs, "install", "-g", "--install-links", temporary], {
    stdio: ["pipe", "inherit", "inherit"],
  });
  if (result.error) {
    console.error(result.error.message);
    process.exitCode = 1;
  } else {
    process.exitCode = result.status ?? 1;
  }
} finally {
  rmSync(temporary, { recursive: true, force: true });
}
