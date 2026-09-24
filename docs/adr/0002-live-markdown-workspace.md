# 0002: Live Markdown workspace with explorer navigation

- Status: Accepted
- Date: 2026-09-23
- Decision basis: The user requested an Obsidian/VS Code-style filesystem experience and confirmed that notes must be accessible as files outside ScholarOS.
- Supersedes: The note-content and one-way Markdown export portions of [ADR 0001](0001-linux-local-foundation.md)
- Related: [Plan](../../plan.md)

## Context

The foundation stores note bodies only in SQLite and presents project pages with a separate note index. That prevents users from opening the same notes in ordinary Markdown tools and does not match the requested folder, file, tab, and breadcrumb interaction.

## Decision

Store every Scratchpad and Log note as a UTF-8 Markdown file inside a ScholarOS workspace directory. Areas and Projects form folders; their Scratchpad and Logs form child folders. File names include a stable note identifier so display names can change without losing identity. SQLite continues to store relationships, hierarchy, selections, tasks, and append-only revision history.

Existing SQLite notes are materialized as Markdown files during migration. ScholarOS imports external file changes as revisions before serving or mutating a note. A missing file is recreated from the latest saved revision; deletion and trash require an explicit future workflow. An oversized, unreadable, or conflicting external edit must fail visibly while retaining both the database revision and any in-app draft.

The desktop UI uses an explorer tree, editor tabs, and breadcrumbs. Tree navigation still issues context-scoped commands, so opening a folder never broadens a Project or Area's Scratchpad to unrelated notes.

Portable backup continues to include note bodies and history. Restore materializes the restored snapshot into the Markdown workspace and retains a rollback snapshot. Filesystem cleanup for notes absent from a restored snapshot must be recoverable and cannot occur as an unlogged recursive deletion.

## Consequences

Users can edit notes with Obsidian, VS Code, and other Markdown tools. ScholarOS must now reconcile two writers and cannot describe SQLite as the sole authority for note content. Stable IDs in names are less visually minimal, but avoid ambiguous matching after external renames. Filesystem watching, explicit rename/delete, shared-note file representation, and selectable workspace locations require complete follow-up slices; until then synchronization occurs on application refresh and navigation.

## Validation

Storage tests must cover initial materialization, external edits becoming revisions, missing-file recovery, conflicts, restart behavior, and backup/restore consistency. UI tests must cover explorer scoping and opening a note from the tree.
