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
→ validated SQLite transaction for resources and reading events
→ Markdown write only when the user explicitly captures a research note
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

[← Previous](09-reading-progress.md) · [Plan index](README.md) · [Next →](11-native-pdf.md)
