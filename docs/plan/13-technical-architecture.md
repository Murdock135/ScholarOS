# 13. Proposed technical architecture

```text
┌──────────────────────────────────────┐
│ Tauri desktop application            │
│ React + TypeScript explorer/editor   │
├──────────────────────────────────────┤
│ Rust application services            │
│ - scoped commands and reconciliation │
│ - local API/native messaging         │
│ - filesystem and OS integrations     │
│ - backup/restore and credentials      │
├───────────────────┬──────────────────┤
│ Live Markdown     │ SQLite           │
│ - note bodies     │ - hierarchy      │
│ - user filenames  │ - links/state    │
│ - user formatting │ - revisions      │
│                   │ - events/indexes │
├───────────────────┴──────────────────┤
│ Background workers                   │
│ - file watching/reconciliation       │
│ - connectors and metadata extraction │
│ - graph/index processing             │
└──────────────────────────────────────┘
          ▲                    ▲
          │                    │
 Browser extension      External editors
```

A storage service coordinates file and database commits, detects external changes, and exposes only context-scoped commands to the UI. SQLite cannot silently overwrite a newer Markdown revision, and a filesystem event cannot bypass note ownership rules. Generated search data and indexes can be rebuilt; live Markdown and committed structured records require backup.

## 13.1 Repository layout

The implemented repository uses:

```text
apps/
└── desktop/          # React/Tauri desktop application

crates/
└── core/             # domain rules, SQLite, backup/restore, file reconciliation

docs/
├── adr/
└── proposals/

logs/                 # implementation work summaries
```

Add `apps/extension`, connector crates, or shared packages only when a delivered vertical slice needs them. Application source remains separate from the user's live workspace.

---

[← Previous](12-personalized-message.md) · [Plan index](README.md) · [Next →](14-data-entities.md)
