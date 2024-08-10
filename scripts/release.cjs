const { execSync } = require("child_process");

execSync("pnpm changeset version");

const packageJson = require("../package.json");

execSync(`pnpm tauri-version ${packageJson.version} --no-git`);
