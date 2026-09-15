-- Initial schema: runs and their sequence records.
-- Mirrors src-tauri/src/data/entities.rs (Run, Record, ResultType).

CREATE TABLE runs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    created_at DATETIME NOT NULL,
    result_type TEXT NOT NULL CHECK (result_type IN ('Fasta', 'Fastq'))
);

-- INTEGER PRIMARY KEY columns are already indexed via SQLite's rowid, so no
-- explicit index is added for runs.id.

CREATE TABLE records (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    run_id INTEGER NOT NULL REFERENCES runs (id) ON DELETE CASCADE,
    seq_id TEXT NOT NULL,
    description TEXT,
    gc_content REAL NOT NULL,
    n_orfs INTEGER NOT NULL,
    is_valid BOOLEAN NOT NULL,
    seq_len INTEGER NOT NULL,
    phred_score INTEGER
);

-- records.run_id is a foreign key, not a primary key, so SQLite doesn't
-- index it automatically; add it explicitly for run -> records lookups.
CREATE INDEX idx_records_run_id ON records (run_id);
