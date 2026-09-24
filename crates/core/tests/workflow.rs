use scholar_core::{Command, Store, View};
use serde_json::{json, Value};
fn send(store: &mut Store, v: Value) -> View {
    store
        .execute(serde_json::from_value::<Command>(v).unwrap())
        .unwrap()
}
fn context(s: &mut Store, kind: &str, name: &str, area: Option<&str>) -> String {
    send(
        s,
        json!({"type":"create_context","kind":kind,"name":name,"area_id":area}),
    )
    .workspace
    .context_id
    .unwrap()
}
fn open(s: &mut Store, c: &str, section: &str) -> View {
    send(s, json!({"type":"open","context_id":c,"section":section}))
}
fn note(s: &mut Store, c: &str, section: &str) -> String {
    send(
        s,
        json!({"type":"create_note","context_id":c,"section":section}),
    )
    .selection
    .unwrap()
    .note_id
}
fn save(s: &mut Store, c: &str, n: &str, body: &str, rev: i64) -> View {
    send(
        s,
        json!({"type":"save_note","context_id":c,"note_id":n,"body":body,"revision":rev,"cursor":4}),
    )
}
fn fails(s: &mut Store, v: Value) {
    assert!(s.execute(serde_json::from_value(v).unwrap()).is_err());
}
#[test]
fn isolation_hierarchy_restart_and_round_trip() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("work.sqlite");
    let mut s = Store::open(&path).unwrap();
    let area = context(&mut s, "area", "Research", None);
    let a = context(&mut s, "project", "Project A", Some(&area));
    let an = note(&mut s, &a, "scratch");
    assert_eq!(s.view().unwrap().notes[0].body, "");
    save(&mut s, &a, &an, "A scratch α\nfreeform", 1);
    let a2 = note(&mut s, &a, "scratch");
    save(&mut s, &a, &a2, "Last opened A", 1);
    open(&mut s, &a, "log");
    let log = note(&mut s, &a, "log");
    save(&mut s, &a, &log, "Activity", 1);
    let b = context(&mut s, "project", "Project B", None);
    let bn = note(&mut s, &b, "scratch");
    save(&mut s, &b, &bn, "B private", 1);
    assert_eq!(s.view().unwrap().notes.len(), 1);
    let av = open(&mut s, &a, "scratch");
    assert_eq!(av.notes.len(), 2);
    assert_eq!(av.selection.unwrap().note_id, a2);
    assert!(av.notes.iter().all(|n| n.id != bn));
    fails(
        &mut s,
        json!({"type":"save_note","context_id":a,"note_id":bn,"body":"leak","revision":2,"cursor":0}),
    );
    fails(
        &mut s,
        json!({"type":"select_note","context_id":a,"section":"scratch","note_id":bn,"cursor":0}),
    );
    fails(
        &mut s,
        json!({"type":"select_note","context_id":a,"section":"scratch","note_id":log,"cursor":0}),
    );
    assert!(open(&mut s, &area, "scratch").notes.is_empty());
    let area_note = note(&mut s, &area, "scratch");
    save(&mut s, &area, &area_note, "Area only", 1);
    fails(
        &mut s,
        json!({"type":"create_milestone","context_id":area,"name":"Invalid"}),
    );
    fails(
        &mut s,
        json!({"type":"create_context","kind":"project","name":"Invalid parent","area_id":a}),
    );
    open(&mut s, &a, "milestones");
    let v = send(
        &mut s,
        json!({"type":"create_milestone","context_id":a,"name":"Outcome"}),
    );
    let m = &v.milestones[0].id;
    fails(
        &mut s,
        json!({"type":"create_task","context_id":b,"milestone_id":m,"name":"Wrong project"}),
    );
    fails(
        &mut s,
        json!({"type":"create_task","context_id":a,"milestone_id":"missing","name":"Orphan"}),
    );
    let v = send(
        &mut s,
        json!({"type":"create_task","context_id":a,"milestone_id":m,"name":"Read a paper"}),
    );
    let t = &v.tasks[0].id;
    fails(
        &mut s,
        json!({"type":"set_task","context_id":b,"task_id":t,"done":true}),
    );
    let v = send(
        &mut s,
        json!({"type":"set_task","context_id":a,"task_id":t,"done":true}),
    );
    assert!(v.tasks[0].completed_at.is_some());
    open(&mut s, &a, "scratch");
    drop(s);
    let mut s = Store::open(&path).unwrap();
    let v = s.view().unwrap();
    assert_eq!(v.workspace.context_id.as_deref(), Some(a.as_str()));
    assert_eq!(v.selection.unwrap().note_id, a2);
    assert!(v.notes.iter().any(|n| n.body == "A scratch α\nfreeform"));
    let backup = s.backup().unwrap();
    assert_eq!(Store::preview(&backup).unwrap().notes, 5);
    let export = dir.path().join("backup.json");
    s.export(&export).unwrap();
    assert!(s.export(&export).is_err());
    assert_eq!(std::fs::read_to_string(export).unwrap(), backup);
    context(&mut s, "project", "Will roll back", None);
    let before = s.backup().unwrap();
    let rollback = s.restore(&backup).unwrap();
    assert_eq!(std::fs::read_to_string(rollback).unwrap(), before);
    assert_eq!(s.backup().unwrap(), backup);
    // Restoring the same snapshot twice does not duplicate anything.
    s.restore(&backup).unwrap();
    assert_eq!(s.backup().unwrap(), backup);
}
#[test]
fn conflicts_and_invalid_restore_leave_authoritative_data_unchanged() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("work.sqlite");
    let mut s = Store::open(&path).unwrap();
    let c = context(&mut s, "project", "Draft", None);
    let n = note(&mut s, &c, "scratch");
    let mut second = Store::open(&path).unwrap();
    save(&mut s, &c, &n, "First writer", 1);
    fails(
        &mut second,
        json!({"type":"save_note","context_id":c,"note_id":n,"body":"Stale","revision":1,"cursor":0}),
    );
    assert_eq!(second.view().unwrap().notes[0].body, "First writer");
    let good = s.backup().unwrap();
    for mutation in 0..10 {
        let mut b: Value = serde_json::from_str(&good).unwrap();
        match mutation {
            0 => b["version"] = json!(99),
            1 => b["links"] = json!([]),
            2 => b["notes"][0]["revision"] = json!(99),
            3 => b["selections"][0]["context_id"] = json!("missing"),
            4 => b["contexts"][0]["kind"] = json!("bad"),
            5 => b["revisions"][1]["body"] = json!("Corrupt"),
            6 => b["workspace"]["section"] = json!("invalid"),
            7 => {
                b["tasks"] = json!([{"id":"t","milestone_id":"missing","name":"Bad","done":false,"completed_at":null}])
            }
            8 => {
                let duplicate = b["selections"][0].clone();
                b["selections"].as_array_mut().unwrap().push(duplicate);
            }
            _ => b["unknown_future_data"] = json!(true),
        }
        assert!(s.restore(&b.to_string()).is_err(), "mutation {mutation}");
        assert_eq!(s.backup().unwrap(), good);
    }
    assert!(s.restore("not JSON").is_err());
    assert_eq!(s.backup().unwrap(), good);
}
#[test]
fn rollback_write_failure_and_database_write_failure_are_atomic() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("work.sqlite");
    let mut s = Store::open(&path).unwrap();
    let c = context(&mut s, "project", "Draft", None);
    let n = note(&mut s, &c, "scratch");
    let before = s.backup().unwrap();
    let other = rusqlite::Connection::open(&path).unwrap();
    other.execute_batch("CREATE TRIGGER fail_save BEFORE INSERT ON revisions WHEN NEW.revision>1 BEGIN SELECT RAISE(ABORT,'disk failure simulation'); END;").unwrap();
    fails(
        &mut s,
        json!({"type":"save_note","context_id":c,"note_id":n,"body":"Cannot commit","revision":1,"cursor":0}),
    );
    assert_eq!(s.backup().unwrap(), before);
    other.execute_batch("DROP TRIGGER fail_save").unwrap();
    // Replace the application-data directory with a file so reconciliation and rollback cannot write.
    let moved = dir.path().with_extension("moved");
    std::fs::rename(dir.path(), &moved).unwrap();
    std::fs::write(dir.path(), "blocked").unwrap();
    assert!(s.restore(&before).is_err());
    assert_eq!(s.backup().unwrap(), before);
    drop(other);
    drop(s);
    std::fs::remove_file(dir.path()).unwrap();
    std::fs::rename(&moved, dir.path()).unwrap();
}
fn markdown_files(root: &std::path::Path) -> Vec<std::path::PathBuf> {
    fn visit(directory: &std::path::Path, files: &mut Vec<std::path::PathBuf>) {
        for entry in std::fs::read_dir(directory).unwrap() {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_dir() {
                visit(&entry.path(), files);
            } else if entry.path().extension().and_then(|value| value.to_str()) == Some("md") {
                files.push(entry.path());
            }
        }
    }
    let mut files = Vec::new();
    visit(root, &mut files);
    files.sort();
    files
}

#[test]
fn workspace_materializes_and_reconciles_external_file_operations() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("work.sqlite");
    let mut store = Store::open(&path).unwrap();
    let project = context(&mut store, "project", "External files", None);
    let first = note(&mut store, &project, "scratch");
    save(&mut store, &project, &first, "Written in ScholarOS", 1);

    let workspace = store.workspace_path().to_path_buf();
    let files = markdown_files(&workspace);
    assert_eq!(files.len(), 1);
    assert_eq!(
        std::fs::read_to_string(&files[0]).unwrap(),
        "Written in ScholarOS"
    );

    std::fs::write(&files[0], "Changed from VS Code with more text").unwrap();
    let view = store.view().unwrap();
    let changed = view.notes.iter().find(|value| value.id == first).unwrap();
    assert_eq!(changed.body, "Changed from VS Code with more text");
    assert_eq!(changed.revision, 3);

    let scratch = files[0].parent().unwrap();
    let nested = scratch.join("Experiments");
    std::fs::create_dir(&nested).unwrap();
    let terminal_note = nested.join("Terminal note.md");
    std::fs::write(&terminal_note, "Created directly from the terminal").unwrap();
    std::fs::write(nested.join("ignored.txt"), "not a note").unwrap();
    let view = store.view().unwrap();
    assert!(view
        .notes
        .iter()
        .any(|value| value.body == "Created directly from the terminal"));
    assert_eq!(view.notes.len(), 2);

    let logs = scratch.parent().unwrap().join("Logs");
    let moved = logs.join("Renamed terminal note.md");
    std::fs::rename(&terminal_note, &moved).unwrap();
    let log_view = open(&mut store, &project, "log");
    assert!(log_view
        .notes
        .iter()
        .any(|value| value.body == "Created directly from the terminal"));
    let scratch_view = open(&mut store, &project, "scratch");
    assert!(!scratch_view
        .notes
        .iter()
        .any(|value| value.body == "Created directly from the terminal"));

    std::fs::remove_file(&moved).unwrap();
    let log_view = open(&mut store, &project, "log");
    assert!(!log_view
        .notes
        .iter()
        .any(|value| value.body == "Created directly from the terminal"));
    assert!(store
        .backup()
        .unwrap()
        .contains("Created directly from the terminal"));
    std::fs::write(&moved, "Created directly from the terminal").unwrap();
    let log_view = store.view().unwrap();
    assert!(log_view
        .notes
        .iter()
        .any(|value| value.body == "Created directly from the terminal"));
}

#[test]
fn version_one_database_materializes_existing_notes() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("legacy.sqlite");
    let db = rusqlite::Connection::open(&path).unwrap();
    db.execute_batch(include_str!("../src/migrations/0001_foundation.sql"))
        .unwrap();
    db.execute(
        "INSERT INTO contexts(id,kind,name) VALUES('p','project','Legacy')",
        [],
    )
    .unwrap();
    db.execute(
        "INSERT INTO notes VALUES('n','scratch','Existing body',1,1,1)",
        [],
    )
    .unwrap();
    db.execute("INSERT INTO note_contexts VALUES('n','p')", [])
        .unwrap();
    db.execute("INSERT INTO revisions VALUES('n',1,'Existing body',1)", [])
        .unwrap();
    db.execute("UPDATE workspace SET context_id='p'", [])
        .unwrap();
    drop(db);

    let mut store = Store::open(&path).unwrap();
    assert_eq!(store.view().unwrap().notes[0].body, "Existing body");
    let files = markdown_files(store.workspace_path());
    assert_eq!(files.len(), 1);
    assert_eq!(std::fs::read_to_string(&files[0]).unwrap(), "Existing body");
    let db = rusqlite::Connection::open(path).unwrap();
    assert_eq!(
        db.pragma_query_value::<i64, _>(None, "user_version", |row| row.get(0))
            .unwrap(),
        2
    );
}

#[test]
fn unsupported_database_is_not_migrated() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("future.sqlite");
    let db = rusqlite::Connection::open(&path).unwrap();
    db.pragma_update(None, "user_version", 42).unwrap();
    drop(db);
    assert!(Store::open(&path).is_err());
    let db = rusqlite::Connection::open(path).unwrap();
    assert_eq!(
        db.pragma_query_value::<i64, _>(None, "user_version", |r| r.get(0))
            .unwrap(),
        42
    );
}

#[test]
fn restart_child() {
    let Ok(path) = std::env::var("SCHOLAR_RESTART_TEST_PATH") else {
        return;
    };
    let mut s = Store::open(std::path::Path::new(&path)).unwrap();
    if std::env::var("SCHOLAR_RESTART_TEST_PHASE").unwrap() == "write" {
        let c = context(&mut s, "project", "Restart proof", None);
        let n = note(&mut s, &c, "scratch");
        save(&mut s, &c, &n, "Saved across process termination", 1);
    } else {
        let v = s.view().unwrap();
        assert_eq!(v.notes[0].body, "Saved across process termination");
        assert_eq!(v.selection.unwrap().note_id, v.notes[0].id);
    }
}

#[test]
fn saved_content_survives_process_restart() {
    let dir = tempfile::tempdir().unwrap();
    for phase in ["write", "read"] {
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "restart_child"])
            .env(
                "SCHOLAR_RESTART_TEST_PATH",
                dir.path().join("restart.sqlite"),
            )
            .env("SCHOLAR_RESTART_TEST_PHASE", phase)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stdout)
        );
    }
}
