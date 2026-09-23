use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};
use uuid::Uuid;

pub type Result<T> = std::result::Result<T, String>;
fn err(e: impl std::fmt::Display) -> String {
    e.to_string()
}
fn id() -> String {
    Uuid::new_v4().to_string()
}
fn now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}
fn name(value: &str) -> Result<&str> {
    let s = value.trim();
    if s.is_empty() || s.chars().count() > 200 {
        Err("Use a name between 1 and 200 characters.".into())
    } else {
        Ok(s)
    }
}

macro_rules! record { ($n:ident { $($f:ident : $t:ty),* $(,)? }) => {
 #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
 #[serde(deny_unknown_fields)]
 pub struct $n { $(pub $f: $t),* }
}; }
record!(Context { id:String, kind:String, name:String, area_id:Option<String>, last_section:String });
record!(Note {
    id: String,
    kind: String,
    body: String,
    revision: i64,
    created_at: i64,
    updated_at: i64
});
record!(Link {
    note_id: String,
    context_id: String
});
record!(Revision {
    note_id: String,
    revision: i64,
    body: String,
    saved_at: i64
});
record!(Selection {
    context_id: String,
    section: String,
    note_id: String,
    cursor: i64
});
record!(Milestone {
    id: String,
    project_id: String,
    name: String
});
record!(Task { id:String, milestone_id:String, name:String, done:bool, completed_at:Option<i64> });
record!(Workspace { context_id:Option<String>, section:String });
record!(Backup { format:String, version:u32, contexts:Vec<Context>, notes:Vec<Note>, links:Vec<Link>, revisions:Vec<Revision>, selections:Vec<Selection>, milestones:Vec<Milestone>, tasks:Vec<Task>, workspace:Workspace });
record!(View { contexts:Vec<Context>, workspace:Workspace, notes:Vec<Note>, selection:Option<Selection>, milestones:Vec<Milestone>, tasks:Vec<Task> });
record!(Preview {
    contexts: usize,
    notes: usize,
    milestones: usize,
    tasks: usize
});

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum Command {
    View,
    CreateContext {
        kind: String,
        name: String,
        area_id: Option<String>,
    },
    Open {
        context_id: String,
        section: String,
    },
    CreateNote {
        context_id: String,
        section: String,
    },
    SelectNote {
        context_id: String,
        section: String,
        note_id: String,
        cursor: i64,
    },
    SaveNote {
        context_id: String,
        note_id: String,
        body: String,
        revision: i64,
        cursor: i64,
    },
    CreateMilestone {
        context_id: String,
        name: String,
    },
    CreateTask {
        context_id: String,
        milestone_id: String,
        name: String,
    },
    SetTask {
        context_id: String,
        task_id: String,
        done: bool,
    },
}

pub struct Store {
    db: Connection,
    directory: PathBuf,
}
impl Store {
    pub fn open(path: &Path) -> Result<Self> {
        let directory = path.parent().unwrap_or(Path::new(".")).to_path_buf();
        fs::create_dir_all(&directory).map_err(err)?;
        let db = Connection::open(path).map_err(err)?;
        configure(&db)?;
        db.pragma_update(None, "journal_mode", "WAL").map_err(err)?;
        migrate(&db)?;
        Ok(Self { db, directory })
    }
    pub fn execute(&mut self, command: Command) -> Result<View> {
        let tx = self.db.transaction().map_err(err)?;
        match command {
            Command::View => {}
            Command::CreateContext {
                kind,
                name: label,
                area_id,
            } => {
                let key = id();
                tx.execute(
                    "INSERT INTO contexts(id,kind,name,area_id) VALUES(?1,?2,?3,?4)",
                    params![key, kind, name(&label)?, area_id],
                )
                .map_err(err)?;
                tx.execute(
                    "UPDATE workspace SET context_id=?1,section='scratch'",
                    [key],
                )
                .map_err(err)?;
            }
            Command::Open {
                context_id,
                section,
            } => {
                require_context(&tx, &context_id, &section)?;
                tx.execute(
                    "UPDATE workspace SET context_id=?1,section=?2",
                    params![context_id, section],
                )
                .map_err(err)?;
                tx.execute(
                    "UPDATE contexts SET last_section=?2 WHERE id=?1",
                    params![context_id, section],
                )
                .map_err(err)?;
            }
            Command::CreateNote {
                context_id,
                section,
            } => {
                require_context(&tx, &context_id, &section)?;
                let key = id();
                let time = now();
                tx.execute(
                    "INSERT INTO notes VALUES(?1,?2,'',1,?3,?3)",
                    params![key, section, time],
                )
                .map_err(err)?;
                tx.execute(
                    "INSERT INTO note_contexts VALUES(?1,?2)",
                    params![key, context_id],
                )
                .map_err(err)?;
                tx.execute(
                    "INSERT INTO revisions VALUES(?1,1,'',?2)",
                    params![key, time],
                )
                .map_err(err)?;
                select(&tx, &context_id, &section, &key, 0)?;
            }
            Command::SelectNote {
                context_id,
                section,
                note_id,
                cursor,
            } => {
                select(&tx, &context_id, &section, &note_id, cursor)?;
            }
            Command::SaveNote {
                context_id,
                note_id,
                body,
                revision,
                cursor,
            } => {
                if body.len() > 10_000_000 {
                    return Err("Note exceeds the 10 MB limit; draft retained.".into());
                }
                let time = now();
                let changed=tx.execute("UPDATE notes SET body=?1,revision=revision+1,updated_at=max(updated_at,?2) WHERE id=?3 AND revision=?4 AND EXISTS(SELECT 1 FROM note_contexts WHERE note_id=?3 AND context_id=?5)",params![body,time,note_id,revision,context_id]).map_err(err)?;
                if changed != 1 {
                    return Err("Note changed elsewhere or is outside this context. Your draft is retained; save it as a recovery copy.".into());
                }
                tx.execute("INSERT INTO revisions SELECT id,revision,body,updated_at FROM notes WHERE id=?1",[&note_id]).map_err(err)?;
                let section: String = tx
                    .query_row("SELECT kind FROM notes WHERE id=?1", [&note_id], |r| {
                        r.get(0)
                    })
                    .map_err(err)?;
                select(&tx, &context_id, &section, &note_id, cursor)?;
            }
            Command::CreateMilestone {
                context_id,
                name: label,
            } => {
                tx.execute(
                    "INSERT INTO milestones VALUES(?1,?2,?3)",
                    params![id(), context_id, name(&label)?],
                )
                .map_err(err)?;
            }
            Command::CreateTask {
                context_id,
                milestone_id,
                name: label,
            } => {
                require_milestone(&tx, &context_id, &milestone_id)?;
                tx.execute(
                    "INSERT INTO tasks VALUES(?1,?2,?3,0,NULL)",
                    params![id(), milestone_id, name(&label)?],
                )
                .map_err(err)?;
            }
            Command::SetTask {
                context_id,
                task_id,
                done,
            } => {
                let changed=tx.execute("UPDATE tasks SET done=?1,completed_at=CASE WHEN ?1 THEN coalesce(completed_at,?2) ELSE NULL END WHERE id=?3 AND milestone_id IN (SELECT id FROM milestones WHERE project_id=?4)",params![done,now(),task_id,context_id]).map_err(err)?;
                if changed != 1 {
                    return Err("Task is outside this project.".into());
                }
            }
        }
        tx.commit().map_err(err)?;
        self.view()
    }
    pub fn view(&self) -> Result<View> {
        let contexts = contexts(&self.db)?;
        let workspace = workspace(&self.db)?;
        let context = workspace.context_id.as_deref().unwrap_or("");
        let section = &workspace.section;
        let notes=query(&self.db,"SELECT n.id,n.kind,n.body,n.revision,n.created_at,n.updated_at FROM notes n JOIN note_contexts l ON l.note_id=n.id WHERE l.context_id=?1 AND n.kind=?2 ORDER BY n.created_at,n.id",params![context,section],note)?;
        let selection=self.db.query_row("SELECT context_id,section,note_id,cursor FROM selections WHERE context_id=?1 AND section=?2",params![context,section],selection).optional().map_err(err)?;
        let milestones = query(
            &self.db,
            "SELECT id,project_id,name FROM milestones WHERE project_id=?1 ORDER BY rowid",
            [context],
            milestone,
        )?;
        let tasks=query(&self.db,"SELECT t.id,t.milestone_id,t.name,t.done,t.completed_at FROM tasks t JOIN milestones m ON m.id=t.milestone_id WHERE m.project_id=?1 ORDER BY t.rowid",[context],task)?;
        Ok(View {
            contexts,
            workspace,
            notes,
            selection,
            milestones,
            tasks,
        })
    }
    pub fn backup(&self) -> Result<String> {
        // One read transaction gives a consistent snapshot even with another connection.
        let tx = self.db.unchecked_transaction().map_err(err)?;
        let data = snapshot(&tx)?;
        let json = serde_json::to_string_pretty(&data).map_err(err)?;
        tx.commit().map_err(err)?;
        Ok(json)
    }
    pub fn export(&self, path: &Path) -> Result<()> {
        atomic_write(path, self.backup()?.as_bytes())
    }
    pub fn preview(json: &str) -> Result<Preview> {
        let (data, _) = stage(json)?;
        Ok(Preview {
            contexts: data.contexts.len(),
            notes: data.notes.len(),
            milestones: data.milestones.len(),
            tasks: data.tasks.len(),
        })
    }
    pub fn restore(&mut self, json: &str) -> Result<PathBuf> {
        let (data, _staging) = stage(json)?;
        // A durable rollback snapshot is required before replacing any authoritative rows.
        let rollback = self.directory.join(format!("before-restore-{}.json", id()));
        atomic_write(&rollback, self.backup()?.as_bytes())?;
        let tx = self.db.transaction().map_err(err)?;
        tx.execute_batch("DELETE FROM selections; DELETE FROM revisions; DELETE FROM note_contexts; DELETE FROM notes; DELETE FROM tasks; DELETE FROM milestones; DELETE FROM workspace; DELETE FROM contexts WHERE kind='project'; DELETE FROM contexts;").map_err(err)?;
        insert_backup(&tx, &data)?;
        tx.commit().map_err(err)?;
        Ok(rollback)
    }
}
fn configure(db: &Connection) -> Result<()> {
    db.pragma_update(None, "foreign_keys", "ON").map_err(err)?;
    db.busy_timeout(std::time::Duration::from_secs(5))
        .map_err(err)?;
    Ok(())
}
fn migrate(db: &Connection) -> Result<()> {
    let version: i64 = db
        .pragma_query_value(None, "user_version", |r| r.get(0))
        .map_err(err)?;
    match version {
        0 => {
            let tx = db.unchecked_transaction().map_err(err)?;
            tx.execute_batch(include_str!("migrations/0001_foundation.sql"))
                .map_err(err)?;
            tx.commit().map_err(err)?;
            Ok(())
        }
        1 => Ok(()),
        _ => Err(format!(
            "Unsupported database schema {version}; database left unchanged."
        )),
    }
}
fn require_context(db: &Connection, context: &str, section: &str) -> Result<()> {
    let kind: Option<String> = db
        .query_row("SELECT kind FROM contexts WHERE id=?1", [context], |r| {
            r.get(0)
        })
        .optional()
        .map_err(err)?;
    if !matches!(section, "scratch" | "log" | "milestones")
        || kind.is_none()
        || (section == "milestones" && kind.as_deref() != Some("project"))
    {
        return Err("Invalid context or section.".into());
    }
    Ok(())
}
fn require_milestone(db: &Connection, context: &str, milestone: &str) -> Result<()> {
    let exists: bool = db
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM milestones WHERE id=?1 AND project_id=?2)",
            params![milestone, context],
            |r| r.get(0),
        )
        .map_err(err)?;
    if exists {
        Ok(())
    } else {
        Err("Milestone is outside this project.".into())
    }
}
fn select(db: &Connection, context: &str, section: &str, note: &str, cursor: i64) -> Result<()> {
    db.execute(
        "INSERT OR REPLACE INTO selections VALUES(?1,?2,?3,?4)",
        params![context, section, note, cursor],
    )
    .map_err(err)?;
    Ok(())
}
fn query<T, P: rusqlite::Params, F: FnMut(&rusqlite::Row<'_>) -> rusqlite::Result<T>>(
    db: &Connection,
    sql: &str,
    p: P,
    f: F,
) -> Result<Vec<T>> {
    let mut stmt = db.prepare(sql).map_err(err)?;
    let rows = stmt.query_map(p, f).map_err(err)?;
    rows.collect::<rusqlite::Result<Vec<_>>>().map_err(err)
}
fn note(r: &rusqlite::Row) -> rusqlite::Result<Note> {
    Ok(Note {
        id: r.get(0)?,
        kind: r.get(1)?,
        body: r.get(2)?,
        revision: r.get(3)?,
        created_at: r.get(4)?,
        updated_at: r.get(5)?,
    })
}
fn selection(r: &rusqlite::Row) -> rusqlite::Result<Selection> {
    Ok(Selection {
        context_id: r.get(0)?,
        section: r.get(1)?,
        note_id: r.get(2)?,
        cursor: r.get(3)?,
    })
}
fn milestone(r: &rusqlite::Row) -> rusqlite::Result<Milestone> {
    Ok(Milestone {
        id: r.get(0)?,
        project_id: r.get(1)?,
        name: r.get(2)?,
    })
}
fn task(r: &rusqlite::Row) -> rusqlite::Result<Task> {
    Ok(Task {
        id: r.get(0)?,
        milestone_id: r.get(1)?,
        name: r.get(2)?,
        done: r.get(3)?,
        completed_at: r.get(4)?,
    })
}
fn contexts(db: &Connection) -> Result<Vec<Context>> {
    query(
        db,
        "SELECT id,kind,name,area_id,last_section FROM contexts ORDER BY rowid",
        [],
        |r| {
            Ok(Context {
                id: r.get(0)?,
                kind: r.get(1)?,
                name: r.get(2)?,
                area_id: r.get(3)?,
                last_section: r.get(4)?,
            })
        },
    )
}
fn workspace(db: &Connection) -> Result<Workspace> {
    db.query_row(
        "SELECT context_id,section FROM workspace WHERE singleton=1",
        [],
        |r| {
            Ok(Workspace {
                context_id: r.get(0)?,
                section: r.get(1)?,
            })
        },
    )
    .map_err(err)
}
fn snapshot(db: &Connection) -> Result<Backup> {
    Ok(Backup {
        format: "scholaros".into(),
        version: 1,
        contexts: contexts(db)?,
        workspace: workspace(db)?,
        notes: query(
            db,
            "SELECT id,kind,body,revision,created_at,updated_at FROM notes ORDER BY id",
            [],
            note,
        )?,
        links: query(
            db,
            "SELECT note_id,context_id FROM note_contexts ORDER BY note_id,context_id",
            [],
            |r| {
                Ok(Link {
                    note_id: r.get(0)?,
                    context_id: r.get(1)?,
                })
            },
        )?,
        revisions: query(
            db,
            "SELECT note_id,revision,body,saved_at FROM revisions ORDER BY note_id,revision",
            [],
            |r| {
                Ok(Revision {
                    note_id: r.get(0)?,
                    revision: r.get(1)?,
                    body: r.get(2)?,
                    saved_at: r.get(3)?,
                })
            },
        )?,
        selections: query(
            db,
            "SELECT context_id,section,note_id,cursor FROM selections ORDER BY context_id,section",
            [],
            selection,
        )?,
        milestones: query(
            db,
            "SELECT id,project_id,name FROM milestones ORDER BY rowid",
            [],
            milestone,
        )?,
        tasks: query(
            db,
            "SELECT id,milestone_id,name,done,completed_at FROM tasks ORDER BY rowid",
            [],
            task,
        )?,
    })
}
fn insert_backup(db: &Connection, b: &Backup) -> Result<()> {
    for c in b
        .contexts
        .iter()
        .filter(|c| c.kind == "area")
        .chain(b.contexts.iter().filter(|c| c.kind != "area"))
    {
        db.execute(
            "INSERT INTO contexts VALUES(?1,?2,?3,?4,?5)",
            params![c.id, c.kind, c.name, c.area_id, c.last_section],
        )
        .map_err(err)?;
    }
    for n in &b.notes {
        db.execute(
            "INSERT INTO notes VALUES(?1,?2,?3,?4,?5,?6)",
            params![n.id, n.kind, n.body, n.revision, n.created_at, n.updated_at],
        )
        .map_err(err)?;
    }
    for l in &b.links {
        db.execute(
            "INSERT INTO note_contexts VALUES(?1,?2)",
            params![l.note_id, l.context_id],
        )
        .map_err(err)?;
    }
    for r in &b.revisions {
        db.execute(
            "INSERT INTO revisions VALUES(?1,?2,?3,?4)",
            params![r.note_id, r.revision, r.body, r.saved_at],
        )
        .map_err(err)?;
    }
    for s in &b.selections {
        select(db, &s.context_id, &s.section, &s.note_id, s.cursor)?;
    }
    for m in &b.milestones {
        db.execute(
            "INSERT INTO milestones VALUES(?1,?2,?3)",
            params![m.id, m.project_id, m.name],
        )
        .map_err(err)?;
    }
    for t in &b.tasks {
        db.execute(
            "INSERT INTO tasks VALUES(?1,?2,?3,?4,?5)",
            params![t.id, t.milestone_id, t.name, t.done, t.completed_at],
        )
        .map_err(err)?;
    }
    db.execute(
        "INSERT INTO workspace VALUES(1,?1,?2)",
        params![b.workspace.context_id, b.workspace.section],
    )
    .map_err(err)?;
    Ok(())
}
fn stage(json: &str) -> Result<(Backup, Connection)> {
    if json.len() > 100_000_000 {
        return Err("Backup exceeds the 100 MB limit.".into());
    }
    let data: Backup = serde_json::from_str(json).map_err(err)?;
    if data.format != "scholaros" || data.version != 1 {
        return Err("Unsupported backup format or version.".into());
    }
    let db = Connection::open_in_memory().map_err(err)?;
    configure(&db)?;
    migrate(&db)?;
    let tx = db.unchecked_transaction().map_err(err)?;
    tx.execute("DELETE FROM workspace", []).map_err(err)?;
    // Reject duplicate selections explicitly: live navigation uses replace, restore must not.
    let mut seen = std::collections::HashSet::new();
    for s in &data.selections {
        if !seen.insert((&s.context_id, &s.section)) {
            return Err("Duplicate context selection.".into());
        }
    }
    insert_backup(&tx, &data)?;
    if let Some(c) = &data.workspace.context_id {
        require_context(&tx, c, &data.workspace.section)?;
    }
    for c in &data.contexts {
        require_context(&tx, &c.id, &c.last_section)?;
    }
    let invalid:bool=tx.query_row("SELECT EXISTS(SELECT 1 FROM notes n WHERE NOT EXISTS(SELECT 1 FROM note_contexts l WHERE l.note_id=n.id) OR NOT EXISTS(SELECT 1 FROM revisions r WHERE r.note_id=n.id AND r.revision=n.revision AND r.body=n.body AND r.saved_at=n.updated_at) OR (SELECT count(*) FROM revisions r WHERE r.note_id=n.id)!=n.revision OR EXISTS(SELECT 1 FROM revisions r WHERE r.note_id=n.id AND (r.revision>n.revision OR r.saved_at<n.created_at OR r.saved_at>n.updated_at)))",[],|r|r.get(0)).map_err(err)?;
    if invalid {
        return Err("Backup has missing note links or inconsistent revision history.".into());
    }
    for n in &data.notes {
        if n.body.len() > 10_000_000 {
            return Err("Backup note exceeds 10 MB.".into());
        }
    }
    tx.commit().map_err(err)?;
    Ok((data, db))
}
fn atomic_write(path: &Path, bytes: &[u8]) -> Result<()> {
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    // Refuse existing destinations so a mistaken backup path cannot overwrite a database.
    if path.exists() {
        return Err("Destination already exists. Choose a new backup filename.".into());
    }
    let mut temp = tempfile::NamedTempFile::new_in(parent).map_err(err)?;
    temp.write_all(bytes).map_err(err)?;
    temp.as_file().sync_all().map_err(err)?;
    temp.persist_noclobber(path).map_err(err)?;
    fs::File::open(parent)
        .and_then(|f| f.sync_all())
        .map_err(err)?;
    Ok(())
}
