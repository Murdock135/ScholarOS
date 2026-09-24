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
- Live-file reconciliation, backup/restore, and structured export preserve note bodies, file identities, context membership, shared identities, archived revisions, and resolvable links.
- Existing SQLite-only notes materialize once without duplication; external Markdown edits become revisions; missing files enter recovery without erasing saved content; concurrent edits retain both drafts.
- Carryover creates no duplicate recurring items and does not mutate archived snapshots.
- Invalid template edits preserve the last valid template. Finishing the checklist does not lock the day; closure does. Timezone changes, refreshes, and template edits do not duplicate occurrences.
- Dragging milestone markers does not complete work. Task changes recalculate unaccepted milestones; explicitly accepted milestones stay complete until reopened. Completing the final milestone does not violate the project activation rule.
- Reimports preserve source notes and corrections; ambiguous matches and distinct versions are not silently merged.

## Workflow acceptance checks

| Scenario | Required behavior |
|---|---|
| Edit a scratch note, switch projects, restart | Restore the committed Markdown file in its original project; preserve pending edits on file or database failure. |
| Edit the same note externally while ScholarOS has a draft | Detect the newer file revision, block silent overwrite, and retain both versions for recovery. |
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
- External Markdown changes appear within two seconds with the desktop running, without rescanning unchanged files.
- Opening the cluster view does not load full document bodies.
- Incremental synchronization does not scan every Drive item.
- Browser capture confirmation appears in under 500 ms when the desktop service is running.

---

[← Previous](16-migration.md) · [Plan index](README.md) · [Next →](18-privacy-security.md)
