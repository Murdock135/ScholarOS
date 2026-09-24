# 15. Delivery roadmap

## Phase 0 — Product discovery and prototypes

**Goal:** remove ambiguity before the affected later slices; the Linux foundation is already implemented.

Deliverables:

- Validate the already selected initial Linux target and record prerequisites; treat Windows and macOS as later portability work.
- Review the referenced mockups/attachments.
- Inspect representative Excel rows, Zotero records, and Mendeley exports, including overlapping papers and notes.
- Inventory the Google Drive library.
- Select five representative PDFs, webpages, videos, and books.
- Prototype project/area Scratchpad and Logs, switching between two projects, shared notes, and unassigned capture.
- Prototype the Today page with scoped resume destinations and persistent unchecked items.
- Prototype the milestone bar and test placement versus completion behavior.
- Prototype list and cluster library views.
- Test metadata extraction against representative sources.
- Test foreground-window detection with the user’s actual PDF viewers.
- Produce a privacy and threat model.

Exit criterion:

- Project-local work, scratch/log capture, startup carryover, and manual reading can be demonstrated with clickable prototypes and test data. Cluster-view and native-viewer experiments are separate later-feature investigations and must not block the work-core implementation.

## Phase 1 — Structured work core

**Status:** partially implemented. The SQLite foundation is complete; live Markdown, explorer navigation, search, capture, sessions, and migration remain.

**Goal:** organize work through typed objects while preserving freeform research notes.

Deliverables:

- SQLite schema and migrations.
- Area → Project → Milestone → Task hierarchy.
- Work Focus view.
- Project view.
- A live Markdown workspace plus an explorer-style tree, editor tabs, breadcrumbs, Project/Area Scratchpad and Logs folders, automatic note indexes, and blank-note capture.
- Optional cross-project browsing and an unassigned capture inbox, separate from project working views.
- Session-to-note links supporting several notes per session and continued notes across sessions.
- Global capture dialog.
- Search.
- Coordinated JSON/file backup and restore with rollback.
- Migration/materialization of existing SQLite note bodies into live Markdown files.
- Read-only importer and dry-run migration report for the existing NewSecondBrain structure. Import work and notes first; retain unresolved reading/reference entries in import staging until phase 3 can normalize them.

Exit criterion:

- The user can manage a real project for a week, including editing the same Markdown note in ScholarOS and an external editor. Switching between two test projects never shows unrelated notes; conflicts retain both drafts, and backup/restore preserves files, revisions, and links.

## Phase 2 — Today dashboard

Deliverables:

- User-owned TOML startup templates and an in-app editor.
- Daily snapshots, archive, and deduplicated carryover of unchecked items.
- Continue-research card.
- Active milestone cards.
- Reading-now card and recent reading feed backed initially by minimal manual Resource, ReadingProgress, ReadingProgressEvent, and ReadingLogEntry records; phase 3 adds full ingestion.
- End-of-day close workflow.
- Rule-based positive messages.
- Optional AI message generation after the offline dashboard is usable; it does not gate MVP delivery.

Exit criterion:

- Startup and shutdown work reliably offline, and no daily data is lost if AI is unavailable.

## Phase 3 — Library foundation

Deliverables:

- Unified resource model.
- Manual import.
- Local PDF folder connector.
- Google Drive connector after the MVP import set is usable.
- Zotero connector plus Excel and Mendeley export importers that preserve source notes.
- YouTube playlist connector as a subsequent library increment.
- Metadata review queue.
- Deduplication.
- Incremental synchronization.
- List view with virtual scrolling.
- Full-text and metadata search.

Exit criterion:

- Phase 3a supplies the MVP set: local PDFs, Zotero, Excel, and supported Mendeley exports. Phase 3b adds Drive and YouTube playlists after the MVP; phase 4 depends only on 3a. Incremental connectors import additions without a full recrawl; repeated spreadsheet/export imports are idempotent. Matching records from Excel, Zotero, and Mendeley produce one canonical entry with attributed source notes; ambiguous matches enter review.

## Phase 4 — Reading capture

Deliverables:

- Browser extension.
- Metadata extraction.
- Gradient progress control.
- Reading-log capture.
- Offline queue.
- Dashboard updates.
- Coordinated backup remains available from phase 1; structured Markdown/YAML export does not gate reading capture.

Exit criterion:

- A user can log progress from representative webpages, YouTube, and browser PDFs in under ten seconds.

## Phase 5 — Native PDF companion

Deliverables:

- Built-in PDF reader.
- Foreground-window detection.
- Okular adapter.
- Acrobat adapter.
- Global shortcut and overlay.
- File resolution and confirmation flow.
- Current-page extraction where supported.
- Manual fallback where unsupported.

Exit criterion:

- Supported-reader progress capture works reliably, and unsupported readers fail transparently rather than recording incorrect progress.

## Phase 6 — Clusters and learning paths

Deliverables:

- Topic hierarchy.
- Topic, level, and prerequisite editing.
- Rule-based initial classification.
- Optional AI classification with confidence.
- WebGL cluster view.
- Topic-learning-path generator.
- Project evidence graph.

Exit criterion:

- A user can select a topic and obtain a credible ordered path consisting exclusively of material already in their library.

## Phase 7 — Structured export and Obsidian enhancements

Deliverables:

- Stable Markdown/YAML export for structured records beyond live research notes.
- Project/area Scratchpad and Logs indexes linking live note identities, shared-note targets, and archived revisions after the shared-file representation is resolved.
- Wikilinks between exported projects, resources, scratch notes, and research/reading logs.
- Link validation.
- Incremental structured export without rewriting unchanged live notes.
- Validate the phase 1 migration report against exported links; migration itself is not deferred to this phase.
- Optional Obsidian dashboard generated from the structured data.

Exit criterion:

- Exported data is readable without the application and introduces no broken generated links.

---

[← Previous](14-data-entities.md) · [Plan index](README.md) · [Next →](16-migration.md)
