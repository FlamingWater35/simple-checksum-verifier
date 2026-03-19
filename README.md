# Simple Checksum Verifier

An application for hashing files inside selected folders and verifying their checksums easily and conveniently. It creates "snapshots" of your folders and compares them against the live state to detect changes, deletions, or corruption.

## ✨ Features

- **Folder Snapshots:** Select any folder to generate SHA-256 checksums for all contained files.
- **Visual Verification:** Navigate a recursive tree view with color-coded statuses:
  - <span style="color: #22c55e">●</span> **Match**: File is unchanged.
  - <span style="color: #ef4444">●</span> **Mismatch**: File content has been modified.
  - <span style="color: #f97316">●</span> **Missing**: File was deleted or moved.
  - <span style="color: #a855f7">●</span> **Untracked**: New file added since the snapshot.
- **Local Storage:** Saved folder lists are stored locally in `%LOCALAPPDATA%`.
- **Modern UI:** Native dark/light mode support.

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
