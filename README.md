# Simple Checksum Verifier

[![Tauri](https://img.shields.io/badge/Tauri-24C8D8?logo=tauri&logoColor=fff)](https://tauri.app/)
[![Svelte](https://img.shields.io/badge/Svelte-%23f1413d.svg?logo=svelte&logoColor=white)](https://svelte.dev/)
[![Latest release](https://img.shields.io/github/v/release/FlamingWater35/simple-checksum-verifier)](https://github.com/FlamingWater35/simple-checksum-verifier/releases)
[![Build Status](https://img.shields.io/github/actions/workflow/status/FlamingWater35/simple-checksum-verifier/create-draft-release.yml?label=build)](https://github.com/FlamingWater35/simple-checksum-verifier/actions/workflows/create-draft-release.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux-purple)

An application for hashing files inside selected folders and verifying their checksums with high performance. It creates "snapshots" of your folders and compares them against the live state to detect bit-rot, accidental edits, deletions, or new files.

## ✨ Features

- **Multi-Algorithm Support:** Choose between **SHA-256** for compatibility or **BLAKE2b** / **BLAKE3** for fast, multi-threaded hashing.
- **Backups:** Configure multiple backup locations for each added folder. Verification checks the main folder and all backups simultaneously against a single snapshot.
- **Verification Modes:**
  - **Quick Mode:** Instantly detects changes by comparing file metadata (size and modification date).
  - **Deep Mode:** Reads every byte to ensure absolute data integrity and detect silent corruption.
- **High Performance:** Uses parallel data processing, helping SSD/HDD read speeds during large scans.
- **Visual Verification:** Navigate a tree view with a clear status for each file:
  - 🟢 **Match**: File is unchanged.
  - 🔴 **Mismatch**: File content differs from the snapshot (Deep Mode).
  - 🟡 **Modified**: Metadata (size/date) has changed (Quick Mode).
  - 🟠 **Missing**: File was deleted or moved.
  - 🟣 **Untracked**: New file added since the snapshot was created.
- **Management:** Edit folder paths, update snapshots (rehash) without re-adding, and search through large folder lists instantly.
- **Other Features:**
  - Native **Dark** theme support.
  - Automatic update checks.

## 📋 Requirements

### System Requirements

- **Windows:** Windows 10 or 11 (64-bit).
- **Linux:** Ubuntu 22.04+ or similar distribution with `webkit2gtk-4.1` installed.

### Development Requirements

To build this project from source, you need:

1. **Rust:** [Install Rust](https://www.rust-lang.org/tools/install) (latest stable version).
2. **Node.js:** [Install Node.js](https://nodejs.org/) (v20 or newer recommended).
3. **Windows Build Tools:** (Windows only) [C++ Build Tools for Visual Studio](https://visualstudio.microsoft.com/visual-cpp-build-tools/).
4. **Linux Dependencies:**

   ```bash
   sudo apt-get install -y libwebkit2gtk-4.1-dev build-essential curl wget file libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
   ```

## 🚀 Getting Started

1. **Clone the repository:**

   ```bash
   git clone https://github.com/yourusername/simple-checksum-verifier.git
   cd simple-checksum-verifier
   ```

2. **Install dependencies:**

   ```bash
   npm install
   ```

3. **Run in Development mode:**

   ```bash
   npm run tauri dev
   ```

4. **Build for Production:**

   ```bash
   npm run tauri build
   ```

## 🛠️ Tech Stack

- **Frontend:** [Svelte](https://svelte.dev/), [Tailwind CSS](https://tailwindcss.com/)
- **Backend:** [Rust](https://www.rust-lang.org/), [Tauri](https://tauri.app/)

## 📁 Data Storage Location

The application stores your folder list metadata and checksums as JSON files in:

- **Windows:** `%LOCALAPPDATA%\com.flamingwater.simple-checksum-verifier\folder_lists\`
- **Linux:** `~/.local/share/com.flamingwater.simple-checksum-verifier/folder_lists/`

## 📜 License

Released under the **MIT License**.
See the [LICENSE](LICENSE) file for full details.
