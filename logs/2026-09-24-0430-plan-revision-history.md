# Plan revision history archive

- Date archived: 2026-09-24
- Source: `plan.md`
- Purpose: Preserve prior verification and revision summaries outside the current product specification.

This file records historical planning work. Current product requirements remain in [the plan](../plan.md), durable architecture decisions remain in [ADRs](../docs/adr/), and implementation outcomes remain in the dated work logs.

## Historical planning verification (original vault assessment)


- ✅ `find .. -name AGENTS.md -print` — located the applicable repository instructions.
- ✅ `find . -maxdepth 2 -path './.git' -prune -o -type f -print | sort` — inspected the top-level vault structure without modifying it.
- ✅ `find . -path ./.git -prune -o -type f -print | sort` — inventoried the vault content and available attachments while excluding `.git/`.
- ✅ `nl -ba Dashboard.md` — examined the existing dashboard, project queries, milestones, task aggregation, and reading-progress block.
- ✅ `nl -ba README.md` — examined the documented `Work`/`Dive` organization and intended vault philosophy.
- ✅ `nl -ba .scripts/make_work_item.py` — examined the current project scaffolding behavior.
- ✅ `rg -n --glob '*.md' '^(---|name:|kind:|type:|status:|# |## |[-*] \[[ xX]\]|```dataview|```tasks|```apb|#milestone)' Dashboard.md README.md Lists Work` — sampled headings, frontmatter, tasks, milestones, and dynamic queries.
- ✅ `git status --short --branch` — confirmed that planning did not modify the working tree.
- ⚠️ Internet research for current third-party platform capabilities was attempted, but the configured web-search service returned `401 Unauthorized`; consequently, this plan does not make a time-sensitive claim that an existing commercial product already supplies the entire workflow.

The checks above describe the original NewSecondBrain planning session, not this repository or the present revision. Vault paths cited throughout are historical references.

## Plan revision — 2026-09-23

Updated this plan from user feedback: freeform research, multiple project contexts, persistent unfinished startup items, user-owned templates, a proposed interactive milestone bar, agent-assisted library organization, and imports from Excel, Zotero, and Mendeley with overlap preservation. Bar semantics, TOML, and agent automation are design recommendations to validate during prototyping. This revision changes documentation only.

Confirmed follow-up after inspecting the Windows vault: preserve project-local Scratchpad and separate Logs sections. Common note storage does not imply a combined global feed. Creation inherits project context; cross-project browsing and sharing are explicit, optional actions. Sessions remain optional. Migration preserves ownership from the existing folder structure.


## Consistency review — project-local research

Reviewed all 20 sections against the project-local Scratchpad/Logs decision. Aligned navigation and capture/search scope, Today resume behavior, task hierarchy exceptions, shared-note archival, canonical source metadata, reading/imported-note routing, AI context, export/backup, entities, migration, tests, and MVP sequencing. Project and Area sections remain the default working surfaces; optional cross-project research features do not broaden those surfaces. This review changes the design document only; product behavior and migration fidelity still require the planned implementation tests.


## Second consistency review — lifecycle and delivery

Resolved remaining conflicts in final-milestone completion, paused versus archived projects, reopening shared notes, live versus closed startup instances, carryover occurrence identity, metadata correction precedence, attachment-specific progress, offline event retries, and MVP dependencies. Phase 3a plus phase 4 defines the library/capture MVP; optional connectors, AI messages, export, graph work, and native-viewer research do not block it. External provider capabilities and user-specific exports remain discovery checks; this pass verifies internal design consistency, not those integrations.

## Initial implementation boundary — 2026-09-23

The implementation request resolved the initial operating-system target to **Linux**, using Tauri, React/TypeScript, Rust, and SQLite. The completed foundation slice includes Areas and draft Projects; scoped SQLite-backed Scratchpad/Logs; freeform autosave and context restoration; Milestones with Tasks; and validated JSON backup/restore. The live Markdown foundation slice now materializes and reconciles real files, coordinates backup/restore, and presents a dark explorer workbench with an activity rail, Area/Project folder tree, nested Markdown paths, breadcrumbs, an active-file tab, and status bar. Multi-file tab management and in-app file rename/move commands remain follow-up work under [ADR 0002](../docs/adr/0002-live-markdown-workspace.md). Project activation/archive, sharing UI, imports, sessions, startup routines, integrations, AI, graphs, PDF tracking, browser capture, and the interactive milestone bar remain subsequent work. This boundary does not claim phase 1 or the overall MVP is complete.

## Live Markdown workspace decision — 2026-09-23

ScholarOS workspaces behave like a file-based editor. Areas and Projects appear as folders, Scratchpad and Logs are child folders, and notes are real Markdown files that can also be opened in Obsidian, VS Code, or another text editor.

Markdown is the live representation of note content. SQLite remains authoritative for identities, explicit context links, hierarchy, selections, tasks, and revision history. ScholarOS reconciles external file changes through stable note identities; it does not treat a missing file as permission to discard saved content. Conflicting in-app and external edits retain a recoverable draft rather than silently choosing one version.

This replaces the earlier one-way-export-only decision for research notes. Export is still appropriate for structured records that do not have a direct file form. The workspace location, file naming contract, migration of existing notes, backup coverage, and reconciliation rules are recorded in ADR 0002. A filesystem-like UI does not weaken project/area note scoping or turn unrelated files into research notes.


## Full-plan consistency review — 2026-09-23

Reviewed all sections after adopting the live Markdown workspace. Aligned the recommendation, work model, browser capture boundary, technical architecture, data entities, delivery phases, migration, tests, performance, security, discovery questions, and MVP. Live research notes now enter in phase 1; phase 7 covers structured export and optional Obsidian enhancements. The initial platform is Linux. The app-managed default workspace and direct discovery of user-created Markdown are resolved phase-1 requirements. Shared-note physical representation remains an explicit design item before sharing UI is implemented; it must preserve one editable body without relying on file copies that can diverge. Rechecked the linked official Zotero, Mendeley, Drive, and YouTube documentation on 2026-09-23; the scoped connector claims remain supported, while Mendeley exports contain reference metadata and do not directly export PDFs.
