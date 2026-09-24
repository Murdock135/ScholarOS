# ScholarOS

A local-first academic workspace with a VS Code/Obsidian-style dark workbench. The current vertical slice implements an activity rail, filesystem explorer, breadcrumbs, active-file editor tab, status bar, Areas, draft Projects, context-local Scratchpad and Logs, freeform autosaving notes, Milestones with Tasks, and validated JSON backup/restore.

## Run on Linux

Install a current stable Rust toolchain and Node.js 22.12+ (or a compatible newer release). On Ubuntu 24.04, install the [Tauri Linux prerequisites](https://v2.tauri.app/start/prerequisites/), including the dialog plugin's D-Bus development dependency:

```sh
sudo apt install build-essential pkg-config libwebkit2gtk-4.1-dev libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev libdbus-1-dev
npm ci
npm run desktop -- dev
```

Run these commands from the repository root. `npm run desktop -- build --no-bundle` builds the desktop executable. Linux is the initial target; distribution packages and other operating systems have not been validated. `npm run dev` alone serves the frontend, but production functionality requires the Tauri host; it does not silently substitute browser storage for SQLite.

## Use the workspace

1. Create an Area, then a Project. Choose its Area or leave it as an unassigned draft.
2. Open Scratchpad or Logs and select **New note**. Write immediately; no title, milestone, task, or session is required.
3. Edits autosave after a short pause. **Saved locally** means the Markdown file and SQLite revision metadata were committed. Switching notes, sections, or contexts flushes pending edits first. Each context retains its last section, selected note per section, and cursor.
4. Open a Project's Milestones tab. Add an outcome, then its Tasks. Checking a Task persists its completion timestamp. Areas have their own notes; they never aggregate Project notes.
5. Browse the same live Markdown workspace in the in-app Explorer, or use **Open folder** to view it in your system file manager. You may create Markdown files directly anywhere below an existing Area or Project's Scratchpad or Logs folder, including nested folders. ScholarOS discovers them on navigation, reload, or when the app regains focus. Moving a file between those folders changes its context or kind when identity can be matched unambiguously.
6. Use **Back up** to choose a new JSON filename. Use **Restore** to select a backup, review validated counts, and explicitly replace the workspace. The previous database snapshot and Markdown workspace are retained beside the database for rollback; the success message shows the JSON path. To roll back, restore that file.

A failed save blocks navigation and retains the draft. **Retry save** retries persistence; **Save as recovery copy** saves a conflicted draft into a separate note without overwriting the original. Unsaved text is also journaled in the local webview storage for crash recovery. A forced process kill cannot guarantee recovery of keystrokes the operating system has not flushed. Saved text lives in Markdown; SQLite retains its identity and revision history.

## Local data and backup

The Linux database is in Tauri's application data directory, normally `${XDG_DATA_HOME:-$HOME/.local/share}/org.scholaros.desktop/scholaros.sqlite`. The live files are in the sibling `ScholarOS Workspace/` directory. The exact base directory follows the desktop environment. Rollback database snapshots and workspace folders live beside the database. Do not copy a live SQLite file alone: use the in-app backup, which reconciles tracked Markdown before taking a consistent database snapshot.

Backups contain the complete implemented slice: contexts, note bodies and revision history, file identities, explicit note links, selections and cursor positions, hierarchy, Task completion, and workspace selection. Format 1 and the database schema are validated before replacement starts. Unknown fields/versions, duplicate identities, broken relationships, invalid hierarchy, and inconsistent note history are rejected. Restore retains the previous Markdown folder instead of deleting it. Backup destinations must be new files; existing files are never overwritten. Backup JSON is unencrypted. The current slice has no attachment records or attachment storage; text that names a local file does not back up that file.

## Checks

```sh
npm run build
npm test
cargo test -p scholar-core
cargo clippy -p scholar-core --all-targets -- -D warnings
cargo fmt --all --check
npx playwright install --with-deps chromium
npm run test:ui
cargo check -p scholaros
```

The Rust tests exercise real SQLite files, explicit scopes, hierarchy failures, conflict handling, failed writes, restore validation/rollback, and persistence in separate processes. Editor tests exercise overlapping edits and recoverable failures. Playwright runs the actual React UI against the same Rust service and SQLite through a **test-only** loopback transport. Its database is temporary and each request reopens it. This verifies UI/storage integration but does not test Tauri IPC, WebKit, native dialogs, or native window-close events. The HTTP test transport and Vite proxy are absent from production builds.

## Scope and architecture

- [Product plan](docs/plan/README.md)
- [Foundation architecture decision](docs/adr/0001-linux-local-foundation.md)
- [Implementation work log](logs/2026-09-23-1600-local-foundation.md)

The application code is in `apps/desktop`; the independent Rust domain/storage service and numbered SQL migrations are in `crates/core`. Markdown is the live note body, while SQLite is authoritative for identity, scope, hierarchy, navigation state, and revision history. The desktop has a single window and a single running instance. No network service runs in the desktop.

All Projects in this slice remain drafts. Direct file discovery is limited to Markdown beneath existing Scratchpad and Logs folders; creating filesystem folders does not create typed Areas or Projects. External deletion hides the note while retaining recoverable history, but trash/recovery UI is deferred. Multi-file tab management, in-app rename/move commands, Project activation, reassignment, completion/archive, explicit sharing UI, revision browsing, search, imports, sessions, startup routines, integrations, AI, graphs, PDF tracking, capture, and the interactive milestone bar are deferred. Explicit context-link and revision tables preserve room for future shared notes and archive snapshots; those future behaviors are not claimed as implemented. Milestone acceptance/progress and editing hierarchy names are also outside this initial slice.
