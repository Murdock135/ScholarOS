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

[← Previous](03-information-architecture.md) · [Plan index](README.md) · [Next →](05-work-model.md)
