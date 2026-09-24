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

[← Previous](02-product-philosophy.md) · [Plan index](README.md) · [Next →](04-daily-startup.md)
