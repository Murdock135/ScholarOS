import { useEffect, useRef, useState } from "react";
import { save } from "@tauri-apps/plugin-dialog";
import { getCurrentWindow } from "@tauri-apps/api/window";
import {
  execute,
  exportBackup,
  openWorkspace,
  previewBackup,
  restoreBackup,
  type View,
  type Section,
  type Preview,
} from "./api";
import { Editor } from "./editor";

const sectionName = (s: Section) =>
  ({ scratch: "Scratchpad", log: "Logs", milestones: "Milestones" })[s];
function NameForm({
  label,
  onSubmit,
}: {
  label: string;
  onSubmit: (name: string) => Promise<void>;
}) {
  const [value, setValue] = useState("");
  const [working, setWorking] = useState(false);
  return (
    <form
      className="name-form"
      onSubmit={async (e) => {
        e.preventDefault();
        if (!value.trim()) return;
        setWorking(true);
        try {
          await onSubmit(value);
          setValue("");
        } finally {
          setWorking(false);
        }
      }}
    >
      <input
        aria-label={label}
        placeholder={label}
        value={value}
        maxLength={200}
        onChange={(e) => setValue(e.target.value)}
        required
      />
      <button disabled={working || !value.trim()} type="submit">
        Add
      </button>
    </form>
  );
}
export default function App() {
  const [view, setView] = useState<View | null>(null),
    [error, setError] = useState(""),
    [busy, setBusy] = useState(false),
    [tick, redraw] = useState(0);
  const [creator, setCreator] = useState<"area" | "project" | null>(null),
    [area, setArea] = useState("");
  const [backup, setBackup] = useState<{
      json: string;
      preview: Preview;
    } | null>(null),
    [notice, setNotice] = useState("");
  const editor = useRef<Editor | null>(null),
    textarea = useRef<HTMLTextAreaElement>(null),
    lock = useRef(false),
    file = useRef<HTMLInputElement>(null);
  const refresh = () => redraw((t) => t + 1);
  function adopt(v: View) {
    editor.current?.dispose();
    const note =
      v.notes.find((n) => n.id === v.selection?.note_id) ?? v.notes[0];
    editor.current =
      note && v.workspace.context_id
        ? new Editor(
            v.workspace.context_id,
            note,
            v.selection?.note_id === note.id ? v.selection.cursor : 0,
            execute,
            refresh,
          )
        : null;
    setView(v);
  }
  async function flush() {
    const ed = editor.current;
    if (!ed) return;
    const wasDirty = ed.dirty;
    await ed.flush();
    // A successful note save already persists the cursor. For a clean editor,
    // reconcile first so a move or deletion outside ScholarOS is not rejected
    // as a stale selection during navigation.
    if (wasDirty) return;
    const current = await execute({ type: "view" });
    if (!current.notes.some((note) => note.id === ed.note.id)) return;
    await execute({
      type: "select_note",
      context_id: ed.context,
      section: ed.note.kind,
      note_id: ed.note.id,
      cursor: ed.cursor,
    });
  }
  async function action(work: () => Promise<void>, saveFirst = true) {
    if (lock.current) return;
    lock.current = true;
    setBusy(true);
    setError("");
    try {
      if (saveFirst) await flush();
      await work();
    } catch (e) {
      setError(String(e));
    } finally {
      lock.current = false;
      setBusy(false);
    }
  }
  useEffect(() => {
    void execute({ type: "view" })
      .then(adopt)
      .catch((e) => setError(String(e)));
    const before = (e: BeforeUnloadEvent) => {
      if (editor.current?.dirty) {
        e.preventDefault();
        e.returnValue = "";
      }
    };
    window.addEventListener("beforeunload", before);
    const focus = () => {
      if (!lock.current)
        void action(async () => adopt(await execute({ type: "view" })));
    };
    window.addEventListener("focus", focus);
    let unlisten: (() => void) | undefined,
      disposed = false;
    if ("__TAURI_INTERNALS__" in window) {
      void getCurrentWindow()
        .onCloseRequested(async (e) => {
          e.preventDefault();
          if (lock.current) return;
          await action(async () => {
            await getCurrentWindow().destroy();
          });
        })
        .then((fn) => {
          if (disposed) fn();
          else unlisten = fn;
        })
        .catch((e) => setError(String(e)));
    }
    return () => {
      disposed = true;
      unlisten?.();
      window.removeEventListener("beforeunload", before);
      window.removeEventListener("focus", focus);
      editor.current?.dispose();
    };
  }, []);
  const ed = editor.current;
  useEffect(() => {
    if (textarea.current && ed) {
      const p = Math.min(ed.cursor, ed.body.length);
      textarea.current.setSelectionRange(p, p);
    }
  }, [view?.workspace.context_id, view?.workspace.section, ed?.note.id]);
  const context = view?.contexts.find(
    (c) => c.id === view.workspace.context_id,
  );
  const section = view?.workspace.section ?? "scratch";
  async function open(context_id: string, s: Section) {
    await action(async () =>
      adopt(await execute({ type: "open", context_id, section: s })),
    );
  }
  async function create(kind: "area" | "project", name: string) {
    await action(async () => {
      adopt(
        await execute({
          type: "create_context",
          kind,
          name,
          area_id: kind === "project" && area ? area : null,
        }),
      );
      setCreator(null);
    });
  }
  async function recoverCopy() {
    if (!ed || !view) return;
    const body = ed.body;
    await action(async () => {
      const v = await execute({
        type: "create_note",
        context_id: ed.context,
        section: ed.note.kind,
      });
      const n = v.notes.find((n) => n.id === v.selection?.note_id)!;
      const saved = await execute({
        type: "save_note",
        context_id: ed.context,
        note_id: n.id,
        body,
        revision: n.revision,
        cursor: ed.cursor,
      });
      ed.forgetRecoveredDraft();
      adopt(saved);
      setNotice("Your draft was saved as a separate recovery copy.");
    }, false);
  }
  return (
    <div className="app">
      <aside className="sidebar">
        <div className="brand">
          <span className="brand-mark">S</span>
          <div>
            ScholarOS<small>A place for your work</small>
          </div>
        </div>
        <div className="nav-heading">
          WORK <span>Local workspace</span>
        </div>
        <div className="create-buttons">
          <button
            onClick={() => {
              setCreator("project");
              setArea(
                context?.kind === "area"
                  ? context.id
                  : (context?.area_id ?? ""),
              );
            }}
          >
            + Project
          </button>
          <button onClick={() => setCreator("area")}>+ Area</button>
        </div>
        {creator && (
          <div className="creation">
            <div className="creation-title">
              New {creator === "project" ? "draft project" : "Area"}
              <button
                aria-label="Cancel creation"
                onClick={() => setCreator(null)}
              >
                ×
              </button>
            </div>
            {creator === "project" && (
              <label>
                Area{" "}
                <select value={area} onChange={(e) => setArea(e.target.value)}>
                  <option value="">Unassigned draft</option>
                  {view?.contexts
                    .filter((c) => c.kind === "area")
                    .map((c) => (
                      <option key={c.id} value={c.id}>
                        {c.name}
                      </option>
                    ))}
                </select>
              </label>
            )}
            <NameForm
              label={creator === "project" ? "Project name" : "Area name"}
              onSubmit={(n) => create(creator, n)}
            />
          </div>
        )}
        <nav aria-label="Workspaces">
          {view?.contexts
            .filter((c) => c.kind === "area")
            .map((a) => (
              <div className="area-group" key={a.id}>
                <button
                  className={`context area ${context?.id === a.id ? "selected" : ""}`}
                  aria-current={context?.id === a.id ? "page" : undefined}
                  disabled={busy}
                  onClick={() => void open(a.id, a.last_section)}
                >
                  <span>▧</span>
                  {a.name}
                </button>
                {view.contexts
                  .filter((c) => c.area_id === a.id)
                  .map((p) => (
                    <button
                      key={p.id}
                      className={`context project ${context?.id === p.id ? "selected" : ""}`}
                      disabled={busy}
                      onClick={() => void open(p.id, p.last_section)}
                    >
                      <span>○</span>
                      {p.name}
                    </button>
                  ))}
              </div>
            ))}
          {!!view?.contexts.some((c) => c.kind === "project" && !c.area_id) && (
            <div className="nav-heading">DRAFT PROJECTS</div>
          )}
          {view?.contexts
            .filter((c) => c.kind === "project" && !c.area_id)
            .map((p) => (
              <button
                key={p.id}
                className={`context ${context?.id === p.id ? "selected" : ""}`}
                disabled={busy}
                onClick={() => void open(p.id, p.last_section)}
              >
                <span>○</span>
                {p.name}
              </button>
            ))}
        </nav>
        <div className="sidebar-footer">
          <span className="offline-dot" />
          Stored on this device
          <div className="backup-actions">
            <button
              disabled={busy || !view}
              onClick={() =>
                void action(async () => {
                  const path = await openWorkspace();
                  setNotice(`Markdown workspace: ${path}`);
                })
              }
            >
              Open folder
            </button>
            <button
              disabled={busy || !view}
              onClick={() =>
                void action(async () => {
                  const path = await save({
                    defaultPath: `ScholarOS-${new Date().toISOString().replace(/[:.]/g, "-")}.json`,
                    filters: [
                      { name: "ScholarOS backup", extensions: ["json"] },
                    ],
                  });
                  if (path) {
                    await exportBackup(path);
                    setNotice(`Backup saved to ${path}`);
                  }
                })
              }
            >
              Back up
            </button>
            <button
              disabled={busy || !view}
              onClick={() => file.current?.click()}
            >
              Restore
            </button>
            <input
              hidden
              ref={file}
              type="file"
              accept=".json,application/json"
              aria-label="Restore backup file"
              onChange={(e) => {
                const f = e.target.files?.[0];
                e.target.value = "";
                if (f)
                  void action(async () => {
                    if (f.size > 100_000_000)
                      throw Error("Backup exceeds 100 MB.");
                    const json = await f.text();
                    setBackup({ json, preview: await previewBackup(json) });
                  });
              }}
            />
          </div>
        </div>
      </aside>
      <main>
        {error && (
          <div role="alert" className="banner error">
            {error}
            <button onClick={() => setError("")} aria-label="Dismiss error">
              ×
            </button>
          </div>
        )}
        {notice && (
          <div role="status" className="banner">
            {notice}
            <button onClick={() => setNotice("")} aria-label="Dismiss notice">
              ×
            </button>
          </div>
        )}
        {backup && (
          <div
            role="dialog"
            aria-modal="true"
            aria-label="Restore workspace"
            className="modal-shade"
          >
            <div className="modal">
              <h2>Replace this workspace?</h2>
              <p>
                Validated backup: {backup.preview.contexts} contexts,{" "}
                {backup.preview.notes} notes, {backup.preview.milestones}{" "}
                milestones, {backup.preview.tasks} tasks.
              </p>
              <p>
                This replaces the current workspace. A rollback backup is saved
                on this device before replacement. No attachments are included
                in this foundation’s backup format.
              </p>
              <button disabled={busy} onClick={() => setBackup(null)}>
                Cancel
              </button>
              <button
                className="primary"
                disabled={busy}
                onClick={() =>
                  void action(async () => {
                    const path = await restoreBackup(backup.json);
                    adopt(await execute({ type: "view" }));
                    setBackup(null);
                    setNotice(
                      `Workspace restored. Previous workspace: ${path}`,
                    );
                  })
                }
              >
                Replace workspace
              </button>
            </div>
          </div>
        )}
        {!context ? (
          <div className="welcome">
            <div className="eyebrow">YOUR RESEARCH, WITH ROOM TO THINK</div>
            <h1>
              Start with a project.
              <br />
              Or just an idea.
            </h1>
            <p>
              Create an Area for an ongoing responsibility, then a draft
              project.
              <br />
              Your notes can begin before the plan does.
            </p>
            <button
              className="primary"
              disabled={!view}
              onClick={() => setCreator("area")}
            >
              Create your first Area
            </button>
            {!view && !error && <p>Opening local workspace…</p>}
          </div>
        ) : (
          <>
            <header className="workspace-header">
              <div className="eyebrow">
                {context.kind === "area"
                  ? "AREA"
                  : (view?.contexts.find((c) => c.id === context.area_id)
                      ?.name ?? "WORK")}{" "}
                <span>/</span>{" "}
                {context.kind === "project"
                  ? "DRAFT PROJECT"
                  : "ONGOING RESPONSIBILITY"}
              </div>
              <h1>{context.name}</h1>
              <div
                className="tabs"
                role="tablist"
                aria-label="Workspace sections"
              >
                {(
                  [
                    "scratch",
                    "log",
                    ...(context.kind === "project" ? ["milestones"] : []),
                  ] as Section[]
                ).map((s) => (
                  <button
                    role="tab"
                    aria-selected={section === s}
                    key={s}
                    disabled={busy}
                    onClick={() => void open(context.id, s)}
                  >
                    {sectionName(s)}
                  </button>
                ))}
              </div>
            </header>
            {section === "milestones" ? (
              <section className="planning">
                <div className="section-intro">
                  <h2>Make room for the next step.</h2>
                  <p>
                    Milestones describe outcomes. Tasks live underneath them.
                  </p>
                </div>
                <NameForm
                  label="New milestone"
                  onSubmit={(n) =>
                    action(async () =>
                      adopt(
                        await execute({
                          type: "create_milestone",
                          context_id: context.id,
                          name: n,
                        }),
                      ),
                    )
                  }
                />
                {view?.milestones.map((m) => (
                  <article className="milestone" key={m.id}>
                    <h3>{m.name}</h3>
                    {view.tasks
                      .filter((t) => t.milestone_id === m.id)
                      .map((t) => (
                        <label
                          className={`task ${t.done ? "done" : ""}`}
                          key={t.id}
                        >
                          <input
                            type="checkbox"
                            checked={t.done}
                            disabled={busy}
                            onChange={(e) => {
                              const done = e.currentTarget.checked;
                              void action(async () =>
                                adopt(
                                  await execute({
                                    type: "set_task",
                                    context_id: context.id,
                                    task_id: t.id,
                                    done,
                                  }),
                                ),
                              );
                            }}
                          />
                          <span>{t.name}</span>
                        </label>
                      ))}
                    <NameForm
                      label={`Add task to ${m.name}`}
                      onSubmit={(n) =>
                        action(async () =>
                          adopt(
                            await execute({
                              type: "create_task",
                              context_id: context.id,
                              milestone_id: m.id,
                              name: n,
                            }),
                          ),
                        )
                      }
                    />
                  </article>
                ))}
              </section>
            ) : (
              <div className="writing-layout">
                <section
                  className="note-list"
                  aria-label={`${context.name} ${sectionName(section)} notes`}
                >
                  <div className="list-heading">
                    <span>
                      {sectionName(section)} <small>{view?.notes.length}</small>
                    </span>
                    <button
                      aria-label="New note"
                      title="New note"
                      disabled={busy}
                      onClick={() =>
                        void action(async () =>
                          adopt(
                            await execute({
                              type: "create_note",
                              context_id: context.id,
                              section,
                            }),
                          ),
                        )
                      }
                    >
                      +
                    </button>
                  </div>
                  {view?.notes.map((n) => {
                    const body = ed?.note.id === n.id ? ed.body : n.body;
                    return (
                      <button
                        key={n.id}
                        className={`note-card ${ed?.note.id === n.id ? "active" : ""}`}
                        disabled={busy}
                        onClick={() =>
                          void action(async () =>
                            adopt(
                              await execute({
                                type: "select_note",
                                context_id: context.id,
                                section,
                                note_id: n.id,
                                cursor: n.id === ed?.note.id ? ed.cursor : 0,
                              }),
                            ),
                          )
                        }
                      >
                        <strong>
                          {body.trim().split("\n")[0]?.slice(0, 65) ||
                            "Untitled note"}
                        </strong>
                        <span>
                          {body
                            .trim()
                            .split("\n")
                            .slice(1)
                            .join(" ")
                            .slice(0, 100) || "A little room to think."}
                        </span>
                        <small>
                          {new Date(n.created_at).toLocaleDateString(
                            undefined,
                            { month: "short", day: "numeric" },
                          )}
                        </small>
                      </button>
                    );
                  })}
                  {!view?.notes.length && (
                    <p className="list-empty">
                      Only this {context.kind}’s{" "}
                      {section === "log" ? "logs" : "scratch notes"} appear
                      here.
                    </p>
                  )}
                </section>
                <section className="editor-pane" aria-label="Note editor">
                  {ed ? (
                    <>
                      <div className="editor-toolbar">
                        <span>
                          {section === "scratch"
                            ? "WORKING NOTE"
                            : "RESEARCH LOG"}
                        </span>
                        <span role="status">{ed.status}</span>
                      </div>
                      {ed.error && (
                        <div role="alert" className="editor-error">
                          {ed.error}
                          <div>
                            <button
                              onClick={() =>
                                void action(async () => {
                                  await ed.flush();
                                }, false)
                              }
                            >
                              Retry save
                            </button>
                            <button onClick={() => void recoverCopy()}>
                              Save as recovery copy
                            </button>
                          </div>
                        </div>
                      )}
                      <textarea
                        key={ed.note.id}
                        ref={textarea}
                        aria-label="Note body"
                        spellCheck
                        placeholder="Start anywhere. This space is yours."
                        value={ed.body}
                        disabled={busy}
                        onChange={(e) =>
                          ed.edit(e.target.value, e.target.selectionStart)
                        }
                        onSelect={(e) => {
                          ed.cursor = e.currentTarget.selectionStart;
                        }}
                      />
                      <div className="editor-footer">
                        <span>
                          {ed.body.trim()
                            ? ed.body.trim().split(/\s+/).length
                            : 0}{" "}
                          words
                        </span>
                        <span>No title or structure required</span>
                      </div>
                    </>
                  ) : (
                    <div className="empty-editor">
                      <span className="empty-glyph">✎</span>
                      <h2>
                        {section === "scratch"
                          ? "Space to work things out."
                          : "Keep a record, in your words."}
                      </h2>
                      <p>
                        A blank note. No forms, timers, or required structure.
                      </p>
                      <button
                        className="primary"
                        disabled={busy}
                        onClick={() =>
                          void action(async () =>
                            adopt(
                              await execute({
                                type: "create_note",
                                context_id: context.id,
                                section,
                              }),
                            ),
                          )
                        }
                      >
                        New note
                      </button>
                    </div>
                  )}
                </section>
              </div>
            )}
          </>
        )}
      </main>
      <span hidden>{tick}</span>
    </div>
  );
}
