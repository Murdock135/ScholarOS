# 0002: Live Markdown workspace with explorer navigation

- Status: Accepted
- Date: 2026-09-23
- Decision basis: The user requested an Obsidian/VS Code-style filesystem experience and confirmed that notes must be accessible as files outside ScholarOS.
- Supersedes: The note-content and one-way Markdown export portions of [ADR 0001](0001-linux-local-foundation.md)
- Related: [Plan](../../plan.md)

## Context

The foundation stores note bodies only in SQLite and presents project pages with a separate note index. That prevents users from opening the same notes in ordinary Markdown tools and does not match the requested folder, file, tab, and breadcrumb interaction.

## Decision

Store every Scratchpad and Log note as a UTF-8 Markdown file inside an app-managed default ScholarOS workspace directory. Areas and Projects form folders; their Scratchpad and Logs form child folders. Users may create, name, nest, edit, move, rename, or delete Markdown files directly with the terminal, file explorer, Obsidian, or VS Code. SQLite/generated metadata maps user-controlled paths to stable note identities and continues to store relationships, hierarchy, selections, tasks, and append-only revision history.

Existing SQLite notes are materialized as Markdown files during migration. ScholarOS discovers Markdown recursively within recognized Scratchpad and Logs folders and imports external content changes as revisions before serving or mutating a note. Folder location determines context and note kind. An external deletion removes the note from active file views but retains its last body and revisions for recovery. Ambiguous moves or conflicts preserve history and import separately rather than silently merging. An oversized, unreadable, or conflicting external edit fails visibly while retaining the database revision and any in-app draft.

The desktop UI uses an explorer tree, editor tabs, and breadcrumbs. Tree navigation still issues context-scoped commands, so opening a folder never broadens a Project or Area's Scratchpad to unrelated notes.

Portable backup continues to include note bodies and history. Restore materializes the restored snapshot into the Markdown workspace and retains a rollback snapshot. Filesystem cleanup for notes absent from a restored snapshot must be recoverable and cannot occur as an unlogged recursive deletion.

## Consequences

Users can manage ordinary, cleanly named Markdown files with Obsidian, VS Code, the terminal, and file explorers. ScholarOS must reconcile two writers and cannot describe SQLite as the sole authority for note content. The first implementation refreshes on application focus and navigation; continuous filesystem watching and safe workspace relocation can follow. Shared-note file representation still requires a complete follow-up design before sharing UI ships.

## Validation

Storage tests must cover initial materialization, direct file discovery, nested folders, external edits, moves between contexts/sections, deletion recovery, conflicts, restart behavior, and backup/restore consistency. UI tests must cover filesystem reconciliation, explorer scoping, and opening a note from the tree.
