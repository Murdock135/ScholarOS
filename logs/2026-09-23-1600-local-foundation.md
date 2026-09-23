# Local foundation implementation

- Date/time: 2026-09-23 16:00 UTC
- Task: Start ScholarOS with the user-authorized Linux/Tauri/React/Rust/SQLite vertical slice; preserve unrelated work and defer integrations and roadmap scaffolding.

## Changes and outcomes

The initial tree contained documentation only and had no unrelated changes. Read repository guidance, the plan, and documentation conventions. Implemented a focused Area/Project workspace, local note list/editor, autosave with revision checks and recoverable drafts, per-context navigation, Milestones/Tasks, and validated transactional JSON restore with rollback snapshots. Added independent core tests, editor tests, and a Playwright workflow using the actual Rust/SQLite service. Added [run and recovery instructions](../README.md), [ADR 0001](../docs/adr/0001-linux-local-foundation.md), and the authorized initial target/scope in the plan.

## Validation

- Passed `npm run build`: strict TypeScript and production Vite build.
- Passed `npm test`: 3 editor tests for serialized autosave, durable failure recovery, and context-isolated conflict journals.
- Passed `cargo test -p scholar-core`: 6 tests, including the subprocess helper; the parent launches separate write/read processes to verify restart persistence. The remaining cases cover scope/hierarchy, consistent backup round trips, malformed backups, failed writes, rollback-write failure, and unsupported schemas.
- Passed `cargo clippy -p scholar-core --all-targets -- -D warnings` and `cargo fmt --all --check`.
- Passed 2 Playwright acceptance workflows against the real Rust/SQLite service: creation, section/project isolation, restored selection, autosave, reload, persisted Task completion, invalid/valid restore, blocked navigation on failure, recovered journals after reload, and conflict recovery copies preserving the other writer’s body. Chromium required `libnspr4`, `libnss3`, and `libasound2t64`, downloaded and extracted under `/tmp/scholar-browser-libs`; the successful command used `LD_LIBRARY_PATH=/tmp/scholar-browser-libs/extracted/usr/lib/x86_64-linux-gnu npm run test:ui`.
- Inspected the generated 1280×820 workspace screenshot. The note index remains beside the freeform editor and metadata stays outside the writing flow.
- Passed `npm audit --audit-level=moderate` (zero vulnerabilities), new documentation link checks, production-bundle exclusion of the test transport, and `git diff --check`.
- Initial checks exposed and fixed a Task checkbox event-lifetime bug, a close-hook unsubscribe typo, a test-runner/Node storage incompatibility, and a missing native icon. All runnable checks above passed after corrections.
- Native `cargo check -p scholaros`, a time-bounded `npm run desktop -- dev`, and `npm run desktop -- build --no-bundle --debug` reached the missing native dependency boundary. The desktop build entry point successfully rebuilt the production frontend before Cargo stopped at missing `dbus-1.pc`. The launch command correctly starts Vite and invokes Cargo; no native window could be tested. Release configuration explicitly enables Tauri’s embedded asset protocol; development disables default features.

## Remaining work

Native Linux dependencies are absent; a desktop compile attempt fails at missing `dbus-1.pc`, and probes also report missing GTK 3/WebKitGTK 4.1 development libraries. `sudo -n true` requires a password. The environment's normal command sandbox also fails before execution due to the `/mnt/wslg/distro` host mount; repository commands were run through approved escalation. Browser runtime prerequisites were extracted to `/tmp` without system installation, and browser checks passed. Native launch, dialogs, and close handling must be verified on a provisioned Linux desktop.
