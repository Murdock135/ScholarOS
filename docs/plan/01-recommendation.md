# 1. Recommendation

Build this as a **local-first academic-work operating system**, rather than attempting to make the entire experience an Obsidian vault.

The recommended product would consist of:

1. **A Tauri desktop application**
   - React/TypeScript interface.
   - Rust host process for filesystem access, native PDF-window detection, background synchronization, and secure credential storage.
   - Targets Linux first; Windows and macOS remain later portability work.

2. **A local SQLite database**
   - The authoritative structured store for projects, milestones, tasks, resources, reading events, note identities, context links, and revision history.
   - SQLite FTS for structured records and an index of reconciled Markdown content.
   - Optional embeddings for semantic clustering and recommendations.

3. **A browser extension**
   - Captures books, papers, articles, videos, and arbitrary webpages.
   - Reports reading progress to the desktop application.
   - Communicates with the local application through a small authenticated loopback API or browser native messaging.

4. **A live Markdown workspace**
   - Stores Scratchpad and Log notes as real Markdown files in Area and Project folders.
   - Allows the same notes to be opened in Obsidian, VS Code, and other text editors.
   - Uses SQLite for structured relationships, navigation state, and revision history while reconciling external file edits.

This preserves the parts of NewSecondBrain that already work—projects, disciplines, milestones, tasks, reading, and dashboards—while generating the workspace structure automatically instead of requiring manual folder and index maintenance. The existing vault explicitly divides `Work/` into projects and disciplines and gives each item five or more subordinate files, which is understandable but creates repeated navigation and maintenance overhead. README.md:3-18 The current scaffolding script reinforces that repetition by creating a `Dive` directory and five template files for every work item. .scripts/make_work_item.py:4-26

Do not begin by building an Obsidian plugin. Obsidian and VS Code can edit the live Markdown workspace, while ScholarOS supplies scoped navigation, structured work, reconciliation, browser capture, background jobs, PDF tracking, and later graph features.

---

[Plan index](README.md) · [Next →](02-product-philosophy.md)
