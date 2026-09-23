CREATE TABLE contexts (
 id TEXT PRIMARY KEY NOT NULL, kind TEXT NOT NULL CHECK(kind IN ('area','project')),
 name TEXT NOT NULL CHECK(length(trim(name)) BETWEEN 1 AND 200),
 area_id TEXT REFERENCES contexts(id), last_section TEXT NOT NULL DEFAULT 'scratch' CHECK(last_section IN ('scratch','log','milestones')),
 CHECK(kind='project' OR (area_id IS NULL AND last_section!='milestones'))
);
CREATE TRIGGER context_area_insert BEFORE INSERT ON contexts WHEN NEW.area_id IS NOT NULL BEGIN
 SELECT CASE WHEN NOT EXISTS(SELECT 1 FROM contexts WHERE id=NEW.area_id AND kind='area') THEN RAISE(ABORT,'Project parent must be an Area') END;
END;
CREATE TABLE notes (
 id TEXT PRIMARY KEY NOT NULL, kind TEXT NOT NULL CHECK(kind IN ('scratch','log')),
 body TEXT NOT NULL, revision INTEGER NOT NULL CHECK(revision>=1),
 created_at INTEGER NOT NULL CHECK(created_at>=0), updated_at INTEGER NOT NULL CHECK(updated_at>=created_at)
);
CREATE TABLE note_contexts (
 note_id TEXT NOT NULL REFERENCES notes(id), context_id TEXT NOT NULL REFERENCES contexts(id), PRIMARY KEY(note_id,context_id)
);
CREATE TABLE revisions (
 note_id TEXT NOT NULL REFERENCES notes(id), revision INTEGER NOT NULL CHECK(revision>=1), body TEXT NOT NULL,
 saved_at INTEGER NOT NULL CHECK(saved_at>=0), PRIMARY KEY(note_id,revision)
);
CREATE TABLE selections (
 context_id TEXT NOT NULL REFERENCES contexts(id), section TEXT NOT NULL CHECK(section IN ('scratch','log')),
 note_id TEXT NOT NULL, cursor INTEGER NOT NULL CHECK(cursor>=0),
 PRIMARY KEY(context_id,section), FOREIGN KEY(note_id,context_id) REFERENCES note_contexts(note_id,context_id)
);
CREATE TRIGGER selection_section_insert BEFORE INSERT ON selections BEGIN
 SELECT CASE WHEN NOT EXISTS(SELECT 1 FROM notes WHERE id=NEW.note_id AND kind=NEW.section) THEN RAISE(ABORT,'Note is outside section') END;
END;
CREATE TABLE milestones (
 id TEXT PRIMARY KEY NOT NULL, project_id TEXT NOT NULL REFERENCES contexts(id),
 name TEXT NOT NULL CHECK(length(trim(name)) BETWEEN 1 AND 200)
);
CREATE TRIGGER milestone_project_insert BEFORE INSERT ON milestones BEGIN
 SELECT CASE WHEN NOT EXISTS(SELECT 1 FROM contexts WHERE id=NEW.project_id AND kind='project') THEN RAISE(ABORT,'Milestone requires a project') END;
END;
CREATE TABLE tasks (
 id TEXT PRIMARY KEY NOT NULL, milestone_id TEXT NOT NULL REFERENCES milestones(id),
 name TEXT NOT NULL CHECK(length(trim(name)) BETWEEN 1 AND 200),
 done INTEGER NOT NULL CHECK(done IN (0,1)), completed_at INTEGER,
 CHECK((done=0 AND completed_at IS NULL) OR (done=1 AND completed_at IS NOT NULL AND completed_at>=0))
);
CREATE TABLE workspace (
 singleton INTEGER PRIMARY KEY CHECK(singleton=1), context_id TEXT REFERENCES contexts(id),
 section TEXT NOT NULL CHECK(section IN ('scratch','log','milestones'))
);
INSERT INTO workspace VALUES(1,NULL,'scratch');
PRAGMA user_version=1;

CREATE TRIGGER context_ownership_update BEFORE UPDATE OF kind,area_id ON contexts BEGIN
 SELECT RAISE(ABORT,'Context ownership changes are not supported in schema 1');
END;
CREATE TRIGGER milestone_project_update BEFORE UPDATE OF project_id ON milestones BEGIN
 SELECT CASE WHEN NOT EXISTS(SELECT 1 FROM contexts WHERE id=NEW.project_id AND kind='project') THEN RAISE(ABORT,'Milestone requires a project') END;
END;
CREATE TRIGGER selection_section_update BEFORE UPDATE ON selections BEGIN
 SELECT CASE WHEN NOT EXISTS(SELECT 1 FROM notes WHERE id=NEW.note_id AND kind=NEW.section) THEN RAISE(ABORT,'Note is outside section') END;
END;
