import { beforeEach, describe, it, expect, vi } from "vitest";
import { Editor } from "../apps/desktop/src/editor";
import type { Note, View, Command } from "../apps/desktop/src/api";
const note: Note = {
  id: "note-a",
  kind: "scratch",
  body: "",
  revision: 1,
  created_at: 0,
  updated_at: 0,
};
const view = (body: string, revision: number) =>
  ({ notes: [{ ...note, body, revision }] }) as View;
beforeEach(() => {
  const data = new Map<string, string>();
  vi.stubGlobal("localStorage", {
    get length() {
      return data.size;
    },
    clear: () => data.clear(),
    getItem: (k: string) => data.get(k) ?? null,
    setItem: (k: string, v: string) => data.set(k, v),
    removeItem: (k: string) => data.delete(k),
  });
});
describe("durable editor coordination", () => {
  it("serializes edits made while a save is in flight", async () => {
    let resolve!: (v: View) => void;
    const send = vi
      .fn<(c: Command) => Promise<View>>()
      .mockImplementationOnce(() => new Promise((r) => (resolve = r)))
      .mockResolvedValueOnce(view("second", 3));
    const e = new Editor("a", note, 0, send, () => {});
    e.edit("first", 5);
    const saving = e.flush();
    e.edit("second", 6);
    resolve(view("first", 2));
    await saving;
    expect(send.mock.calls[1][0]).toMatchObject({
      body: "second",
      revision: 2,
      context_id: "a",
      note_id: "note-a",
    });
    expect(e.dirty).toBe(false);
    expect(e.status).toBe("Saved locally");
    expect(localStorage.length).toBe(0);
    e.dispose();
  });
  it("retains a failed draft through reopening and retries without overwriting the base revision", async () => {
    const send = vi.fn().mockRejectedValue(Error("Write failed"));
    const e = new Editor("a", note, 0, send, () => {});
    e.edit("unsaved", 7);
    await expect(e.flush()).rejects.toThrow();
    e.dispose();
    const retry = vi.fn().mockResolvedValue(view("unsaved", 2));
    const reopened = new Editor("a", note, 0, retry, () => {});
    expect(reopened.body).toBe("unsaved");
    expect(reopened.cursor).toBe(7);
    await reopened.flush();
    expect(retry.mock.calls[0][0].revision).toBe(1);
    expect(localStorage.length).toBe(0);
  });
  it("keeps journals isolated by context and does not treat restored conflicts as saved", async () => {
    const fail = vi.fn().mockRejectedValue(Error("Conflict"));
    const e = new Editor("a", note, 0, fail, () => {});
    e.edit("my draft", 3);
    await expect(e.flush()).rejects.toThrow();
    e.dispose();
    const b = new Editor("b", note, 0, fail, () => {});
    expect(b.body).toBe("");
    const re = new Editor(
      "a",
      { ...note, body: "other writer", revision: 2 },
      0,
      fail,
      () => {},
    );
    expect(re.revision).toBe(1);
    await expect(re.flush()).rejects.toThrow();
    expect(re.body).toBe("my draft");
    expect(re.dirty).toBe(true);
    re.dispose();
  });
});
