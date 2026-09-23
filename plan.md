# Product and implementation plan

## 1. Recommendation

Build this as a **local-first academic-work operating system**, rather than attempting to make the entire experience an Obsidian vault.

The recommended product would consist of:

1. **A Tauri desktop application**
   - React/TypeScript interface.
   - Rust host process for filesystem access, native PDF-window detection, background synchronization, and secure credential storage.
   - Runs on Linux, Windows, and eventually macOS.

2. **A local SQLite database**
   - The authoritative structured store for projects, milestones, tasks, resources, reading progress, and logs.
   - SQLite FTS for search.
   - Optional embeddings for semantic clustering and recommendations.

3. **A browser extension**
   - Captures books, papers, articles, videos, and arbitrary webpages.
   - Reports reading progress to the desktop application.
   - Communicates with the local application through a small authenticated loopback API or browser native messaging.

4. **An optional Obsidian-compatible export layer**
   - Generates readable Markdown and YAML from the structured database.
   - Allows the user to browse or back up their information in Obsidian without forcing the application itself to model everything as notes.
   - When delivered in phase 7, the export is one-way; bidirectional synchronization should not be attempted until the data model is stable.

This preserves the parts of NewSecondBrain that already work—projects, disciplines, milestones, tasks, reading, and dashboards—without retaining its folder and file-management burden. The existing vault explicitly divides `Work/` into projects and disciplines and gives each item five or more subordinate files, which is understandable but creates repeated navigation and maintenance overhead. README.md:3-18 The current scaffolding script reinforces that repetition by creating a `Dive` directory and five template files for every work item. .scripts/make_work_item.py:4-26

I would **not** begin by building an Obsidian plugin. Obsidian could remain a useful export/viewer, but the requested live integrations, browser capture, background jobs, native PDF tracking, and lazy-loaded visual graph are better served by a dedicated application.

---

# 2. Product philosophy

## 2.1 Structured work, freeform research

Projects, milestones, tasks, resources, and activity links remain typed records. Research logs open as blank notes: users choose their own structure, formatting, headings, and optional templates. Do not require a summary, result, activity category, or next action.

Store the note body with lightweight system metadata (ID, timestamps, optional session and resource links). Research sessions and notes can link to zero, one, or multiple projects and milestones through join tables. Linking a milestone must retain its owning project. A note can be written without starting a timer, and a session can be resumed without completing a form.

### Project-local Scratchpad and Logs

Each project has its own **Scratchpad**, containing only that project's freeform working notes. Opening a project's Scratchpad immediately opens that context; the user never has to filter a global note feed or scan project metadata to find their work. New scratch notes automatically belong to the open project. Show a compact local note index beside the editor, with optional pinning and local search. Maintain the index automatically rather than requiring an `_INDEX.md` file.

Keep **Logs** as a separate project-local section for freeform accounts of research activity. Scratchpad holds ideas, speculation, drafts, and working material. Neither section imposes headings, formatting, a next action, or a timer. A lightweight, changeable note kind distinguishes scratch notes from logs; it does not impose a content schema.

Sessions are optional records of when work occurred and which notes were used. A session can reference several notes, and a note can continue across sessions. A log can exist without a session. Use `ResearchNote` as their common storage entity without turning common storage into a single mandatory user-facing feed.

Notes may be explicitly linked to additional projects, with one underlying body and no duplicate copies. Show a shared note only in projects to which the user linked it; do not include other projects' notes merely because a session spans those projects. Notes created outside a project can remain in a separate unassigned capture inbox. Cross-project search and browsing are optional actions in Research, not the default project working surface. Apply the same local Scratchpad and Logs pattern to Areas, matching the vault’s discipline organization.

“Continue research” reopens the selected project's Scratchpad or Logs and the relevant note. For sessions spanning multiple projects, offer explicit project/context choices rather than combining their scratchpads.

Verified against the Windows vault at `/mnt/c/Users/Zayan/Documents/obsidian_vaults/meta_coordinator`: its README describes per-project/per-discipline `Dive/Scratch/` folders and manual indexes. Actual examples include `Work/P-AgenticSystem/Dive/Scratch/` (experiments, improvements, paper outline), `Work/P-CombiningEvidenceInLLMs/Dive/Scratch/`, and a separate `Dive/Logs/` in the latter project. Preserve this contextual organization while removing manual index maintenance.

## 2.2 Separate outcomes from ongoing responsibilities

Use only four top-level work concepts:

| Concept | Meaning | Example |
|---|---|---|
| **Project** | A finite outcome with a completion condition | Submit a paper |
| **Area** | An ongoing responsibility without an end date | PhD, health, teaching |
| **Milestone** | A meaningful intermediate outcome inside a project | Complete literature synthesis |
| **Task** | A concrete action that belongs to a milestone | Classify ten relevant papers |

“Discipline” should become **Area**. “Project” should remain Project.

This is simpler than placing disciplines and projects in the same folder and giving them nearly identical internals, as the existing vault currently does. README.md:6-16

## 2.3 Enforce the hierarchy

The primary relationship should be:

```text
Area
└── Project
    └── Milestone
        └── Task
```

Rules:

- Every active project belongs to one area.
- Every actionable task belongs to one milestone.
- Activating a project requires an Area and at least one planned or active milestone. Completing its final milestone may leave the project active while the user reviews the outcome; do not force project closure or a dummy milestone. Draft projects may start with a Scratchpad before work is planned.
- Tasks without a milestone go to a temporary **Clarify** queue.
- A milestone describes a result, not an activity.
- Completed projects become read-only unless reopened. Preserve the note revisions visible at completion so edits to a shared note from another active project do not alter the archived project. Reopening uses current shared revisions for editing while keeping the completion snapshot available read-only. Restoring an older body is an explicit new revision, with the affected editable contexts shown; it never silently rolls back a shared note.
- References, reading items, scratch notes, and research logs may relate to projects or milestones without belonging to the task hierarchy. Notes and logs do not require a milestone. Area notes are explicitly linked to the Area; they do not automatically aggregate its projects’ notes.

This directly satisfies “tasks should be under milestones” and prevents the current situation in which milestones and tasks are separate files without an enforced relationship. The dashboard currently queries all milestone-tagged tasks and all unfinished tasks globally, but does not encode milestone ownership. Dashboard.md:32-40

---

# 3. Proposed information architecture

## 3.1 Main navigation

Keep the application to six primary destinations:

1. **Today**
2. **Work**
3. **Research**
4. **Library**
5. **Review**
6. **Settings**

Avoid separate top-level navigation destinations for tasks, milestones, reading logs, and integrations. These belong inside the relevant contexts. Scratchpad and Logs are sections within a Project or Area.

**Work** opens project/area workspaces. **Research** is a launcher for recent and pinned research contexts plus optional session history and an unassigned capture inbox. It opens the same project-local Scratchpad or Logs used in Work. Cross-project note browsing/search requires an explicit action; Research does not default to a combined note feed.

### Capture, search, and return behavior

- “New note” inside a Scratchpad or Logs inherits that project/area and section, with no metadata form. Default to a blank editor; titles and templates are optional.
- Global capture opened from a project/area defaults to that context and Scratchpad, visibly indicating the destination. The user can switch to Logs or another context. Outside a project/area, capture defaults to the unassigned inbox; do not guess from an old session.
- Local search stays within the open section and project/area. Offer explicit broader project search and “Search everywhere”; global results identify their context and open there.
- Retain the last-opened note and editor position per context. Switching projects restores each project's own working context.
- Autosave freeform drafts locally and show pending/saved/error state. “Saved” means committed to local storage. Flush pending edits on context switch; if persistence fails, retain the draft and show recovery/retry instead of discarding it. Concurrent editors use revision checks and preserve conflicting drafts rather than silently replacing newer text. Note creation does not depend on a title or network connection.
- Linking a note to another project is explicit and updates that project's matching section. A resource link or session link never implicitly shares a note.
- Removing a project link removes the note from that project only. Deleting a shared note is a separate action whose affected contexts are shown. If all editable context links are removed, keep the current note in the unassigned inbox even when archived snapshots remain. Unlinking never silently discards content. Explicit deletion moves current content to recoverable trash; archive snapshots remain until explicitly removed. Archiving a project alone does not move its notes into the inbox. The inbox contains never-assigned notes or notes explicitly unlinked from their final editable context; archive-only notes remain discoverable through Archive.

## 3.2 Today: the homepage

The dashboard should be the homepage, but it should be action-oriented rather than a complete report.

Recommended order:

### Header

- Date and time.
- Current academic role.
- A concise personalized positive message.
- Global capture button.
- “Resume” entry point to recent and pinned research contexts across projects.

### Daily startup

- Today’s generated checklist.
- Progress ring.
- Estimated total duration.
- “Skip with reason” rather than forcing completion.
- Add/remove/reorder controls.
- Closed or expired startup instances become immutable daily records; checking the last item does not close the day automatically.

### Continue research

Show up to three recent or pinned project/area research contexts, with “Show more” for the rest. Each card includes its context name, last activity time, and a preview of its last-opened Scratchpad or Log note. Reopening a note is useful without session tracking and must not start a timer or create a log.

An optional multi-project session card appears once, with explicit project/context destinations. Selecting a destination opens only that project's section and notes. Group its destinations within the card rather than repeating it for every linked project. Pinning lets users retain less recently used contexts.

“Resume session” is a separate explicit action: create a new session interval linked through `resumed_from_session_id`, retaining previous timestamps and allowing the user to choose today's project links. Merely opening a note creates no session. The user decides what to do next when returning.

### Active milestones

Show a limited number—ideally three to five—rather than every open task in the system:

- Project.
- Milestone status.
- Target date.
- Progress based on completed tasks.
- Next task.
- Blocker indicator.

### Reading now

A compact card for currently active reading items:

- Cover or source icon.
- Title and author.
- Current location and total.
- Gradient progress bar.
- Last reading date.
- “Log progress” button.
- Most recent takeaway.

### Recent academic reading

A chronological feed:

```text
Today, 08:50 · Paper
Read pp. 4–11 of “Example Paper”
Takeaway: The authors distinguish epistemic conflict from label noise.
Related project: Combining Evidence in LLMs
```

### End-of-day action

A single “Close day” workflow:

- Review unchecked startup items.
- Leave unchecked items on the active list automatically, like TasksNext.
- Optionally dismiss or revise an item.
- Optionally record a short day outcome.
- Archive the daily snapshot without removing unfinished work.
- Choose the next research action upon returning; no advance commitment is required.

The existing dashboard already contains projects, milestones, global tasks, and book progress, so this is an evolution of its intent rather than an unrelated replacement. Dashboard.md:1-18 Dashboard.md:22-40

---

# 4. Daily startup design

## 4.1 Templates versus daily instances

Do not ask an AI to invent the checklist every morning.

Use:

- **Role templates**: PhD student, undergraduate, lecturer, independent researcher, etc.
- **User rules**: weekdays, teaching days, deadlines, preferred work hours.
- **Daily context**: calendar, overdue milestones, and persistent unfinished items.
- **Generated instance**: a checklist initialized from a saved template/context snapshot for a particular local date. Its live items remain editable until the day is closed; generation is not rerun on every refresh.

AI can suggest revisions, but generation should be deterministic and explainable. In the MVP, calendar/teaching/meeting conditions come from user-entered rules; calendar account integration is not a prerequisite. Missing optional context omits the corresponding conditional item rather than blocking startup.

## 4.2 Example PhD startup template

```text
Daily startup — PhD researcher

[ ] Review today's calendar and fixed commitments
[ ] Review the active milestone and its completion criterion
[ ] Reopen relevant research notes
[ ] Choose one principal research outcome for today
[ ] Open the relevant project materials
[ ] Check experiments or long-running jobs
[ ] Process no more than three urgent messages
[ ] Begin a focused research session
```

Conditional items could include:

- On teaching days: review class obligations.
- On meeting days: review last meeting decision and agenda.
- When experiments are running: inspect their status.
- When a deadline is within seven days: confirm the critical path.
- When the reading queue is stale: select one reading item.

## 4.3 Archival

Persist templates and daily instances, with item snapshots and carryover links as described in section 14:

```text
DailyStartupTemplate
DailyStartupInstance
```

Each instance records:

- Date.
- Template version.
- Items presented.
- Completion state and timestamp.
- Skipped items and reason.
- Context used to generate it.
- Close-of-day summary.

Archive explicitly closed or expired daily snapshots and keep them queryable through Review. Before archival, item completion, additions, and ordering remain editable. Unchecked work remains active until completed or explicitly dismissed, except routines whose user-configured carryover is disabled. Use stable item identities and carryover links: yesterday's unfinished recurring item must not also be generated as a duplicate today. Completing a carried item updates the active record and today's activity, leaving yesterday's snapshot unchanged. Separate the persistent open occurrence (`StartupItem`) from each day's presentation/state (`DailyStartupItem`). Deduplicate by template ID plus item ID while an occurrence is open. Weekday rules create new occurrences; they do not hide unfinished carryover on other days. Completing an old occurrence satisfies today's displayed item; create the next occurrence only on the next eligible date. Record the instance's local date and timezone and generate it idempotently. Missing days do not create a backlog of routine instances.

“Skip today” records a reason on today's presentation while leaving a carried occurrence open; “Dismiss” ends that occurrence. Neither action disables the recurring template. After “Close day,” research, reading, and persistent unfinished items remain usable. Later item changes record timestamped activity without rewriting the closed snapshot; the next daily instance reflects the current item state. No second startup instance is generated for the same date. Correct a closed record through an attributed amendment, preserving its original snapshot.

## 4.4 User-owned startup templates

Provide a configurable `templates/startup/` folder in the user's application data, with an “Open templates folder” action. Recommend TOML for stable IDs, ordering, recurrence, and optional conditions; provide an in-app editor so editing configuration text is optional. Item text remains user-defined. Ship an editable example:

```toml
schema_version = 1
id = "my-research-startup"
name = "Research morning"

[[items]]
id = "review-context"
text = "Reopen my research notes"
carry_forward = true
weekdays = ["mon", "tue", "wed", "thu", "fri"]

[[items]]
id = "check-jobs"
text = "Check running experiments"
carry_forward = true
```

Array order controls presentation; omitted weekdays means every day. Carryover defaults to enabled. Disabling carryover for a routine leaves its incomplete occurrence in the archived day only; it never deletes a separately linked task. Startup checklist items are routine records, distinct from milestone-owned Tasks; referencing a Task does not change its ownership. Validate edits and retain the last valid version if parsing fails. Templates are data, never executable scripts. Snapshot the template version and content into each generated day. Edits affect future instances; deleting a template item must not delete unfinished carried work. Keep template files separate from generated daily records, and include both in backup/restore.

---

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

## 5.4 Optional disk export

Export should preserve project/area context and the Scratchpad/Logs distinction:

```text
Export/
├── Work/
│   ├── Areas/<area>/
│   │   ├── Overview.md
│   │   ├── Scratchpad/Index.md
│   │   └── Logs/Index.md
│   ├── Projects/<project>/
│   │   ├── Overview.md
│   │   ├── Scratchpad/Index.md
│   │   └── Logs/Index.md
│   └── Archive/<project>/
├── Notes/                         # canonical bodies, stable IDs
├── Inbox/Index.md
├── Reading/2026/
├── Library/Library Index.md
└── Daily/2026/
```

Each project/area index links only to its own notes, preserving optional pin/order settings. Canonical note files carry stable IDs and freeform bodies; multiple context indexes can link to the same file without editable copies. Export preserved revisions for archived-project links separately when necessary. Resolve imported wikilinks and attachment references through an original-path-to-ID mapping; report unresolved links without altering the original vault. The user enters through a project index, with no need to browse the canonical Notes directory.

Generate indexes automatically and write files atomically. JSON backup/restore preserves note bodies, revisions, kind, project/area/milestone/resource links, session links, context state, and import provenance. Preserve local note attachments in a backup bundle or provide an explicit missing/external attachment report; JSON references alone are not an attachment backup.

The phase 1 backup contract covers the entire application state, not just notes: hierarchy, notes/revisions, sessions, startup templates and occurrences, resources and reading events as those features arrive, import mappings, and schema version. Separate portable application data from device paths and credentials. Restore into a staging database, validate schema and relationships, preview missing files, and activate only after validation; retain the previous database for rollback. MVP restore replaces a workspace rather than merging two histories. Pause ingestion during restore, then require account reauthorization/reconnection as needed and reconcile source cursors before resuming. Rebuild search indexes from restored records. Never export OAuth or local capture tokens; rotate the capture token and repair extension pairing when restoring onto another installation.

---

# 6. Library design

## 6.1 Unified resource model

All source types should normalize into one `Resource` model:

```text
Resource
- id
- sources[]: ResourceSource
- canonical_url
- attachments[]: ResourceAttachment
- title
- authors[]
- publication_date
- resource_type
- abstract_or_description
- page_count_or_duration
- thumbnail
- topics[]
- level
- prerequisites[]
- reading_state
- progress
- project_links[]
- first_seen_at

ResourceAttachment
- id
- resource_id
- source_id (optional)
- local_path_or_url
- content_hash
- page_count_or_duration

ResourceSource
- id
- resource_id
- source_type
- source_account
- external_id
- source_metadata
- source_notes[]
- content_hash
- sync_version
- first_seen_at
- last_seen_at
- last_synced_at
```

Supported resource types:

- Book.
- Book chapter.
- Journal article.
- Preprint.
- Thesis.
- Web article.
- Video.
- Lecture.
- Dataset.
- Code repository.
- Presentation.
- Other PDF.

## 6.2 Integrations

### Google Drive

The user selects one or more Drive folders.

The connector should:

1. Perform an initial paginated crawl.
2. Save source IDs and synchronization cursors.
3. Fetch metadata before downloading content.
4. Download or parse a file only when needed.
5. Detect additions, updates, moves, and deletions.
6. Store source metadata separately from user corrections.
7. Use scheduled incremental synchronization with change notifications where deployment permits.

### YouTube

Allow:

- Selected playlists.
- “Watch later” only if supported through the chosen authorization flow.
- Individual videos.
- Channel playlists as an optional later feature.

Store video duration, channel, playlist membership, position, watched progress when obtainable, and user-entered progress otherwise.

### Academic literature

The user's academic references are distributed across **Excel, Zotero, and Mendeley Web**, with overlap. Support all three as inputs to one canonical library without requiring migration out of the existing tools.

- **Excel:** import XLSX/CSV with a column-mapping preview for title, link, publication date, and notes. Preserve original cell values and source workbook/sheet provenance. Prefer a stable row ID when available; row number alone is not identity because rows can move. Reimports without stable IDs require content comparison and review for ambiguous edits.
- **Zotero:** implement read-only incremental synchronization of metadata, collections, and notes using library/item identities and versions. Attachment access is a separate concern. [Zotero API synchronization documentation](https://www.zotero.org/support/dev/web_api/v3/syncing).
- **Mendeley:** initially use supported reference exports. Current guidance documents reference exports including BibTeX, RIS, and EndNote XML, but does not establish the available export workflow for this user’s web library. Verify that workflow and, if needed, access to the same library through the desktop application during discovery; do not promise direct web synchronization or assume exports preserve every note. [Mendeley export guidance](https://www.elsevier.support/mendeley/answer/how-can-i-export-my-library).

Spreadsheet and reference exports are snapshots that may contain only selected rows/items. Absence from a later import is not a deletion signal. Tombstone source membership only from a provider's explicit deletion event or a successful complete scan of the configured scope; incomplete scans, authentication failures, and filtered exports must not remove records. Every import reports counts, unresolved matches, and fields/notes absent from the source format rather than claiming unavailable content was preserved.

### Overlap and deduplication

One canonical `Resource` can have multiple `ResourceSource` records. Store provider IDs, original metadata, source notes, and import history separately from user corrections.

1. Recognize an already-imported source item by source identity.
2. Match normalized persistent identifiers such as DOI or PMID, checking contradictory metadata before automatic linkage.
3. Use title, author, and year similarity to propose matches for review; title alone is insufficient.
4. Preserve preprints, published versions, editions, and supplements as distinct but related records when appropriate.
5. Preserve notes from all three sources with attribution. Never overwrite an Excel note with a Zotero or Mendeley note.
6. Make merges reversible. One removed source link must not remove the canonical resource's other links, notes, or reading history.

A resource may have multiple files with different pagination; preserve attachment identity rather than treating one local path as definitive. The source records own provider-specific fields such as account, external ID, sync version, and original metadata. The canonical resource owns resolved display metadata and user activity; it must not have only one source slot.

Imported Excel, Zotero, and Mendeley annotations remain attributed `ResourceSourceNote` records in the resource detail view. They are not automatically research notes. A user can explicitly create a Scratchpad note or Log referencing an annotation in a chosen context; the imported original and its provenance remain intact. Resource/project associations do not automatically populate project Scratchpads.

## 6.3 Keeping the library alive

Use an ingestion pipeline:

```text
Connector scan
→ raw source event
→ metadata normalization
→ identifier/metadata extraction
→ duplicate resolution
→ metadata search indexing and usable library record
→ UI notification
→ optional full-text extraction, classification, and graph enrichment
```

Required operational properties:

- Incremental cursors.
- Idempotent jobs.
- Exponential retry.
- Per-connector rate limiting.
- Content hashes.
- Tombstones for externally deleted items.
- A dead-letter queue for failed imports.
- User-visible last-sync state.
- Manual “sync now.”
- Never silently delete a user’s annotations or reading history.

## 6.4 Agent-assisted organization — proposed approach

Use connectors to enumerate and synchronize sources, then an optional constrained agent to interpret their content. Drive exposes a changes feed and YouTube exposes playlist-item enumeration; use these interfaces for retrieval rather than asking a model to navigate the source from scratch. [Drive changes](https://developers.google.com/workspace/drive/api/guides/manage-changes), [YouTube playlist items](https://developers.google.com/youtube/v3/docs/playlistItems/list).

Suggested flow:

```text
Selected source → reliable inventory → normalized records → deduplication
                → contextual classification suggestions → library + review queue
```

Give the agent folder ancestry, playlist title/description/order, bibliographic metadata, and bounded text samples when the user permits it. It can suggest topics, resource types, levels, collection membership, and likely duplicates. For example, a course playlist can become a collection preserving lecture order, with inferred topics offered as editable suggestions. Resources may belong to several collections.

Each suggestion stores evidence, rationale, model/version, and confidence. Confidence is a review signal, not proof. Apply deterministic mappings automatically; initially review agent suggestions in batches. Allow later opt-in automation for tested, reversible classifications. Never let the agent delete originals, overwrite user corrections, silently merge uncertain matches, or obey instructions embedded in imported documents.

Cache results by content hash and classification version, process only changed material, cap model usage, and queue failures independently of successful imports. Manual organization and ordinary import must work with AI disabled. Begin with rules and metadata; add bounded agent tools for missing identifiers and ambiguous classifications after the import pipeline works.

## 6.5 Lazy loading

The UI should:

- Fetch library rows in cursor-based pages.
- Load thumbnails only when visible.
- Load full text only when a user opens an item or when a background indexing budget allows.
- Cluster only the visible topic or filtered result set.
- Move graph-layout computation to a Web Worker.
- Render large graphs using WebGL, not thousands of DOM elements.
- Cache recent queries and resource cards.

---

# 7. Library list and cluster views

## 7.1 List view

Columns and filters:

- Title.
- Author/source.
- Type.
- Topic.
- Level.
- Reading state.
- Progress.
- Added date.
- Source.
- Related project.

Saved views:

- Reading now.
- Unread introductory material.
- Recently added.
- Papers for an active milestone.
- Videos under 30 minutes.
- Items missing metadata.
- Duplicates requiring review.

## 7.2 Cluster view

The graph should have three intentional modes rather than one visually impressive but confusing network:

### Topic map

- Resource nodes clustered by topic.
- Color represents topic.
- Shape represents resource type.
- Size represents relevance or engagement.

### Learning path

- Left-to-right progression:
  - Orientation.
  - Introductory.
  - Intermediate.
  - Advanced.
  - Frontier/research.
- Edges represent recommended sequence or prerequisites.

### Project evidence map

- Project and milestone nodes.
- Connected papers, books, videos, datasets, and explicitly included Scratchpad/Log notes from the selected project. Open notes in their project-local section; expanding to other projects requires an explicit action.
- Edge type indicates “supports,” “challenges,” “background,” or “method.”

Always provide a synchronized list beside the graph. Selecting a node should filter the list, and selecting a list item should highlight the node.

---

# 8. Material levels and learning paths

## 8.1 Do not rely on AI alone

Each resource receives:

- `level`: orientation, introductory, intermediate, advanced, frontier.
- `level_source`: user, imported, rules, AI.
- `level_confidence`.
- `level_rationale`.
- `prerequisites`.
- `topics`.

Use evidence in this order:

1. User designation.
2. Course or collection placement.
3. Publisher and bibliographic metadata.
4. Explicit phrases such as “introduction,” “handbook,” or “graduate text.”
5. Citation and reference relationships.
6. Language-model classification.

The user can always override a level.

## 8.2 “Learn this topic” workflow

When the user selects a topic:

1. Ask what they already know.
2. Ask available time and preferred media.
3. Select only saved materials by default.
4. Build a short path with an explanation:
   - One orientation item.
   - One introductory foundation.
   - One applied item.
   - One advanced or frontier item.
5. Detect prerequisite gaps.
6. Offer external discovery only through an explicit separate action.
7. Save the result as a collection, not a new project unless the user requests one.

---

# 9. Reading progress model

A single percentage is not sufficient for every medium. Store both normalized and native progress:

```text
ReadingProgress
- resource_id
- unit: page | chapter | section | timestamp | percent | location
- attachment_id (optional; identifies the edition/file used)
- current_value
- total_value (nullable)
- normalized_percent (nullable when no reliable mapping exists)
- started_at
- updated_at
- completed_at
- source: browser | pdf | manual | import
```

Examples:

- PDF: page 17 of 42.
- Book: page 120 of 550.
- EPUB/web reader: 38%.
- Video: 24:15 of 1:10:00.
- Paper without reliable pages: section 3 of 7.

Current progress is keyed by a reading track: resource plus attachment (or an explicit resource-level track when there is no attachment). Each track has its own revision and unit. `Resource.progress` is a display projection of the user's selected track, not an independently writable value. Switching attachments must not overwrite the previous track; the resource card labels the selected file/unit. Changing units needs an explicit mapping or a fresh progress value, never an assumed conversion.

For numeric units with known positive totals, derive a percentage within 0–100. Unknown totals and opaque location strings retain native progress without an invented percentage; show the location and allow manual correction. Do not transfer page offsets between attachments with different pagination.

Each capture has a client-generated event ID, client timestamp, and server receipt timestamp. Retrying the same ID is idempotent. Append corrections as new events; do not rewrite history or use maximum progress, since rereading can move backward. A queued event carries the progress revision it was based on: if that revision is stale, retain the event in history and offer it for review rather than silently overwriting newer current progress. Apply the resource update, progress event, and optional reading log in one transaction. A stale event receives a durable “saved for review” receipt; it is not retried indefinitely as a failed write. Capture without a known base revision may initialize an empty track, but must enter review before replacing existing progress. Do not let unresolved conflicts block unrelated captures.

Reading activity uses `ReadingLogEntry`, separate from a freeform research Log (`ResearchNote` with `kind = log`). Show reading entries with their resource and the Today reading feed. Linking a reading entry to a project makes it available in that project's reading history, not its research Logs. Users may explicitly reference it from a Scratchpad note or research Log.

## 9.1 Gradient slider

Use a clear, accessible gradient:

- 0–25%: muted violet.
- 25–60%: blue.
- 60–90%: cyan/green.
- 90–100%: green/gold.
- Completed: solid green with a check.

Requirements:

- Keyboard-operable.
- Numeric label.
- Native unit label.
- High-contrast mode.
- No reliance on color alone.
- Optional snap points for chapters or sections.

The current dashboard’s book progress is manually encoded as text inside a progress-bar block. Dashboard.md:13-18 The new progress record should generate that presentation from structured data.

---

# 10. Browser extension plan

## 10.1 Extension responsibilities

The extension should:

- Detect the current item.
- Extract metadata.
- Show existing library state.
- Let the user set progress.
- Accept a takeaway and optional project/milestone.
- Save to the local application.
- Queue locally if the application is temporarily unavailable.
- Avoid capturing full page content unless the user explicitly enables it.

## 10.2 Metadata extraction order

User corrections always take precedence over extracted metadata and remain protected on later captures. For fields without a correction, use this extraction order:

1. DOI, ISBN, arXiv ID, PMID, or other persistent identifier.
2. Embedded JSON-LD/schema.org metadata.
3. Citation meta tags.
4. Open Graph metadata.
5. Browser page title and visible byline.

Extract:

- Title.
- Author(s).
- Publication date.
- Publisher or venue.
- Canonical URL.
- Resource type.
- DOI/ISBN/arXiv ID.
- Page count or duration when available.

Display confidence and allow correction before saving.

## 10.3 Popup interaction

```text
Title
Author · publication date
Source/type

Progress
[ gradient slider ---------------- ] 42%
Page 21 of 50

What did you take away?
[                                      ]

Project      [select]
Milestone    [select]
Topics       [select]

[Save]
```

Saving should create:

1. Create or update the resource.
2. A progress event only when progress was provided; saving a resource or takeaway must not require a page number or percentage.
3. A reading-log event if a takeaway or reading-session detail was provided.

Capture saves resource metadata, optional progress, and optional reading history; it does not start a research session or create a Scratchpad/research Log note. Project selection is optional and visible, with milestones restricted to the selected project. The same rule applies to native PDF capture. Linking a resource or reading entry to a project updates that explicit association only. If an offline capture refers to a project that has since been archived, save the resource/activity and flag the requested association for review; do not mutate the archived context or lose the capture.

A capture ID deduplicates the whole transaction even when no progress is supplied. Keep queued payloads until the desktop acknowledges durable storage. Distinguish “Queued on this device,” “Saved,” and “Saved for review”; network receipt alone is not a successful save.

## 10.4 Writing to disk

The extension should **not** receive arbitrary filesystem permissions.

Preferred flow:

```text
Browser extension
→ authenticated local API/native messaging
→ desktop application
→ SQLite transaction
→ optional atomic Markdown/JSON export
```

Use:

- A randomly generated local authentication token.
- Loopback-only binding.
- Strict extension-origin allowlist.
- Schema validation.
- Atomic file replacement.
- An append-only event record for recovery.
- No credentials inside the vault or exported files.

---

# 11. Native PDF companion plan

## 11.1 Feasibility boundary

“Click any PDF window and automatically extract everything” is feasible only in degrees. PDF applications expose different amounts of information.

The desktop companion should use a tiered strategy:

1. **First-party in-app PDF reader**
   - Most reliable.
   - Exact page number, page count, title, and reading duration.

2. **Browser PDF viewer integration**
   - Browser extension can identify the URL and sometimes page state.

3. **Native viewer adapters**
   - Okular through Linux window metadata, DBus where available, recent documents, and file association.
   - Adobe Acrobat through supported automation/accessibility interfaces where available.
   - Platform accessibility APIs for active-window title and controls.

4. **Fallback**
   - Identify a likely file from window title.
   - Ask the user to confirm it once.
   - Remember the viewer-title-to-path mapping.
   - Let the user enter the current page if the viewer does not expose it.

Do not use screen scraping or OCR as the primary mechanism.

## 11.2 Interaction

Provide a global shortcut such as:

```text
Ctrl+Alt+L — Log reading progress
```

Flow:

1. Detect foreground window and process.
2. Resolve the PDF path or URL.
3. Extract PDF metadata and total page count.
4. Attempt to retrieve the current page.
5. Show a small overlay.
6. Let the user correct the page, title, author, and date.
7. Record progress and takeaway.
8. Link the event to an optional project and milestone.

## 11.3 Implementation order

Start with:

1. The application’s own PDF reader.
2. Browser PDFs.
3. Okular on Linux.
4. Adobe Acrobat on Windows.
5. macOS Preview.
6. Additional readers based on demand.

Trying to support every PDF reader in the first release would delay the useful core product.

---

# 12. Personalized daily message

## 12.1 Profile settings

Let the user configure:

- Role and career stage.
- Current priorities.
- Preferred tone.
- Formality.
- Warmth.
- Concision.
- Humor.
- Metaphor density.
- Whether literary language is welcome.
- Topics or expressions to avoid.

Offer tone presets such as:

- Warm and cutesy.
- Calm and direct.
- Scholarly and austere.
- Literary and introspective.
- Energetic coach.
- Minimalist.

Prefer descriptive style controls such as “19th-century philosophical prose” rather than promising exact imitation of a particular writer.

## 12.2 Generation policy

Generate once per day and cache the result.

Context should be limited to:

- The user’s chosen name.
- Role.
- Today’s principal milestone.
- Recent completion or streak.
- Explicit tone settings.

Do not send Scratchpad bodies, research Logs, imported annotations, paper text, or sensitive project content unless the user explicitly opts in to that context and provider. Sharing a note across local projects does not grant AI consent. A chosen milestone is optional; missing one must not trigger a required planning step.

Fallback message templates must exist for:

- Offline mode.
- Provider failure.
- Missing API key.
- Safety or privacy settings.

The message should encourage without falsely claiming knowledge of the user’s emotional state.

---

# 13. Proposed technical architecture

```text
┌───────────────────────────────┐
│ Tauri desktop application     │
│ React + TypeScript UI         │
├───────────────────────────────┤
│ Rust application services     │
│ - local API/native messaging  │
│ - filesystem/export           │
│ - OS/PDF integrations         │
│ - secure credentials          │
├───────────────────────────────┤
│ SQLite                        │
│ - normalized records          │
│ - event log                   │
│ - full-text search            │
│ - sync cursors                │
├───────────────────────────────┤
│ Background workers            │
│ - Drive connector             │
│ - YouTube connector           │
│ - Zotero/literature connector │
│ - metadata extraction         │
│ - graph/index processing      │
└───────────────────────────────┘
          ▲              ▲
          │              │
 Browser extension    Optional web client
```

## 13.1 Suggested repository layout for implementation

When implementation begins, place the application outside the vault content structure:

```text
apps/
├── desktop/
├── extension/
└── web/

crates/
├── core/
├── storage/
├── connectors/
├── pdf-observer/
└── export/

packages/
├── domain/
├── schemas/
├── ui/
└── client/

docs/
├── architecture/
├── product/
└── decisions/

fixtures/
└── sample-library/
```

Do not put application source inside `Work/` because that directory is currently defined as user work content rather than software infrastructure. README.md:4-8

---

# 14. Core data entities

Core schema and later feature entities (the release boundary is in section 20):

```text
UserProfile
Area
Project
Milestone
Task

ResearchSession
ResearchSessionProject
ResearchSessionArea
ResearchSessionMilestone
ResearchNote
ResearchNoteRevision
ResearchNoteProject
ResearchNoteArea
ResearchNoteMilestone
ResearchSessionNote
ResearchNoteResource
ResearchContextState
DailyStartupTemplate
DailyStartupInstance
StartupItem
DailyStartupItem
StartupItemEvent
DailyStartupAmendment
StartupCarryoverLink

Resource
ResourceAttachment
ResourceSource
ResourceSourceNote
ResourceProject
ResourceMilestone
ResourceIdentifier
ResourceTopic
Collection
CollectionItem

CaptureReceipt
ReadingProgressEvent
ReadingProgress
ReadingLogEntry
ReadingLogProject
ReadingLogMilestone

ConnectorAccount
SyncCursor
IngestionJob
ImportConflict
ImportBatch
ImportRecordMapping

Topic
TopicRelationship
ResourceRelationship
```

Important constraints:

- `Task.milestone_id` is required for actionable tasks.
- `Milestone.project_id` is required.
- `Project.area_id` is required for active projects.
- `ResearchNote` stores ID, optional title, freeform body, `kind: scratch | log`, created/edited times, and an optional activity date for Logs. Blank drafts are allowed. Note revisions support recovery and completed-project snapshots.
- Next actions, headings, templates, and sessions are optional. Moving note kind does not duplicate its body.
- Project/area joins define note visibility, with per-context pin/order and optional archived revision/kind references. Preserve archived membership against later unlink/delete operations; restoring or removing archived material is a separate explicit action. Milestone links require the note’s explicit association to the milestone’s owning project. Removing that project association also removes the associated milestone links in the same operation, while retaining other project links.
- `ResearchContextState` stores last-opened section/note and editor position by project/area. These navigation details never broaden note membership.
- Session membership and note membership are independent. Session-note joins do not create project-note joins. `resumed_from_session_id` preserves session history.
- Resource source annotations, reading logs, and research notes have distinct storage and routing; explicit links connect them without automatic copies.
- Sessions and notes support multiple project/milestone links.
- Milestone placement, completion weights, and manual project estimates are separate fields.
- Archived daily snapshots are immutable; carryover retains stable item identity.
- A canonical resource may have multiple attributed source records and notes.
- Progress events are append-only with unique client event IDs; current progress updates check the supplied base revision. Native units and attachment identity remain authoritative when normalized progress is unavailable.
- Current progress is derived or transactionally updated from events per resource/attachment track, with at most one current record per track. `CaptureReceipt` deduplicates resource-only and takeaway-only captures as well as progress captures. Startup post-close activity/amendments are stored separately from immutable daily snapshots.
- Imported metadata and user-corrected metadata are stored separately.
- Source deletion never deletes user-generated activity.
- Every AI-generated classification stores provenance and confidence.

---

# 15. Delivery roadmap

## Phase 0 — Product discovery and prototypes

**Goal:** remove ambiguity before constructing the system.

Deliverables:

- Confirm target operating systems.
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

**Goal:** organize work through typed objects while preserving freeform research notes.

Deliverables:

- SQLite schema and migrations.
- Area → Project → Milestone → Task hierarchy.
- Work Focus view.
- Project view.
- Project-local Scratchpad and Logs sections with automatic note indexes and blank-note capture; equivalent Area sections.
- Optional cross-project browsing and an unassigned capture inbox, separate from project working views.
- Session-to-note links supporting several notes per session and continued notes across sessions.
- Global capture dialog.
- Search.
- JSON backup and restore.
- Read-only importer and dry-run migration report for the existing NewSecondBrain structure. Import work and notes first; retain unresolved reading/reference entries in import staging until phase 3 can normalize them.

Exit criterion:

- The user can manage a real project for a week, including freeform scratch work and logs, without maintaining vault indexes or task/milestone files. Switching between two test projects never shows unrelated notes; backup/restore preserves their bodies and links.

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
- JSON backup remains available from phase 1; full Markdown/YAML export follows in phase 7 and does not gate reading capture.

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

## Phase 7 — Obsidian compatibility and migration

Deliverables:

- Stable Markdown/YAML export.
- Project/area Scratchpad and Logs indexes linking canonical note files, including shared and archived revisions.
- Wikilinks between exported projects, resources, scratch notes, and research/reading logs.
- Link validation.
- Incremental export.
- Validate the phase 1 migration report against exported links; migration itself is not deferred to this phase.
- Optional Obsidian dashboard generated from the structured data.

Exit criterion:

- Exported data is readable without the application and introduces no broken generated links.

---

# 16. Migration plan for NewSecondBrain

## 16.1 Map existing concepts

| Current vault element | New entity |
|---|---|
| `D-*` folder | Area |
| `P-*` folder | Project |
| `Milestones.md` item | Milestone |
| `Tasks.md` item | Task |
| `Reading.md` item | Resource or reading queue entry |
| `References.md` item | Resource |
| `Work/*/Dive/Logs/*` | Freeform log in the owning project/area’s Logs section; preserve available dates |
| `Work/*/Dive/Scratch/*` | Freeform scratch note in the owning project/area’s Scratchpad; preserve content and original path |
| Unscoped scratch notes | Unassigned capture inbox unless ownership is explicit |
| Project `README.md` frontmatter | Project metadata |

The existing project README already contains fields such as `name`, `kind`, and `type`, making it a useful migration anchor. Work/P-CombiningEvidenceInLLMs/README.md:1-8

## 16.2 Migration safeguards

The importer should:

1. Read the source vault without modification; write imported records only to the application database.
2. Produce a dry-run report.
3. Detect duplicate project names and inconsistent `README.md` casing.
4. Preserve original paths.
5. Preserve completion dates and tags.
6. Flag tasks with no obvious milestone.
7. Flag conflict markers and malformed Markdown.
8. Never delete or rewrite the original vault.
9. Allow the same migration to be rerun idempotently.

A dry run is particularly important because at least one current task file contains unresolved Git conflict markers, so permissive parsing and a review queue will be necessary. Work/P-CombiningEvidenceInLLMs/Dive/Tasks.md:28-49

Scratch imports require no manual classification before becoming usable notes. Derive project/area ownership and scratch/log kind from their containing vault folders. Preserve original content, formatting, and paths, including manual index content; generate the application’s navigation index automatically. Leave ownership unset only for genuinely unscoped or ambiguous notes. Users can add explicit cross-project links later. Do not infer timed sessions from log filenames or prose. Preserve unknown dates rather than inventing durations. Track imported notes by source identity; on rerun, local edits and source changes require conflict review instead of overwriting either. Resolve existing note/attachment links through the import mapping and report unresolved targets.

## 16.3 Milestone migration

Existing tasks cannot safely be assigned to milestones automatically merely because both occur inside one project.

Use:

- Heading proximity.
- Shared tags.
- Semantic similarity.
- Dates.
- Manual confirmation.

Uncertain tasks should enter **Clarify**, never be silently attached to a guessed milestone. Preserve their known project as an intake hint; they are not actionable until assigned to a milestone. Projects lacking an Area or milestone import as drafts with their original status preserved in provenance, rather than guessing hierarchy to satisfy activation rules.

---

# 17. Testing strategy

## Domain tests

- Tasks cannot become active without a milestone.
- Milestones cannot belong to multiple projects.
- Progress remains between 0 and 100.
- Completion creates the correct timestamps.
- Imported metadata cannot overwrite user corrections.
- Source deletion preserves logs and progress.
- Blank freeform drafts and unassigned notes can be created without a session, title, milestone, or required prose fields.
- Project Scratchpad and Logs views contain only notes explicitly belonging to that project and section; creation inherits the open context.
- Multi-project sessions never broaden a project’s Scratchpad to unrelated notes.
- Freeform notes and sessions can span multiple projects without duplicate notes or resume cards.
- Scratch imports preserve content and original paths without requiring classification.
- Session-to-note links support multiple notes per session and multiple sessions per note.
- Opening or editing a note starts no session and creates no research Log automatically.
- Capture and search respect project/area/section scope; broader search requires an explicit action.
- Shared note edits appear only in explicitly linked editable contexts; completed-project revisions remain unchanged.
- Unlinking retains the note in remaining contexts or the unassigned inbox; kind changes move it without duplication.
- Imported annotations and reading captures never appear automatically in Scratchpad or research Logs.
- Backup/restore and Markdown export preserve note bodies, context membership, shared identities, archived revisions, and resolvable links.
- Carryover creates no duplicate recurring items and does not mutate archived snapshots.
- Invalid template edits preserve the last valid template. Finishing the checklist does not lock the day; closure does. Timezone changes, refreshes, and template edits do not duplicate occurrences.
- Dragging milestone markers does not complete work. Task changes recalculate unaccepted milestones; explicitly accepted milestones stay complete until reopened. Completing the final milestone does not violate the project activation rule.
- Reimports preserve source notes and corrections; ambiguous matches and distinct versions are not silently merged.

## Workflow acceptance checks

| Scenario | Required behavior |
|---|---|
| Edit a scratch note, switch projects, restart | Restore the committed draft in its original project; preserve pending edits on save failure. |
| Close a project whose note is shared elsewhere | Archive shows its saved revision; the other project's editable note stays live; no inbox copies appear. |
| Close the day, then finish a carried item | Record completion without changing the closed snapshot or generating a duplicate daily instance. |
| Read two PDFs of the same paper | Keep their page positions and revision checks separate; label the selected track. |
| Retry an offline takeaway with no progress | Save one resource update/reading entry and return the existing capture receipt. |
| Import a partial spreadsheet or reference export | Retain previously imported sources absent from this batch; report unavailable fields. |
| Restore a backup with missing files or expired credentials | Validate before activation, retain rollback data, report unavailable attachments, and reconnect before syncing. |

## Connector contract tests

Each connector should be tested against:

- Initial crawl.
- Incremental addition.
- Rename.
- Move.
- Metadata update.
- Duplicate.
- Deletion.
- Authentication expiry.
- Rate limiting.
- Partial outage.
- Pagination.

## Browser extension tests

Use representative fixtures for:

- JSON-LD book page.
- DOI journal page.
- arXiv abstract.
- YouTube video.
- HTML article.
- Browser PDF.
- Page with missing or contradictory metadata.
- Resource/takeaway capture without progress; unknown totals and incompatible attachment pagination.
- Offline retries producing one event/log, late events preserving newer progress, and atomic rollback on failed capture.

## Native integration tests

Test:

- Multiple PDF viewers open.
- Same filename in multiple folders.
- Unsaved or temporary browser PDFs.
- Password-protected PDF.
- Viewer title that omits the extension.
- Page labels differing from physical page indices.
- Unsupported viewer.
- Inaccessible window metadata.

## Performance targets

Initial targets:

- Dashboard useful content visible in under one second from a warm local database.
- Library scrolling remains smooth with 50,000 metadata records.
- Search response under 200 ms for ordinary local queries.
- Opening the cluster view does not load full document bodies.
- Incremental synchronization does not scan every Drive item.
- Browser capture confirmation appears in under 500 ms when the desktop service is running.

---

# 18. Privacy and security requirements

- Local-first by default.
- OAuth tokens in the operating system credential store.
- No secrets in Markdown, SQLite exports, or `.obsidian/`.
- Per-provider AI consent.
- Clear preview of context sent to an AI model.
- Ability to disable all AI without losing core functionality.
- Loopback API authenticated and origin-restricted.
- Export files written atomically.
- Database backups encrypted when requested.
- Connector permissions kept to the smallest feasible scope.
- User-visible audit history for imports and generated classifications.
- Ability to delete cached extracted text independently of source metadata.

This aligns with the repository’s existing direction that secrets should not be committed and local Obsidian configuration should be treated cautiously. AGENTS.md:40-42

---

# 19. Decisions to resolve before implementation

The most important discovery questions are:

1. **Operating systems**
   - Linux only initially?
   - Linux and Windows?
   - Is macOS required?

2. **PDF readers**
   - Exact Okular version and desktop environment.
   - Exact Adobe Acrobat edition and operating system.
   - Whether using an integrated PDF reader is acceptable.

3. **Academic import details**
   - Sources confirmed: Excel, Zotero, and Mendeley Web.
   - Confirm spreadsheet columns and stable row identities.
   - Confirm Zotero library access and Mendeley desktop/export availability.
   - Check note fidelity and duplicate/version examples across all three.

4. **Deployment**
   - Entirely local?
   - Optional private cloud synchronization?
   - Multi-device use required in the first release?

5. **Obsidian’s role**
   - Current decision: optional one-way export/viewer.
   - Confirm export location and desired fidelity. Editing exported files does not update the application; a primary or bidirectional Obsidian interface would require a separate scope change.

6. **Meaning of progress**
   - Page number.
   - Percentage.
   - Chapter.
   - Time.
   - User-selected per item.

7. **Attachments**
   - The startup-list and cluster-view attachments referenced in the request were not identifiable as product mockups in the repository inspection, so they should be reviewed explicitly during Phase 0 rather than guessed from unrelated vault images.

---

# 20. Recommended MVP boundary

The first genuinely useful release should include only:

- Areas, projects, milestones, and tasks.
- Project/area-local Scratchpad and Logs with automatic indexes, scoped capture/search, and optional sessions.
- Explicit note sharing across projects and a separate unassigned inbox; resumable contexts preserve project boundaries.
- Editable startup templates, daily archive, and persistent unchecked items.
- Today dashboard.
- Unified resource records.
- Local PDFs, Zotero synchronization, and Excel/Mendeley export imports.
- Browser extension capture.
- Reading progress and reading-log cards.
- List view.
- JSON backup and restore.

Defer:

- Google Drive and YouTube playlist connectors until the MVP import set works; these remain subsequent phase 3 increments.
- Agent-assisted organization until reliable ingestion and review are established.
- Optional AI daily messages and complete Markdown/YAML export.
- Semantic cluster graph.
- Automatic learning paths.
- Every academic database.
- Every native PDF reader.
- Bidirectional Obsidian sync.
- Collaboration.
- Mobile applications.
- Fully automatic AI classification.

That boundary produces a coherent academic workflow before investing in visually attractive but operationally complex graph features.

---

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

The implementation request resolves the initial operating-system target to **Linux**, using Tauri, React/TypeScript, Rust, and SQLite. The first vertical slice is narrower than full phase 1: Areas and draft Projects; their local Scratchpad/Logs; freeform autosave and context restoration; Milestones with Tasks; and validated backup/restore. Project activation/archive, sharing UI, imports, sessions, startup routines, integrations, AI, graphs, PDF tracking, browser capture, and the interactive milestone bar remain subsequent work. This boundary does not claim phase 1 or the overall MVP is complete. See [ADR 0001](docs/adr/0001-linux-local-foundation.md) for the implemented persistence and restore architecture.
