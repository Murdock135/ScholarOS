import type { Note, View, Command } from "./api";
interface Draft {
  body: string;
  revision: number;
  cursor: number;
}
// A recovery journal, never the authoritative note store. Acknowledgement follows SQLite commit.
export class Editor {
  body: string;
  revision: number;
  cursor: number;
  savedBody: string;
  status = "Saved locally";
  error = "";
  private pending: Promise<void> | null = null;
  private timer: ReturnType<typeof setTimeout> | undefined;
  readonly key: string;
  constructor(
    readonly context: string,
    readonly note: Note,
    cursor: number,
    private send: (c: Command) => Promise<View>,
    private changed: () => void,
    private storage: Storage = localStorage,
  ) {
    this.key = `scholaros:draft:${context}:${note.id}`;
    this.body = this.savedBody = note.body;
    this.revision = note.revision;
    this.cursor = cursor;
    try {
      const raw = storage.getItem(this.key);
      if (raw) {
        const draft: Draft = JSON.parse(raw);
        if (
          typeof draft.body !== "string" ||
          !Number.isInteger(draft.revision) ||
          !Number.isInteger(draft.cursor)
        )
          throw Error("Invalid recovery journal");
        if (draft.body !== note.body) {
          this.body = draft.body;
          this.revision = draft.revision;
          this.cursor = draft.cursor;
          this.status = "Recovered draft · not yet saved";
        } else storage.removeItem(this.key);
      }
    } catch (e) {
      this.error = `Recovery journal unavailable: ${String(e)}`;
    }
  }
  get dirty() {
    return this.body !== this.savedBody;
  }
  edit(body: string, cursor: number) {
    this.body = body;
    this.cursor = cursor;
    this.status = "Unsaved changes";
    this.error = "";
    this.journal();
    this.changed();
    clearTimeout(this.timer);
    this.timer = setTimeout(() => {
      void this.flush().catch(() => {});
    }, 450);
  }
  private journal() {
    try {
      this.storage.setItem(
        this.key,
        JSON.stringify({
          body: this.body,
          revision: this.revision,
          cursor: this.cursor,
        }),
      );
    } catch {
      this.error =
        "Recovery storage is unavailable. Keep this window open until the note is saved.";
    }
  }
  flush(): Promise<void> {
    clearTimeout(this.timer);
    if (this.pending) return this.pending;
    this.pending = this.save().finally(() => {
      this.pending = null;
    });
    return this.pending;
  }
  private async save() {
    try {
      while (this.dirty) {
        const body = this.body;
        this.status = "Saving…";
        this.changed();
        const v = await this.send({
          type: "save_note",
          context_id: this.context,
          note_id: this.note.id,
          body,
          revision: this.revision,
          cursor: this.cursor,
        });
        const saved = v.notes.find((n) => n.id === this.note.id);
        // Saving is scoped to the open context; navigation is serialized behind flush.
        if (!saved)
          throw Error(
            "Saved note missing from the current context. Draft retained.",
          );
        this.revision = saved.revision;
        this.savedBody = body;
        if (this.dirty) this.journal();
      }
      this.storage.removeItem(this.key);
      this.status = "Saved locally";
      this.error = "";
      this.changed();
    } catch (e) {
      this.status = "Not saved";
      this.error = String(e);
      this.journal();
      this.changed();
      throw e;
    }
  }
  forgetRecoveredDraft() {
    clearTimeout(this.timer);
    this.storage.removeItem(this.key);
  }
  dispose() {
    clearTimeout(this.timer);
  }
}
