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
- `ResearchNote` stores ID, mutable live file path, optional title, content hash/revision, `kind: scratch | log`, created/edited times, and an optional activity date for Logs. The stable ID remains authoritative when the path changes. Its freeform body lives in UTF-8 Markdown; committed revision snapshots support recovery and completed-project views. Blank drafts are allowed.
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

[← Previous](13-technical-architecture.md) · [Plan index](README.md) · [Next →](15-delivery-roadmap.md)
