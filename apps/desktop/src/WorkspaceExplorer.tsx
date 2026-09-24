import type { Context, Section, WorkspaceFile } from "./api";

const sectionName = (section: "scratch" | "log") =>
  section === "scratch" ? "Scratchpad" : "Logs";
const fileName = (path: string) => path.split("/").at(-1) ?? path;
const localPath = (file: WorkspaceFile) => {
  const marker = file.kind === "scratch" ? "/Scratchpad/" : "/Logs/";
  const index = file.relative_path.indexOf(marker);
  return index < 0
    ? fileName(file.relative_path)
    : file.relative_path.slice(index + marker.length);
};

interface Branch {
  folders: Map<string, Branch>;
  files: WorkspaceFile[];
}

function buildTree(files: WorkspaceFile[]) {
  const root: Branch = { folders: new Map(), files: [] };
  for (const file of files) {
    const parts = localPath(file).split("/");
    let branch = root;
    for (const folder of parts.slice(0, -1)) {
      let child = branch.folders.get(folder);
      if (!child) {
        child = { folders: new Map(), files: [] };
        branch.folders.set(folder, child);
      }
      branch = child;
    }
    branch.files.push(file);
  }
  return root;
}

function Files({
  branch,
  active,
  onOpen,
}: {
  branch: Branch;
  active?: string;
  onOpen: (file: WorkspaceFile) => void;
}) {
  return (
    <>
      {[...branch.folders.entries()].map(([folder, child]) => (
        <details className="tree-folder nested" key={folder} open>
          <summary>
            <span>›</span>
            <i>▱</i>
            {folder}
          </summary>
          <div className="tree-children">
            <Files branch={child} active={active} onOpen={onOpen} />
          </div>
        </details>
      ))}
      {branch.files.map((file) => (
        <button
          className={`tree-file ${active === file.note_id ? "active" : ""}`}
          key={file.note_id}
          title={file.relative_path}
          aria-label={fileName(file.relative_path)}
          onClick={() => onOpen(file)}
        >
          <b>M↓</b>
          <span>{fileName(file.relative_path)}</span>
        </button>
      ))}
    </>
  );
}

export function WorkspaceExplorer({
  contexts,
  files,
  activeContext,
  activeSection,
  activeNote,
  onOpenSection,
  onOpenFile,
}: {
  contexts: Context[];
  files: WorkspaceFile[];
  activeContext?: string;
  activeSection: Section;
  activeNote?: string;
  onOpenSection: (context: string, section: Section) => void;
  onOpenFile: (file: WorkspaceFile) => void;
}) {
  const section = (owner: Context, kind: "scratch" | "log") => {
    const matches = files.filter(
      (file) => file.context_id === owner.id && file.kind === kind,
    );
    return (
      <details
        className="tree-folder"
        key={`${owner.id}-${kind}`}
        open={owner.id === activeContext}
      >
        <summary onClick={() => onOpenSection(owner.id, kind)}>
          <span>›</span>
          <i>▱</i>
          {sectionName(kind)}
          <small>{matches.length || ""}</small>
        </summary>
        <div className="tree-children">
          <Files
            branch={buildTree(matches)}
            active={activeNote}
            onOpen={onOpenFile}
          />
          {!matches.length && <em>No Markdown files</em>}
        </div>
      </details>
    );
  };

  const context = (owner: Context, nested = false) => {
    const projects =
      owner.kind === "area"
        ? contexts.filter(
            (item) => item.kind === "project" && item.area_id === owner.id,
          )
        : [];
    return (
      <details
        className={`tree-context ${nested ? "nested-context" : ""}`}
        key={owner.id}
        open
      >
        <summary onClick={() => onOpenSection(owner.id, owner.last_section)}>
          <span>›</span>
          <i>{owner.kind === "area" ? "◇" : "○"}</i>
          {owner.name}
        </summary>
        <div className="tree-children">
          {section(owner, "scratch")}
          {section(owner, "log")}
          {owner.kind === "project" && (
            <button
              className={`tree-special ${owner.id === activeContext && activeSection === "milestones" ? "active" : ""}`}
              onClick={() => onOpenSection(owner.id, "milestones")}
            >
              <b>⌁</b>
              <span>Milestones</span>
            </button>
          )}
          {!!projects.length && (
            <div className="project-group">
              <div className="tree-group-label">PROJECTS</div>
              {projects.map((project) => context(project, true))}
            </div>
          )}
        </div>
      </details>
    );
  };

  const areas = contexts.filter((item) => item.kind === "area");
  const drafts = contexts.filter(
    (item) => item.kind === "project" && !item.area_id,
  );
  return (
    <nav className="workspace-tree" aria-label="Workspace files">
      {areas.map((area) => context(area))}
      {!!drafts.length && (
        <div className="tree-group-label">DRAFT PROJECTS</div>
      )}
      {drafts.map((project) => context(project))}
    </nav>
  );
}
