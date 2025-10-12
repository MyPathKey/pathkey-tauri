// scripts/generate-latest.mjs
import fs from "node:fs";
import path from "node:path";

const REPO = process.env.GITHUB_REPOSITORY;      // "owner/repo"
const TAG  = process.env.RELEASE_TAG ||           // <-- explicit tag from workflow_dispatch
             process.env.GITHUB_REF_NAME ||
             "latest";
const ARTIFACTS_DIR = process.env.ARTIFACTS_DIR || "artifacts";
const NOTES = process.env.RELEASE_NOTES ?? "";
const VERSION_OVERRIDE = process.env.VERSION_OVERRIDE;

function walk(dir) {
  const out = [];
  if (!fs.existsSync(dir)) return out;
  for (const e of fs.readdirSync(dir, { withFileTypes: true })) {
    const p = path.join(dir, e.name);
    e.isDirectory() ? out.push(...walk(p)) : out.push(p);
  }
  return out;
}

function urlFor(fileAbs) {
  const filename = path.basename(fileAbs);
  return `https://github.com/${REPO}/releases/download/${encodeURIComponent(TAG)}/${encodeURIComponent(filename)}`;
}

function maybeSigFor(fileAbs) {
  const sigPath = `${fileAbs}.sig`;
  return fs.existsSync(sigPath) ? fs.readFileSync(sigPath, "utf8").trim() : "";
}

function firstMatch(files, regexes) {
  for (const rx of regexes) {
    const hit = files.find(f => rx.test(f.replaceAll(path.sep, "/")));
    if (hit) return hit;
  }
}

function detectMacArch(filename) {
  const lower = filename.toLowerCase();
  if (lower.includes("arm64") || lower.includes("aarch64")) return "darwin-aarch64";
  return "darwin-x86_64";
}

function computeVersion() {
  if (VERSION_OVERRIDE) return VERSION_OVERRIDE;
  return TAG.startsWith("v") ? TAG.slice(1) : TAG;
}

const files = walk(ARTIFACTS_DIR);

const linuxCandidates = [/\.AppImage$/i, /\.deb$/i, /\.rpm$/i];
const macCandidates   = [/\.dmg$/i];
const winCandidates   = [/-setup\.exe$/i, /\.msi$/i];

const linuxFile = firstMatch(files, linuxCandidates);
const macFile   = firstMatch(files, macCandidates);
const winFile   = firstMatch(files, winCandidates);

const platforms = {};
if (linuxFile) platforms["linux-x86_64"] = { url: urlFor(linuxFile), signature: maybeSigFor(linuxFile) };
if (macFile)   platforms[detectMacArch(path.basename(macFile))] = { url: urlFor(macFile), signature: maybeSigFor(macFile) };
if (winFile)   platforms["windows-x86_64"] = { url: urlFor(winFile), signature: maybeSigFor(winFile) };

if (!Object.keys(platforms).length) {
  console.error("❌ No distributables found in", ARTIFACTS_DIR);
  process.exit(1);
}

const out = {
  version: computeVersion(),
  notes: NOTES,
  pub_date: new Date().toISOString(),
  platforms
};

fs.writeFileSync("latest.json", JSON.stringify(out, null, 2));
console.log("✅ latest.json generated:\n");
console.log(JSON.stringify(out, null, 2));
