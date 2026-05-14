# Simple Checksum Verifier

[![Tauri](https://img.shields.io/badge/Tauri-24C8D8?logo=tauri&logoColor=fff)](https://tauri.app/)
[![Svelte](https://img.shields.io/badge/Svelte-%23f1413d.svg?logo=svelte&logoColor=white)](https://svelte.dev/)
[![Tailwind CSS](https://img.shields.io/badge/Tailwind_CSS-%2338B2AC.svg?logo=tailwind-css&logoColor=white)](https://tailwindcss.com/)
[![Latest release](https://img.shields.io/github/v/release/FlamingWater35/simple-checksum-verifier)](https://github.com/FlamingWater35/simple-checksum-verifier/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux-purple)

A high-performance file integrity tool for hashing directories and verifying checksums. It creates "snapshots" of your folders and compares them against the live state to detect bit-rot, accidental edits, deletions, or new files.

## ✨ Features

- **Multi-Algorithm Support:** High-speed hashing using **BLAKE3**, **BLAKE2b**, **SHA-256**, **SHA-1**, or **MD5**.
- **Backup Synchronization:** Link multiple backup locations to a single folder. Verify the integrity of your main source and all backups simultaneously against one snapshot.
- **Verification Modes:**
  - **Quick Mode:** Instantly detects changes by comparing file metadata (size and modification date).
  - **Deep Mode:** Performs a full cryptographic hash check to ensure absolute data integrity and detect silent bit-rot.
- **Performance Options:**
  - **Concurrency Control:** Switch between **Parallel** mode (best for NVMe/SSDs) and **Sequential** mode (best for HDDs).
  - **Adjustable I/O:** Fine-tune read performance with customizable buffer sizes (128KB, 256KB, or 512KB).
- **Visual Verification Tree:** Navigate scan results with clear status indicators:
  - 🟢 **Match**: File is unchanged.
  - 🔴 **Mismatch**: File content differs from the snapshot.
  - 🟡 **Modified**: Metadata (size/date) has changed.
  - 🟠 **Missing**: File was deleted or moved.
  - 🟣 **Untracked**: New file added since the snapshot.
  - 🌹 **Access Denied**: File could not be read due to OS permissions.
- **Modern UI/UX:**
  - Native theme mode support.
  - Memory-efficient snapshot handling using buffered streaming.
  - Automatic update checks via GitHub API.

## 📋 Requirements

### System Requirements

- **Windows:** Windows 10 or 11 (64-bit).
- **Linux:** Ubuntu 22.04+ or similar distribution with `webkit2gtk-4.1` installed.

### Development Requirements

To build from source:

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

- **Frontend:** [Svelte](https://svelte.dev/), [Tailwind CSS](https://tailwindcss.com/), [Bits-UI](https://bits-ui.com/)
- **Backend:** [Rust](https://www.rust-lang.org/), [Tauri](https://tauri.app/)

## 📁 Data Storage Location

Snapshots and settings are stored locally:

- **Windows:** `%LOCALAPPDATA%\SimpleChecksumVerifier\`
- **Linux:** `~/.local/share/SimpleChecksumVerifier/`

## 📜 License

Released under the **MIT License**.
See the [LICENSE](LICENSE) file for full details.
