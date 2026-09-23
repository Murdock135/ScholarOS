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
   - The export is one-way in the first release; bidirectional synchronization should not be attempted until the data model is stable.

This preserves the parts of NewSecondBrain that already work—projects, disciplines, milestones, tasks, reading, and dashboards—without retaining its folder and file-management burden. The existing vault explicitly divides `Work/` into projects and disciplines and gives each item five or more subordinate files, which is understandable but creates repeated navigation and maintenance overhead. README.md:3-18 The current scaffolding script reinforces that repetition by creating a `Dive` directory and five template files for every work item. .scripts/make_work_item.py:4-26

I would **not** begin by building an Obsidian plugin. Obsidian could remain a useful export/viewer, but the requested live integrations, browser capture, background jobs, native PDF tracking, and lazy-loaded visual graph are better served by a dedicated application.

---

# 2. Product philosophy

## 2.1 “No notes” means typed records

The system should not expose a generic blank note editor. Everything the user creates should have an explicit type and purpose:

- Project
- Area
- Milestone
- Task
- Research session
- Research log entry
- Resource
- Reading session
- Reading log entry
- Daily startup
- Collection
- Topic
- Person
- Decision
- Question

A research log can still contain prose, but it is a **structured event**, not a freestanding note. For example:

```text
Research log
Project: Combining Evidence in LLMs
Milestone: Complete initial ablation
Started: 2026-09-23 09:30
Duration: 55 minutes
Activity: Experiment
Summary: Ran the first conflict-ratio sweep.
Result: Accuracy deteriorates sharply above 0.4 conflict.
Next action: Inspect the three disagreement cases.
Resources: [experiment-output.csv, paper-17]
```

That addresses the existing vault’s fragmented logging. At present, a project log may be only an isolated dated bullet, while reading activity is recorded separately and informally. Work/P-CombiningEvidenceInLLMs/Dive/Logs/10-13-2025.md Work/D-AIAndComputing/Dive/Reading Log.md:1-3

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
- A project must have at least one active or planned milestone.
- Tasks without a milestone go to a temporary **Clarify** queue.
- A milestone describes a result, not an activity.
- Completed projects become read-only unless reopened.
- References and reading items are related to a project or milestone but do not live inside its task hierarchy.

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

Avoid separate navigation destinations for tasks, milestones, reading logs, and integrations. These belong inside the relevant contexts.

## 3.2 Today: the homepage

The dashboard should be the homepage, but it should be action-oriented rather than a complete report.

Recommended order:

### Header

- Date and time.
- Current academic role.
- A concise personalized positive message.
- Global capture button.
- “Resume” button for the most recently active research session.

### Daily startup

- Today’s generated checklist.
- Progress ring.
- Estimated total duration.
- “Skip with reason” rather than forcing completion.
- Add/remove/reorder controls.
- Completed startup instances become immutable daily records.

### Continue research

Show the three most recent research logs, each with:

- Project and milestone.
- When the session ended.
- Last result.
- Explicit “next action.”
- “Resume session” button.

This is much more useful than showing merely the most recently edited records.

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
- Carry forward, dismiss, or revise.
- Record a short day outcome.
- Archive the startup instance.
- Identify tomorrow’s first research action.

The existing dashboard already contains projects, milestones, global tasks, and book progress, so this is an evolution of its intent rather than an unrelated replacement. Dashboard.md:1-18 Dashboard.md:22-40

---

# 4. Daily startup design

## 4.1 Templates versus daily instances

Do not ask an AI to invent the checklist every morning.

Use:

- **Role templates**: PhD student, undergraduate, lecturer, independent researcher, etc.
- **User rules**: weekdays, teaching days, deadlines, preferred work hours.
- **Daily context**: calendar, overdue milestones, unfinished shutdown actions.
- **Generated instance**: a frozen checklist for a particular date.

AI can suggest revisions, but generation should be deterministic and explainable.

## 4.2 Example PhD startup template

```text
Daily startup — PhD researcher

[ ] Review today's calendar and fixed commitments
[ ] Review the active milestone and its completion criterion
[ ] Read the previous research log's “next action”
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

Persist two objects:

```text
StartupTemplate
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

Archive completed or expired instances automatically, but keep them queryable through Review. Do not represent every daily checklist as a manually managed folder.

---

# 5. Work model and lower-cognitive-load structure

## 5.1 Work screen

Offer four views over the same underlying records:

- **Focus**: only active projects and their current milestones.
- **Board**: projects by status.
- **Timeline**: milestone dates and dependencies.
- **Archive**: completed, cancelled, and paused projects.

Default to Focus.

## 5.2 Project screen

A project page should contain:

```text
Project title
Outcome / definition of done
Status and target date
Area
Current milestone
Next action

Milestones
Research activity
Related library resources
Decisions and open questions
```

Do not reproduce separate `Milestones.md`, `Tasks.md`, `Reading.md`, `References.md`, and `Roadmap.md` screens. Those are database-filtered views within one project.

## 5.3 Optional disk export

If users want a human-readable export, generate:

```text
Export/
├── Work/
│   ├── Areas/
│   │   └── AI and Computing.md
│   ├── Projects/
│   │   └── Combining Evidence in LLMs.md
│   └── Archive/
├── Logs/
│   ├── Research/
│   │   └── 2026/
│   └── Reading/
│       └── 2026/
├── Library/
│   └── Library Index.md
└── Daily/
    └── 2026/
```

This removes `Dive` and repeated subordinate files while remaining Obsidian-link-safe.

---

# 6. Library design

## 6.1 Unified resource model

All source types should normalize into one `Resource` model:

```text
Resource
- id
- source_type
- source_account
- external_id
- canonical_url
- local_path
- title
- authors[]
- publication_date
- resource_type
- abstract_or_description
- page_count_or_duration
- thumbnail
- topics[]
- difficulty
- prerequisites[]
- reading_state
- progress
- project_links[]
- source_metadata
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

Do not invent another bibliography manager in the first release. Integrate **Zotero** as the principal academic-library source, then enrich entries through DOI, Crossref, OpenAlex, arXiv, PubMed, or Semantic Scholar identifiers where permitted.

“Academic literature” must be narrowed during discovery to one of:

- A Zotero library.
- A directory of PDFs.
- Saved DOI/arXiv identifiers.
- A reference-manager export.
- A combination of these.

## 6.3 Keeping the library alive

Use an ingestion pipeline:

```text
Connector scan
→ raw source event
→ metadata normalization
→ duplicate resolution
→ text/metadata extraction
→ topic classification
→ level recommendation
→ search and graph indexing
→ UI notification
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

## 6.4 Lazy loading

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
- Connected papers, books, videos, datasets, and research logs.
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
- current_value
- total_value
- normalized_percent
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

Use the most reliable available source:

1. DOI, ISBN, arXiv ID, PMID, or other persistent identifier.
2. Embedded JSON-LD/schema.org metadata.
3. Citation meta tags.
4. Open Graph metadata.
5. Browser page title and visible byline.
6. User correction.

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

[Save progress]
```

Saving should create:

1. Or update the resource.
2. A progress event.
3. A reading-log event if a takeaway or session detail was provided.

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

Do not send research-log bodies, paper text, or sensitive project content unless the user opts in.

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

Minimum first-release schema:

```text
UserProfile
Area
Project
Milestone
Task

ResearchSession
ResearchLogEntry
DailyStartupTemplate
DailyStartupInstance
DailyStartupItem

Resource
ResourceSource
ResourceIdentifier
ResourceTopic
Collection
CollectionItem

ReadingProgressEvent
ReadingLogEntry

ConnectorAccount
SyncCursor
IngestionJob
ImportConflict

Topic
TopicRelationship
ResourceRelationship
```

Important constraints:

- `Task.milestone_id` is required for actionable tasks.
- `Milestone.project_id` is required.
- `Project.area_id` is required for active projects.
- Progress events are append-only.
- Current progress is derived or transactionally updated from events.
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
- Define what “academic literature” means for the first connector.
- Inventory the Google Drive library.
- Select five representative PDFs, webpages, videos, and books.
- Prototype the Today page.
- Prototype list and cluster library views.
- Test metadata extraction against representative sources.
- Test foreground-window detection with the user’s actual PDF viewers.
- Produce a privacy and threat model.

Exit criterion:

- The core workflows can be demonstrated with clickable prototypes and test data.

## Phase 1 — Structured work core

**Goal:** replace folders and generic notes with typed academic-work objects.

Deliverables:

- SQLite schema and migrations.
- Area → Project → Milestone → Task hierarchy.
- Work Focus view.
- Project view.
- Research-session and research-log capture.
- Global capture dialog.
- Search.
- JSON backup and restore.
- Read-only importer for the existing NewSecondBrain structure.

Exit criterion:

- The user can manage one real project for a week without using `Tasks.md` or `Milestones.md`.

## Phase 2 — Today dashboard

Deliverables:

- Role-based startup templates.
- Daily instance generation and archive.
- Continue-research card.
- Active milestone cards.
- Reading-now card.
- Recent reading feed.
- End-of-day close workflow.
- Rule-based positive messages.
- Optional AI message generation.

Exit criterion:

- Startup and shutdown work reliably offline, and no daily data is lost if AI is unavailable.

## Phase 3 — Library foundation

Deliverables:

- Unified resource model.
- Manual import.
- Local PDF folder connector.
- Google Drive connector.
- Zotero connector or chosen academic source.
- YouTube playlist connector.
- Metadata review queue.
- Deduplication.
- Incremental synchronization.
- List view with virtual scrolling.
- Full-text and metadata search.

Exit criterion:

- Adding an item to each configured source results in one deduplicated library record without a full recrawl.

## Phase 4 — Reading capture

Deliverables:

- Browser extension.
- Metadata extraction.
- Gradient progress control.
- Reading-log capture.
- Offline queue.
- Dashboard updates.
- Optional Markdown/JSON disk export.

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
- Wikilinks between exported projects, resources, and logs.
- Link validation.
- Incremental export.
- Existing-vault migration report.
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
| `Dive/Logs/*` | Research log |
| `Scratch/*` | Import-review item; manually classify |
| Project `README.md` frontmatter | Project metadata |

The existing project README already contains fields such as `name`, `kind`, and `type`, making it a useful migration anchor. Work/P-CombiningEvidenceInLLMs/README.md:1-8

## 16.2 Migration safeguards

The importer should:

1. Operate read-only.
2. Produce a dry-run report.
3. Detect duplicate project names and inconsistent `README.md` casing.
4. Preserve original paths.
5. Preserve completion dates and tags.
6. Flag tasks with no obvious milestone.
7. Flag conflict markers and malformed Markdown.
8. Never delete or rewrite the original vault.
9. Allow the same migration to be rerun idempotently.

A dry run is particularly important because at least one current task file contains unresolved Git conflict markers, so permissive parsing and a review queue will be necessary. Work/P-CombiningEvidenceInLLMs/Dive/Tasks.md:28-49

## 16.3 Milestone migration

Existing tasks cannot safely be assigned to milestones automatically merely because both occur inside one project.

Use:

- Heading proximity.
- Shared tags.
- Semantic similarity.
- Dates.
- Manual confirmation.

Uncertain tasks should enter **Clarify**, never be silently attached to a guessed milestone.

---

# 17. Testing strategy

## Domain tests

- Tasks cannot become active without a milestone.
- Milestones cannot belong to multiple projects.
- Progress remains between 0 and 100.
- Completion creates the correct timestamps.
- Imported metadata cannot overwrite user corrections.
- Source deletion preserves logs and progress.

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

3. **Academic library source**
   - Zotero?
   - Google Drive PDFs?
   - A BibTeX library?
   - Papers saved on publisher sites?
   - All of the above?

4. **Deployment**
   - Entirely local?
   - Optional private cloud synchronization?
   - Multi-device use required in the first release?

5. **Obsidian’s role**
   - Backup/export only.
   - Parallel user interface.
   - Primary interface.
   - Eventual retirement.

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
- Research session logging with a mandatory next action.
- Daily startup and archive.
- Today dashboard.
- Unified resource records.
- Local PDFs plus one cloud connector.
- Browser extension capture.
- Reading progress and reading-log cards.
- List view.
- JSON backup and restore.

Defer:

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

## Planning verification

- ✅ `find .. -name AGENTS.md -print` — located the applicable repository instructions.
- ✅ `find . -maxdepth 2 -path './.git' -prune -o -type f -print | sort` — inspected the top-level vault structure without modifying it.
- ✅ `find . -path ./.git -prune -o -type f -print | sort` — inventoried the vault content and available attachments while excluding `.git/`.
- ✅ `nl -ba Dashboard.md` — examined the existing dashboard, project queries, milestones, task aggregation, and reading-progress block.
- ✅ `nl -ba README.md` — examined the documented `Work`/`Dive` organization and intended vault philosophy.
- ✅ `nl -ba .scripts/make_work_item.py` — examined the current project scaffolding behavior.
- ✅ `rg -n --glob '*.md' '^(---|name:|kind:|type:|status:|# |## |[-*] \[[ xX]\]|```dataview|```tasks|```apb|#milestone)' Dashboard.md README.md Lists Work` — sampled headings, frontmatter, tasks, milestones, and dynamic queries.
- ✅ `git status --short --branch` — confirmed that planning did not modify the working tree.
- ⚠️ Internet research for current third-party platform capabilities was attempted, but the configured web-search service returned `401 Unauthorized`; consequently, this plan does not make a time-sensitive claim that an existing commercial product already supplies the entire workflow.

No files were changed, no commit was created, and no pull request was opened, in accordance with the instruction to **only plan**.