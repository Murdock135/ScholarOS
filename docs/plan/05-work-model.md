# 5. Work model and lower-cognitive-load structure

## 5.1 Work screen

Offer four views over the same underlying records:

- **Focus**: only active projects and their current milestones.
- **Board**: projects by status.
- **Timeline**: milestone dates and dependencies.
- **Archive**: completed and cancelled projects. Paused projects remain editable and accessible through Board/status filters; pausing does not archive their notes.

Default to Focus. Draft projects are reachable through Board/status filters. Here, editable contexts include draft, active, and paused projects; Areas follow the same explicit archive rule. Cancelling a project freezes its context like completion; unfinished tasks require an explicit disposition when completing or cancelling a project. Pausing retains them unchanged.

## 5.2 Project screen

A project page should contain:

```text
Project title
Outcome / definition of done
Status and target date
Area
Interactive progress bar with milestone markers
Current milestones
Optional next action

Milestones
Scratchpad — only this project’s working notes
Logs — only this project’s research activity notes
Optional session history
Related library resources
Decisions and open questions
```

Present milestones with their nested tasks and related library resources inside the project workspace. Scratchpad and Logs remain distinct project-local sections, each with its own compact note index and editor. Optional session history links back to those notes. Reading history stays with resources or an explicitly opened project reading view; it does not fill the Scratchpad or research Logs automatically.

Decisions and questions may be written freely in notes; do not introduce mandatory structured forms or separate entities for them in the MVP. Notes can change between Scratchpad and Logs without copying their body; this kind change moves a shared note to the corresponding section in all linked editable contexts; archived contexts retain their saved kind and revision. Show those destinations before applying the change.

## 5.3 Interactive project progress — proposed design

The bar is a useful overview and planning control, provided placement and completion have distinct meanings. Prototype this before finalizing its behavior.

- Users insert milestone markers and drag them along a 0–100% project track. Their positions express planned stages, not dates or completion claims.
- The fill shows calculated completion. Milestone placement does not change the fill or check off tasks.
- Calculate completion as `100 × sum(weight × milestone_completion) / sum(weight)`. Start with equal milestone weights, with optional explicit weights. Marker positions do not implicitly set weights.
- An explicitly completed milestone contributes 1. Otherwise its task fraction supplies progress; a milestone with no tasks contributes 0 until explicitly completed. Checking every task can fill its contribution, while milestone acceptance remains an explicit action.
- Completing a milestone with unfinished tasks leaves those tasks visible; do not silently complete them. Reopening a task does not undo explicit milestone acceptance. Reopening the milestone removes that override and recalculates its contribution from tasks, so progress can decrease. Use positive weights, omit cancelled milestones, and show “Not planned” when there are no included milestones.
- A separately labeled optional draggable “My estimate” marker lets the user express where they believe the project stands. It never overwrites calculated completion. Hide it by default to keep the interface simple.
- Show milestone states independently, since work can finish out of order. Adding/removing scope can change the percentage; explain the change and offer undo.
- Keep project status (active, paused, completed) separate from percentage. Reaching 100% does not automatically close the project.
- Support keyboard movement and numeric placement alongside mouse dragging; use labels and symbols, not color alone.

Research is not always linear. Keep the milestone list available beside the bar, and present percentage as an estimate of tracked work rather than an objective measure of scientific achievement.

## 5.4 Live Markdown workspace and portable export

The live workspace preserves project/area context and the Scratchpad/Logs distinction:

```text
ScholarOS Workspace/
├── Areas/<area-name>--<stable-id>/
│   ├── Scratchpad/<user-chosen-name>.md
│   ├── Logs/<user-chosen-name>.md
│   └── Projects/<project-name>--<stable-id>/...
├── Projects/<unassigned-project>--<stable-id>/...
├── Inbox/<user-chosen-name>.md
└── .scholaros/                    # generated identity/index data; no secrets
```

Files are UTF-8 Markdown and remain freeform; frontmatter is optional user content, not required system metadata. Filenames and nested folders inside Scratchpad and Logs are user-controlled. Stable note IDs and path mappings live in SQLite/generated metadata rather than being forced into filenames. The initial workspace uses an app-managed default location with an “Open workspace folder” action; safe relocation can follow later. ScholarOS discovers Markdown created from the terminal or file explorer, infers its Project/Area and note kind from the containing Scratchpad/Logs tree, records external changes as revisions, and updates search indexes. “Saved” means both the Markdown file and structured revision metadata were durably committed. If those writes cannot be completed consistently, retain the in-app draft and report recovery actions.

An external rename or move preserves identity when ScholarOS can match it unambiguously; moving between recognized Scratchpad/Logs trees changes the note’s context or kind. An externally deleted file leaves the active view while its last body and revisions remain recoverable in SQLite. Ambiguous rename-plus-edit operations retain the missing history and import the new file separately rather than risking a false merge. Ignore transient/non-Markdown files and prevent symlink/path traversal outside the managed workspace.

The first live-file slice covers notes with one editable context. Shared notes must still have one editable body: do not create independent Markdown copies in every linked folder. Before sharing UI ships, resolve its physical representation in a proposal and test it with editors that save by atomic replacement. The application explorer may present context-scoped aliases regardless of the physical representation. Archived contexts retain revision snapshots rather than writable file copies.

Generated indexes are optional navigation aids and remain separate from authoritative note files. Structured Markdown/YAML export can later add project overviews, library indexes, daily records, wikilinks, and archive snapshots without changing the live-note contract.

JSON backup/restore preserves note bodies, revisions, file identities, kind, project/area/milestone/resource links, session links, context state, and import provenance. Preserve local attachments in a backup bundle or provide an explicit missing/external attachment report; JSON references alone are not an attachment backup. Restore into staging, validate database relationships and destination paths, preview conflicts/missing files, retain a rollback snapshot of database and live files, then activate both together. Never remove unrelated files merely because they are absent from a restored snapshot.

The backup contract expands with delivered features: hierarchy, notes/revisions, sessions, startup templates and occurrences, resources and reading events, import mappings, and schema version. Device paths and credentials remain installation-local. Pause ingestion during restore, require account reauthorization as needed, reconcile source cursors before resuming, and rebuild generated indexes. Never export OAuth or local capture tokens; rotate the capture token and repair extension pairing on another installation.

---

[← Previous](04-daily-startup.md) · [Plan index](README.md) · [Next →](06-library.md)
