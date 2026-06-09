# PortKiller for Windows

Discover listening TCP ports and kill blocking processes from a **system tray app** or **CLI**.

Inspired by [productdevbook/port-killer](https://github.com/productdevbook/port-killer) (core features only).

## Features

- Auto-discover listening TCP ports (IPv4 + IPv6)
- Graceful then force process termination
- Search/filter, favorites, auto-refresh
- System tray with show/hide on close
- Scriptable CLI with JSON output
- Settings: autostart, add-to-PATH, refresh interval, start minimized

## Quick start

### Prerequisites

- Windows 10+
- [Rust](https://rustup.rs/)
- [Bun](https://bun.sh/) 1.1+

### Development

```powershell
bun install
bun run tauri dev
```

### CLI (standalone binary)

```powershell
cargo run -p portkiller-cli -- list
cargo run -p portkiller-cli -- kill 3000
cargo run -p portkiller-cli -- list --json
```

### Release build

```powershell
bun run build
bun run tauri build
```

Installer output: `src-tauri/target/release/bundle/`

## CLI reference

```text
portkiller list [--json]
portkiller search <query> [--json] [--limit <n>]
portkiller port <port>... [--json]
portkiller kill <port> [--force]
portkiller kill --pid <pid> [--force]
portkiller favorites list|add <port>|remove <port>
portkiller settings show|set <key> <value>
```

The tray binary (`PortKiller.exe`) also accepts CLI subcommands when invoked from a terminal.

## Settings

Stored at `%APPDATA%/portkiller/config.json`:

- `favorites` — quick-access ports
- `refresh_interval_secs` — tray auto-refresh (default 5)
- `start_minimized` — launch to tray
- `autostart` — start with Windows
- `add_to_path` — add install dir to user PATH

Toggle **Add to PATH** in Settings, then open a **new** terminal to run `portkiller list`.

## Project structure

```text
crates/portkiller-core/   # scanner, killer, settings (no Tauri)
crates/portkiller-cli/    # headless CLI binary
src-tauri/                # tray app + IPC
src/                      # SolidJS UI
```

## Tech stack

- **Rust**: `windows-sys`, `sysinfo`, `wmi`, `clap`
- **Shell**: Tauri 2 + tray
- **UI**: SolidJS, Tailwind 4, TanStack Query, neverthrow, remeda, solid-primitives i18n

## License

MIT
