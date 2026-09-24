CREATE TABLE note_files (
 note_id TEXT PRIMARY KEY NOT NULL REFERENCES notes(id),
 relative_path TEXT NOT NULL UNIQUE,
 content_hash TEXT NOT NULL,
 byte_size INTEGER NOT NULL CHECK(byte_size>=0),
 modified_ns INTEGER NOT NULL CHECK(modified_ns>=0),
 missing INTEGER NOT NULL DEFAULT 0 CHECK(missing IN (0,1))
);
PRAGMA user_version=2;
