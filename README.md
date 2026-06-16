# Local Vibe

Launch pre-configured multi-pane terminal workspaces in one click — built for vibe coding on Windows.

Instead of opening multiple CMD windows and arranging them by hand, pick a **template** and get a resizable grid of real shells inside one app window. Each pane can have its own working directory, startup command, shell, and environment variables.

## Features

- **Embedded terminal grid** — 2×2 (or custom) split panes in one window
- **YAML templates** — define layout, panes, commands, and env vars
- **Profiles** — bind templates to project paths with per-pane overrides
- **Template editor** — edit templates as YAML from the UI
- **Windows-first** — auto-detects PowerShell / pwsh / cmd

## Quick start

### Prerequisites

- [Node.js](https://nodejs.org/) 18+
- [Rust](https://rustup.rs/)

### Development

```bash
npm install
npm run tauri:dev
```

### Production build

```bash
npm run tauri:build
```

Installers are produced at `src-tauri/target/release/bundle/` (`.msi` and `-setup.exe`).

The installer will be in `src-tauri/target/release/bundle/`.

## Templates

Templates live in `~/.local-vibe/templates/` (seeded from [`templates/`](templates/) on first run).

Example — **Vibe Coding** (2×2):

```yaml
id: vibe-coding
name: Vibe Coding
layout:
  type: grid
  rows: 2
  cols: 2
panes:
  - id: dev
    title: Dev Server
    cwd: "{{projectRoot}}"
    command: npm run dev
  - id: shell
    title: Shell
    cwd: "{{projectRoot}}"
    command: ""
```

Variables: `{{projectRoot}}`, `{{home}}`

## Profiles

Profiles live in `~/.local-vibe/profiles/`:

```yaml
id: my-app
name: My App
projectRoot: C:/dev/my-app
templateId: vibe-coding
overrides:
  dev:
    command: pnpm dev
```

## Project structure

```
local_vibe/
├── src/                 # React frontend
├── src-tauri/           # Tauri + Rust backend
├── templates/           # Bundled workspace templates
└── profiles/            # Example profiles
```

## License

MIT — see [LICENSE](LICENSE).