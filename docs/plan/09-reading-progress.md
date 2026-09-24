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

[← Previous](08-material-levels.md) · [Plan index](README.md) · [Next →](10-browser-extension.md)
