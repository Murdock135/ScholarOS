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

[← Previous](01-recommendation.md) · [Plan index](README.md) · [Next →](03-information-architecture.md)
