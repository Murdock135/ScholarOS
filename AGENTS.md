# ScholarOS agent guidance

## Product context

ScholarOS is a local-first academic work application. Read the `plan.md` entry point and the relevant numbered files under `docs/plan/` before changing product behavior or architecture, then inspect the relevant implementation and documentation. The plan is a working specification; proposed interactions and unresolved discovery questions are not settled decisions.

The proposed stack is Tauri, React/TypeScript, Rust, and SQLite. Confirm the actual repository tooling before choosing commands or introducing dependencies. Do not scaffold the entire roadmap for a narrowly scoped task.

## Product invariants

- Each Project and Area has its own Scratchpad and Logs. Opening either shows only explicitly associated notes in that section. A global notes feed is not the default working surface.
- Notes are freeform. Titles, templates, milestones, next actions, and timed sessions are optional. New notes inherit the open context.
- Sessions and note ownership are independent. A multi-project session must not expose unrelated notes in a project's Scratchpad.
- Shared notes have one editable body with explicit context links. Archived contexts preserve their saved revisions.
- Actionable Tasks belong to Milestones; Milestones belong to Projects. Unassigned task captures remain in Clarify. Draft projects can exist before their hierarchy is complete.
- Unfinished startup items persist according to their carryover settings without duplication. Closed daily snapshots remain unchanged.
- Reading logs, imported source annotations, and research notes remain distinct. Linking a resource to a project does not populate its Scratchpad automatically.
- Preserve user corrections, source provenance, and user activity during imports, merges, and source deletions. Imports never modify the original vault or reference library.
- Core work, notes, startup, and manual reading must function offline without AI. External AI use requires the user's configured consent for that provider and context.

## Implementation workflow

1. Read applicable instructions and inspect the working tree. Preserve unrelated user changes.
2. Implement the smallest complete workflow that satisfies the request. Keep UI behavior, domain rules, storage, and migrations consistent.
3. Keep project/area scope explicit in queries and commands. Enforce data invariants in application services and storage, not only UI controls.
4. Use versioned database migrations. Protect saved data, preserve recoverable drafts, and make import/capture retries idempotent.
5. Run checks appropriate to the change using the repository's actual scripts. Use `npm run check` for the complete Linux suite when warranted. Test consequential behavior and failure paths; avoid tests that merely duplicate implementation details. Documentation-only edits need document/link checks, not application tests.
6. Update affected documentation and write a concise work log for substantive work. Report what changed, validation performed, and remaining limitations honestly.

Do not invent test commands or report unrun checks as passing. Keep secrets and provider tokens out of the repository, logs, and portable backups. Keep exported data and generated indexes separate from authoritative application state.

## Decisions and proposals

- `plan.md`: stable plan entry point and topic index.
- `docs/plan/`: the numbered product specification, roadmap, constraints, and unresolved decisions.
- `docs/proposals/`: unresolved designs, alternatives, and experiments. Use a proposal when a meaningful choice needs discussion or validation.
- `docs/adr/`: durable records of consequential architectural decisions, their rationale, and tradeoffs.
- `logs/`: concise agent work summaries and handoff notes, not a second source of product requirements.

Use the directory READMEs and templates. Routine implementation details do not need an ADR or proposal. Continue authorized work without adding an approval ceremony; document routine assumptions and raise only consequential unresolved choices. Do not label a proposal accepted or claim user agreement without supporting authorization. Keep the plan and accepted ADRs consistent when scope changes. If they conflict, investigate the decision history and flag unresolved conflicts instead of silently choosing one.

## Scope and delivery

Prefer a working vertical slice over broad scaffolding. Initial work should prove project-local notes, persistence, hierarchy, and backup/restore. Native PDF adapters, graph features, optional connectors, AI, and Markdown export must not block the core workflow. Treat operating-system targets and unverified provider capabilities as discovery items until resolved.
