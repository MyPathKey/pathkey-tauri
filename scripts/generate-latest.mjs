#!/usr/bin/env node
/**
 * Generate a Tauri-compatible latest.json manifest (no signing)
 *
 * Expected env vars:
 *   GITHUB_REPOSITORY   e.g. "owner/repo"
 *   GITHUB_REF_NAME     e.g. "v1.0.0"
 *   ARTIFACTS_DIR       (optional) defaults to "artifacts"
 *   RELEASE_NOTES       (optional) release notes or changelog text
 *
 * Output:
 *   ./latest.json
 */

import { readdirSync, writeFileSync, existsSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";

const ARTIFACTS_DIR = resolve(process.env.ARTIFACTS_DIR || "artifacts");
const REPO = process.env.GITHUB_REPOSITORY || "yourname/yourrepo";
const TAG = process.env.GITHUB_REF_NAME || "v0.0.0";
const VERSION = TAG.replace(/^v/, "");
const NOTES = process.env.RELEASE_NOTES || "";
const PUB_DATE = new Date().toISOString();

if (!existsSync(ARTIFACTS_DIR)) {
  console.error(`❌ Artifacts directory not found: ${ARTIFACTS_DIR}`);
  process.exit(1);
}

function assetUrl(filename) {
  return `https://github.com/${REPO}/releases/download/${TAG}/${encodeURIComponent(filename)}`;
}

function detectPlatform(filename) {
  const f = filename.toLowerCase();
  if (f.endsWith(".dmg")) return "darwin-x86_64";
  if (f.endsWith(".msi") || f.endsWith(".exe")) return "windows-x86_64";
  if (f.endsWith(".appimage") || f.endsWith(".deb") || f.endsWith(".rpm")) return "linux-x86_64";
  return null;
}

// collect files recursively
function getAllFiles(dir) {
  const files = [];
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    const p = join(dir, entry.name);
    if (entry.isDirectory()) files.push(...getAllFiles(p));
    else files.push(p);
  }
  return files;
}

const files = getAllFiles(ARTIFACTS_DIR);
const platforms = {};

for (const file of files) {
  const filename = file.split(/[\\/]/).pop();
  const platform = detectPlatform(filename);
  if (!platform) continue;
  platforms[platform] = { url: assetUrl(filename), signature: "" };
}

const latest = {
  version: VERSION,
  notes: NOTES,
  pub_date: PUB_DATE,
  platforms,
};

const outPath = resolve("latest.json");
writeFileSync(outPath, JSON.stringify(latest, null, 2));
console.log(`✅ latest.json generated:\n\n${readFileSync(outPath, "utf8")}`);
