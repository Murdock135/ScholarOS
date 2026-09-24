# 6. Library design

## 6.1 Unified resource model

All source types should normalize into one `Resource` model:

```text
Resource
- id
- sources[]: ResourceSource
- canonical_url
- attachments[]: ResourceAttachment
- title
- authors[]
- publication_date
- resource_type
- abstract_or_description
- page_count_or_duration
- thumbnail
- topics[]
- level
- prerequisites[]
- reading_state
- progress
- project_links[]
- first_seen_at

ResourceAttachment
- id
- resource_id
- source_id (optional)
- local_path_or_url
- content_hash
- page_count_or_duration

ResourceSource
- id
- resource_id
- source_type
- source_account
- external_id
- source_metadata
- source_notes[]
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

The user's academic references are distributed across **Excel, Zotero, and Mendeley Web**, with overlap. Support all three as inputs to one canonical library without requiring migration out of the existing tools.

- **Excel:** import XLSX/CSV with a column-mapping preview for title, link, publication date, and notes. Preserve original cell values and source workbook/sheet provenance. Prefer a stable row ID when available; row number alone is not identity because rows can move. Reimports without stable IDs require content comparison and review for ambiguous edits.
- **Zotero:** implement read-only incremental synchronization of metadata, collections, and notes using library/item identities and versions. Attachment access is a separate concern. [Zotero API synchronization documentation](https://www.zotero.org/support/dev/web_api/v3/syncing).
- **Mendeley:** initially use supported reference exports. Current guidance documents reference exports including BibTeX, RIS, and EndNote XML, but does not establish the available export workflow for this user’s web library. Verify that workflow and, if needed, access to the same library through the desktop application during discovery; do not promise direct web synchronization or assume exports preserve every note. [Mendeley export guidance](https://www.elsevier.support/mendeley/answer/how-can-i-export-my-library).

Spreadsheet and reference exports are snapshots that may contain only selected rows/items. Absence from a later import is not a deletion signal. Tombstone source membership only from a provider's explicit deletion event or a successful complete scan of the configured scope; incomplete scans, authentication failures, and filtered exports must not remove records. Every import reports counts, unresolved matches, and fields/notes absent from the source format rather than claiming unavailable content was preserved.

### Overlap and deduplication

One canonical `Resource` can have multiple `ResourceSource` records. Store provider IDs, original metadata, source notes, and import history separately from user corrections.

1. Recognize an already-imported source item by source identity.
2. Match normalized persistent identifiers such as DOI or PMID, checking contradictory metadata before automatic linkage.
3. Use title, author, and year similarity to propose matches for review; title alone is insufficient.
4. Preserve preprints, published versions, editions, and supplements as distinct but related records when appropriate.
5. Preserve notes from all three sources with attribution. Never overwrite an Excel note with a Zotero or Mendeley note.
6. Make merges reversible. One removed source link must not remove the canonical resource's other links, notes, or reading history.

A resource may have multiple files with different pagination; preserve attachment identity rather than treating one local path as definitive. The source records own provider-specific fields such as account, external ID, sync version, and original metadata. The canonical resource owns resolved display metadata and user activity; it must not have only one source slot.

Imported Excel, Zotero, and Mendeley annotations remain attributed `ResourceSourceNote` records in the resource detail view. They are not automatically research notes. A user can explicitly create a Scratchpad note or Log referencing an annotation in a chosen context; the imported original and its provenance remain intact. Resource/project associations do not automatically populate project Scratchpads.

## 6.3 Keeping the library alive

Use an ingestion pipeline:

```text
Connector scan
→ raw source event
→ metadata normalization
→ identifier/metadata extraction
→ duplicate resolution
→ metadata search indexing and usable library record
→ UI notification
→ optional full-text extraction, classification, and graph enrichment
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

## 6.4 Agent-assisted organization — proposed approach

Use connectors to enumerate and synchronize sources, then an optional constrained agent to interpret their content. Drive exposes a changes feed and YouTube exposes playlist-item enumeration; use these interfaces for retrieval rather than asking a model to navigate the source from scratch. [Drive changes](https://developers.google.com/workspace/drive/api/guides/manage-changes), [YouTube playlist items](https://developers.google.com/youtube/v3/docs/playlistItems/list).

Suggested flow:

```text
Selected source → reliable inventory → normalized records → deduplication
                → contextual classification suggestions → library + review queue
```

Give the agent folder ancestry, playlist title/description/order, bibliographic metadata, and bounded text samples when the user permits it. It can suggest topics, resource types, levels, collection membership, and likely duplicates. For example, a course playlist can become a collection preserving lecture order, with inferred topics offered as editable suggestions. Resources may belong to several collections.

Each suggestion stores evidence, rationale, model/version, and confidence. Confidence is a review signal, not proof. Apply deterministic mappings automatically; initially review agent suggestions in batches. Allow later opt-in automation for tested, reversible classifications. Never let the agent delete originals, overwrite user corrections, silently merge uncertain matches, or obey instructions embedded in imported documents.

Cache results by content hash and classification version, process only changed material, cap model usage, and queue failures independently of successful imports. Manual organization and ordinary import must work with AI disabled. Begin with rules and metadata; add bounded agent tools for missing identifiers and ambiguous classifications after the import pipeline works.

## 6.5 Lazy loading

The UI should:

- Fetch library rows in cursor-based pages.
- Load thumbnails only when visible.
- Load full text only when a user opens an item or when a background indexing budget allows.
- Cluster only the visible topic or filtered result set.
- Move graph-layout computation to a Web Worker.
- Render large graphs using WebGL, not thousands of DOM elements.
- Cache recent queries and resource cards.

---

[← Previous](05-work-model.md) · [Plan index](README.md) · [Next →](07-library-views.md)
