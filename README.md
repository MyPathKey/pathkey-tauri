<p align="center">
  <img src="./public/icon.png" alt="PathKey Icon" width="120" style="border-radius:20px;" />
</p>

<h1 align="center">PathKey Desktop</h1>

<p align="center">
  A Tauri-powered desktop app for the <strong>Path Key</strong> numerology engine and console UI.
</p>

---

## ✨ About

PathKey Desktop is a lightweight desktop client built with [Tauri v2](https://tauri.app) and static HTML/CSS/JS.
It loads your frontend directly from `apps/desktop/dist` and packages as a secure, cross-platform binary.

---

## 📂 Project Structure

repo-root/ apps/ desktop/ dist/           # Static frontend (HTML, CSS, JS, partials) public/ icon.png          # App icon (used in builds + README) src-tauri/ tauri.conf.json   # Tauri configuration Cargo.toml        # Rust manifest src/              # Rust backend package.json        # pnpm scripts for tauri dev/build

---

## 🛠️ Development

Ensure you have:

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [pnpm 10.14.0+](https://pnpm.io) (via Corepack recommended)
- Node.js 18 / 20 / 22

### Install deps

```bash
pnpm install

Run in dev mode

pnpm tauri dev

This launches a debug Tauri window that loads from apps/desktop/dist.

Build release binary

pnpm tauri build

Your packaged binaries will be in src-tauri/target/release/.


---

📜 License

This repo uses a dual license setup:

MIT license for upstream Tauri-derived code (LICENSE-MIT)

Proprietary license for PathKey™ additions (LICENSE-PATHKEY)

Attribution notices in NOTICE



---

<p align="center">
  <small>© PathKey — All rights reserved.</small>
</p>
