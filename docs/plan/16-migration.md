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

1. Read the source vault without modification; write imported notes to the live workspace and structured records to SQLite through one idempotent import transaction.
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

[← Previous](15-delivery-roadmap.md) · [Plan index](README.md) · [Next →](17-testing.md)
