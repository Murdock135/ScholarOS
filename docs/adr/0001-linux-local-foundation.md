# 0001: Linux local foundation with scoped notes and transactional restore

- Status: Accepted
- Date: 2026-09-23
- Decision basis: User implementation request selected Linux, Tauri, React/TypeScript, Rust, and SQLite, authorized routine implementation choices, and required a working foundation; recorded in the [work log](../../logs/2026-09-23-1600-local-foundation.md).
- Related: [Plan](../../plan.md), sections 2, 3, 5.4, 13–15, and 17

## Context

The repository began with product and process documentation only. The first usable slice must prove project isolation, freeform note persistence, hierarchy, and backup/restore without implementing the entire roadmap. The plan leaves future integrations and several interactions unresolved.

## Decision

Use one Tauri 2 desktop window and a single application instance on Linux. React presents context-local sections; a standalone Rust crate owns SQLite access and validation. Keep the core independent of WebKit so domain/persistence tests can run without a desktop environment. No browser-storage fallback is used for authoritative data.

Use normalized SQLite tables with foreign keys, checks, and triggers, initialized through numbered migration `0001_foundation.sql` in a transaction. A typed context identifies an Area or Project; Projects are drafts in this slice and may omit an Area. Area links only accept Areas. Milestones require Projects; Tasks require Milestones. Scope appears explicitly in mutation commands and note-list queries.

Store one body per note, explicit note-context links, append-only body revisions, and per-context/per-section selections. No session or task owns a note. Notes can be blank. UI labels derive from the first body line. Sharing and archiving UI are deferred; their future schema and workflows must preserve these identities and revisions.

Autosave uses a 450 ms debounce, serialized writes, and an expected revision check. Navigation and native close wait for saves. A local webview journal retains pending drafts across interruption, but only a committed database write yields “Saved locally.” Conflicts retain the draft and support a separate recovery copy.

Portable backup is a strict versioned JSON snapshot from a SQLite read transaction. Restore builds and validates an in-memory staging database with the same constraints, validates revision history, writes a durable rollback snapshot, and replaces live rows in a single transaction. This retains the open database connection safely and avoids a filesystem rename across an active WAL. Restore replaces rather than merges. Exports use a temporary file, file/directory sync, and no-clobber publication. Existing backup destinations are rejected. No paths, credentials, attachments, or future entities are implied by format 1.

## Alternatives

- Browser storage for primary data would bypass Rust invariants and complicate native persistence/backup. It is used only as an auxiliary draft journal.
- A JSON blob database would hide relationship constraints; normalized tables make cross-context and hierarchy failures testable at storage boundaries.
- Replacing the live SQLite file requires careful connection/WAL lifecycle management. Validated transactional replacement plus a rollback snapshot is simpler for this bounded schema.
- Scaffolding connectors, session graphs, and an export framework would add unused components before the foundation is proven.

## Consequences

The foundation works offline and has a narrow IPC surface. Backups preserve the whole implemented domain and note revision history. Full revision history and full-body note-list loading trade storage/memory for simplicity; large-workspace pagination, revision retention, and streaming backup are future work. Notes are limited to 10 MB and restore input to 100 MB. No destructive note deletion or project completion is exposed before recovery/archive semantics are implemented. Native runtime behavior still needs verification on a Linux host with the required libraries.

## Validation

Real SQLite tests cover project/section isolation, hierarchy rejection, optimistic concurrency, transaction rollback, invalid restore, full round trips, and restart in separate processes. Editor tests cover edits during a save and recovery journals. Browser workflow tests use a test-only adapter to this same Rust service. See the [work log](../../logs/2026-09-23-1600-local-foundation.md) for executed check results and native-tooling limitations.
