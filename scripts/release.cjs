const exec = require("@actions/exec");

exec.exec("pnpm changeset version");

const packageJson = require("../package.json");

exec.exec(`pnpm tauri-version ${packageJson.version} --no-git`);
