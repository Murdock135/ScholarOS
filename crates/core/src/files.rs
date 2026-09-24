use rusqlite::{params, Connection};
use std::{
    collections::{HashMap, HashSet},
    fs,
    io::Write,
    path::{Component, Path, PathBuf},
    time::UNIX_EPOCH,
};

use crate::{err, id, now, query, Result};

const NOTE_LIMIT: usize = 10_000_000;

#[derive(Debug)]
pub(crate) struct WrittenFile {
    pub path: PathBuf,
    pub hash: String,
    pub size: i64,
    pub modified_ns: i64,
}

#[derive(Debug, Clone)]
struct TrackedFile {
    note_id: String,
    relative_path: String,
    body: String,
    updated_at: i64,
    hash: String,
    size: i64,
    modified_ns: i64,
    missing: bool,
}

#[derive(Debug, Clone)]
struct FoundFile {
    context_id: String,
    kind: String,
    relative_path: String,
    path: PathBuf,
}

pub(crate) fn default_root(database_directory: &Path) -> PathBuf {
    database_directory.join("ScholarOS Workspace")
}

pub(crate) fn materialize(
    db: &mut Connection,
    root: &Path,
    overwrite_existing: bool,
) -> Result<()> {
    fs::create_dir_all(root).map_err(err)?;
    ensure_context_directories(db, root)?;
    let notes = query(
        db,
        "SELECT n.id,n.body FROM notes n LEFT JOIN note_files f ON f.note_id=n.id WHERE f.note_id IS NULL ORDER BY n.id",
        [],
        |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
    )?;
    for (note_id, body) in notes {
        let proposed = generated_relative_path(db, &note_id, &body)?;
        let relative = available_relative_path(root, &proposed)?;
        let path = prepare_path(root, Path::new(&relative))?;
        let bytes = if path.exists() && !overwrite_existing {
            fs::read(&path).map_err(err)?
        } else {
            replace_file(&path, body.as_bytes())?;
            body.as_bytes().to_vec()
        };
        validate_bytes(&path, &bytes)?;
        let (size, modified_ns) = file_metadata(&path)?;
        db.execute(
            "INSERT INTO note_files(note_id,relative_path,content_hash,byte_size,modified_ns,missing) VALUES(?1,?2,?3,?4,?5,0)",
            params![note_id, relative, content_hash(&bytes), size, modified_ns],
        )
        .map_err(err)?;
    }
    Ok(())
}

pub(crate) fn reconcile(db: &mut Connection, root: &Path) -> Result<()> {
    materialize(db, root, false)?;
    let tracked = tracked_files(db)?;
    let found = discover_files(db, root)?;
    let found_by_path: HashMap<String, FoundFile> = found
        .iter()
        .cloned()
        .map(|file| (file.relative_path.clone(), file))
        .collect();
    let tracked_paths: HashSet<&str> = tracked
        .iter()
        .map(|file| file.relative_path.as_str())
        .collect();
    let mut missing: Vec<TrackedFile> = tracked
        .iter()
        .filter(|file| !found_by_path.contains_key(&file.relative_path))
        .cloned()
        .collect();
    let mut untracked = Vec::new();
    for file in &found {
        if !tracked_paths.contains(file.relative_path.as_str()) {
            let bytes = fs::read(&file.path).map_err(err)?;
            validate_bytes(&file.path, &bytes)?;
            untracked.push((file.clone(), String::from_utf8(bytes).map_err(err)?));
        }
    }

    let tx = db.transaction().map_err(err)?;
    for file in tracked
        .iter()
        .filter(|file| found_by_path.contains_key(&file.relative_path))
    {
        let found = &found_by_path[&file.relative_path];
        let (size, modified_ns) = file_metadata(&found.path)?;
        if !file.missing && size == file.size && modified_ns == file.modified_ns {
            continue;
        }
        let bytes = fs::read(&found.path).map_err(err)?;
        validate_bytes(&found.path, &bytes)?;
        let body = String::from_utf8(bytes).map_err(err)?;
        let hash = content_hash(body.as_bytes());
        if hash != file.hash || body != file.body {
            let saved_at = now().max(file.updated_at + 1);
            tx.execute(
                "UPDATE notes SET body=?1,revision=revision+1,updated_at=?2 WHERE id=?3",
                params![body, saved_at, file.note_id],
            )
            .map_err(err)?;
            tx.execute(
                "INSERT INTO revisions SELECT id,revision,body,updated_at FROM notes WHERE id=?1",
                [&file.note_id],
            )
            .map_err(err)?;
        }
        tx.execute(
            "UPDATE note_files SET content_hash=?1,byte_size=?2,modified_ns=?3,missing=0 WHERE note_id=?4",
            params![hash, size, modified_ns, file.note_id],
        )
        .map_err(err)?;
    }

    let mut claimed_missing = HashSet::new();
    let mut imported_paths = HashSet::new();
    for (found, body) in &untracked {
        let hash = content_hash(body.as_bytes());
        let candidates: Vec<&TrackedFile> = missing
            .iter()
            .filter(|old| !claimed_missing.contains(&old.note_id) && old.hash == hash)
            .collect();
        if candidates.len() == 1 {
            let old = candidates[0];
            let (size, modified_ns) = file_metadata(&found.path)?;
            tx.execute("DELETE FROM selections WHERE note_id=?1", [&old.note_id])
                .map_err(err)?;
            tx.execute("DELETE FROM note_contexts WHERE note_id=?1", [&old.note_id])
                .map_err(err)?;
            tx.execute(
                "INSERT INTO note_contexts(note_id,context_id) VALUES(?1,?2)",
                params![old.note_id, found.context_id],
            )
            .map_err(err)?;
            tx.execute(
                "UPDATE notes SET kind=?1 WHERE id=?2",
                params![found.kind, old.note_id],
            )
            .map_err(err)?;
            tx.execute(
                "UPDATE note_files SET relative_path=?1,byte_size=?2,modified_ns=?3,missing=0 WHERE note_id=?4",
                params![found.relative_path, size, modified_ns, old.note_id],
            )
            .map_err(err)?;
            claimed_missing.insert(old.note_id.clone());
            imported_paths.insert(found.relative_path.clone());
        }
    }
    missing.retain(|file| !claimed_missing.contains(&file.note_id));

    for (found, body) in untracked {
        if imported_paths.contains(&found.relative_path) {
            continue;
        }
        let note_id = id();
        let saved_at = now();
        let (size, modified_ns) = file_metadata(&found.path)?;
        tx.execute(
            "INSERT INTO notes(id,kind,body,revision,created_at,updated_at) VALUES(?1,?2,?3,1,?4,?4)",
            params![note_id, found.kind, body, saved_at],
        )
        .map_err(err)?;
        tx.execute(
            "INSERT INTO note_contexts(note_id,context_id) VALUES(?1,?2)",
            params![note_id, found.context_id],
        )
        .map_err(err)?;
        tx.execute(
            "INSERT INTO revisions(note_id,revision,body,saved_at) VALUES(?1,1,?2,?3)",
            params![note_id, body, saved_at],
        )
        .map_err(err)?;
        tx.execute(
            "INSERT INTO note_files(note_id,relative_path,content_hash,byte_size,modified_ns,missing) VALUES(?1,?2,?3,?4,?5,0)",
            params![note_id, found.relative_path, content_hash(body.as_bytes()), size, modified_ns],
        )
        .map_err(err)?;
    }

    for file in missing {
        tx.execute("DELETE FROM selections WHERE note_id=?1", [&file.note_id])
            .map_err(err)?;
        tx.execute(
            "UPDATE note_files SET missing=1 WHERE note_id=?1",
            [&file.note_id],
        )
        .map_err(err)?;
    }
    tx.commit().map_err(err)?;
    Ok(())
}

pub(crate) fn write_note(
    db: &Connection,
    root: &Path,
    note_id: &str,
    body: &str,
) -> Result<WrittenFile> {
    if body.len() > NOTE_LIMIT {
        return Err("Note exceeds the 10 MB limit; draft retained.".into());
    }
    let relative: String = db
        .query_row(
            "SELECT relative_path FROM note_files WHERE note_id=?1",
            [note_id],
            |row| row.get(0),
        )
        .map_err(err)?;
    let path = prepare_path(root, Path::new(&relative))?;
    replace_file(&path, body.as_bytes())?;
    let (size, modified_ns) = file_metadata(&path)?;
    Ok(WrittenFile {
        path,
        hash: content_hash(body.as_bytes()),
        size,
        modified_ns,
    })
}

pub(crate) fn restore_body(path: &Path, body: &str) -> Result<()> {
    replace_file(path, body.as_bytes())
}

fn tracked_files(db: &Connection) -> Result<Vec<TrackedFile>> {
    query(
        db,
        "SELECT n.id,f.relative_path,n.body,n.updated_at,f.content_hash,f.byte_size,f.modified_ns,f.missing FROM notes n JOIN note_files f ON f.note_id=n.id ORDER BY n.id",
        [],
        |row| {
            Ok(TrackedFile {
                note_id: row.get(0)?,
                relative_path: row.get(1)?,
                body: row.get(2)?,
                updated_at: row.get(3)?,
                hash: row.get(4)?,
                size: row.get(5)?,
                modified_ns: row.get(6)?,
                missing: row.get(7)?,
            })
        },
    )
}

fn discover_files(db: &Connection, root: &Path) -> Result<Vec<FoundFile>> {
    let mut found = Vec::new();
    for (context_id, kind, relative) in context_roots(db)? {
        let directory = prepare_directory(root, &relative)?;
        collect_markdown(root, &directory, &context_id, &kind, &mut found)?;
    }
    Ok(found)
}

fn collect_markdown(
    root: &Path,
    directory: &Path,
    context_id: &str,
    kind: &str,
    found: &mut Vec<FoundFile>,
) -> Result<()> {
    for entry in fs::read_dir(directory).map_err(err)? {
        let entry = entry.map_err(err)?;
        let file_type = entry.file_type().map_err(err)?;
        if file_type.is_symlink() {
            continue;
        }
        let path = entry.path();
        if file_type.is_dir() {
            collect_markdown(root, &path, context_id, kind, found)?;
        } else if file_type.is_file()
            && path
                .extension()
                .and_then(|extension| extension.to_str())
                .is_some_and(|extension| extension.eq_ignore_ascii_case("md"))
        {
            let relative = path
                .strip_prefix(root)
                .map_err(err)?
                .to_str()
                .ok_or_else(|| "Workspace path is not valid UTF-8.".to_string())?
                .to_owned();
            found.push(FoundFile {
                context_id: context_id.to_owned(),
                kind: kind.to_owned(),
                relative_path: relative,
                path,
            });
        }
    }
    Ok(())
}

fn ensure_context_directories(db: &Connection, root: &Path) -> Result<()> {
    for (_, _, relative) in context_roots(db)? {
        prepare_directory(root, &relative)?;
    }
    Ok(())
}

fn context_roots(db: &Connection) -> Result<Vec<(String, String, PathBuf)>> {
    let contexts = query(
        db,
        "SELECT id,name,kind,area_id FROM contexts ORDER BY rowid",
        [],
        |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, Option<String>>(3)?,
            ))
        },
    )?;
    let names: HashMap<&str, &str> = contexts
        .iter()
        .map(|(id, name, _, _)| (id.as_str(), name.as_str()))
        .collect();
    let mut roots = Vec::new();
    for (context_id, context_name, context_kind, area_id) in &contexts {
        let own = format!("{}--{}", safe_name(context_name), short_id(context_id));
        let base = if context_kind == "area" {
            PathBuf::from("Areas").join(own)
        } else if let Some(area_id) = area_id {
            let area_name = names
                .get(area_id.as_str())
                .ok_or_else(|| "Project Area is missing.".to_string())?;
            PathBuf::from("Areas")
                .join(format!("{}--{}", safe_name(area_name), short_id(area_id)))
                .join("Projects")
                .join(own)
        } else {
            PathBuf::from("Projects").join(own)
        };
        roots.push((
            context_id.clone(),
            "scratch".into(),
            base.join("Scratchpad"),
        ));
        roots.push((context_id.clone(), "log".into(), base.join("Logs")));
    }
    Ok(roots)
}

fn generated_relative_path(db: &Connection, note_id: &str, body: &str) -> Result<String> {
    let (context_id, kind): (String, String) = db
        .query_row(
            "SELECT l.context_id,n.kind FROM notes n JOIN note_contexts l ON l.note_id=n.id WHERE n.id=?1 ORDER BY l.context_id LIMIT 1",
            [note_id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .map_err(err)?;
    let root = context_roots(db)?
        .into_iter()
        .find(|(candidate, candidate_kind, _)| candidate == &context_id && candidate_kind == &kind)
        .map(|(_, _, path)| path)
        .ok_or_else(|| "Note context folder is missing.".to_string())?;
    let title = body
        .lines()
        .next()
        .unwrap_or("")
        .trim()
        .trim_start_matches('#')
        .trim();
    let title = if title.is_empty() {
        "Untitled".into()
    } else {
        safe_name(title)
    };
    root.join(format!("{title}.md"))
        .to_str()
        .map(str::to_owned)
        .ok_or_else(|| "Workspace path is not valid UTF-8.".into())
}

fn available_relative_path(root: &Path, proposed: &str) -> Result<String> {
    let proposed = Path::new(proposed);
    if !root.join(proposed).exists() {
        return proposed
            .to_str()
            .map(str::to_owned)
            .ok_or_else(|| "Workspace path is not valid UTF-8.".into());
    }
    let parent = proposed.parent().unwrap_or(Path::new(""));
    let stem = proposed
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("Untitled");
    for index in 2..=10_000 {
        let candidate = parent.join(format!("{stem} {index}.md"));
        if !root.join(&candidate).exists() {
            return candidate
                .to_str()
                .map(str::to_owned)
                .ok_or_else(|| "Workspace path is not valid UTF-8.".into());
        }
    }
    Err("Could not choose an available Markdown filename.".into())
}

fn prepare_directory(root: &Path, relative: &Path) -> Result<PathBuf> {
    let marker = relative.join(".scholaros-directory-marker");
    let marker_path = prepare_path(root, &marker)?;
    let directory = marker_path
        .parent()
        .ok_or_else(|| "Workspace directory is invalid.".to_string())?
        .to_path_buf();
    Ok(directory)
}

fn prepare_path(root: &Path, relative: &Path) -> Result<PathBuf> {
    if relative.is_absolute()
        || relative
            .components()
            .any(|part| !matches!(part, Component::Normal(_)))
    {
        return Err("Invalid Markdown workspace path.".into());
    }
    fs::create_dir_all(root).map_err(err)?;
    let mut current = root.to_path_buf();
    let mut parts = relative.components().peekable();
    while let Some(Component::Normal(part)) = parts.next() {
        current.push(part);
        if parts.peek().is_some() {
            if current.exists() {
                if fs::symlink_metadata(&current)
                    .map_err(err)?
                    .file_type()
                    .is_symlink()
                {
                    return Err(format!(
                        "Workspace path contains a symlink: {}",
                        current.display()
                    ));
                }
                if !current.is_dir() {
                    return Err(format!(
                        "Workspace folder path is not a directory: {}",
                        current.display()
                    ));
                }
            } else {
                fs::create_dir(&current).map_err(err)?;
            }
        } else if current.exists()
            && fs::symlink_metadata(&current)
                .map_err(err)?
                .file_type()
                .is_symlink()
        {
            return Err(format!("Markdown note is a symlink: {}", current.display()));
        }
    }
    Ok(current)
}

fn replace_file(path: &Path, bytes: &[u8]) -> Result<()> {
    let parent = path
        .parent()
        .ok_or_else(|| "Markdown file has no parent directory.".to_string())?;
    let mut temp = tempfile::NamedTempFile::new_in(parent).map_err(err)?;
    temp.write_all(bytes).map_err(err)?;
    temp.as_file().sync_all().map_err(err)?;
    temp.persist(path).map_err(err)?;
    fs::File::open(parent)
        .and_then(|directory| directory.sync_all())
        .map_err(err)?;
    Ok(())
}

fn validate_bytes(path: &Path, bytes: &[u8]) -> Result<()> {
    if bytes.len() > NOTE_LIMIT {
        return Err(format!("Markdown file exceeds 10 MB: {}", path.display()));
    }
    std::str::from_utf8(bytes)
        .map(|_| ())
        .map_err(|_| format!("Markdown file is not valid UTF-8: {}", path.display()))
}

fn file_metadata(path: &Path) -> Result<(i64, i64)> {
    let metadata = fs::metadata(path).map_err(err)?;
    if !metadata.is_file() {
        return Err(format!(
            "Markdown note is not a regular file: {}",
            path.display()
        ));
    }
    let size = i64::try_from(metadata.len()).map_err(err)?;
    let modified_ns = metadata
        .modified()
        .map_err(err)?
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos()
        .min(i64::MAX as u128) as i64;
    Ok((size, modified_ns))
}

fn content_hash(bytes: &[u8]) -> String {
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

fn safe_name(value: &str) -> String {
    let cleaned: String = value
        .trim()
        .chars()
        .map(|character| {
            if character.is_alphanumeric() || matches!(character, ' ' | '-' | '_') {
                character
            } else {
                '-'
            }
        })
        .collect();
    let cleaned = cleaned.trim_matches([' ', '-', '_']);
    if cleaned.is_empty() {
        "Untitled".into()
    } else {
        cleaned.chars().take(80).collect()
    }
}

fn short_id(value: &str) -> &str {
    value.get(..8).unwrap_or(value)
}
