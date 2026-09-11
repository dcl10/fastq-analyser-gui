# CLAUDE.md

## Project Overview

`fastq-analyser-gui` is a Tauri desktop app that analyses DNA sequence records. Users paste FASTA/FASTQ text or pick
a file (optionally gzip-compressed), and the app reports per-record statistics: GC content, open reading frame (ORF)
count, sequence length, validity, and — for FASTQ — a Phred quality score. Results can be saved to and reloaded from
JSON.

The project is currently being resurrected after a period of inactivity. The frontend (Vite + React + Chakra UI) and
the Tauri v1 shell are both slated for replacement — see open GitHub issues for the Tauri v2 upgrade / Next.js
migration before assuming the structure below is final.

## Build & Test

Frontend (from repo root):

```bash
npm install
npm run dev          # vite dev server only
npm run tauri dev    # full Tauri app in dev mode
npm run tauri build  # production build
```

Backend (from `src-tauri/`):

```bash
cargo build
cargo test
```

CI currently builds release binaries on push to a `release` branch via `.github/workflows/build_for_release.yml`.
There is no PR-triggered test workflow yet (tracked in an open issue).

## Architecture

```
src/                          # React frontend (Vite + Chakra UI)
├── App.jsx                   # top-level layout; owns text/file input and results state
├── analysis.jsx              # invoke() wrappers calling into the Tauri commands below
└── components/                # Chakra UI components (inputs, modal, result panels, toggle)

src-tauri/src/
├── main.rs                   # registers the #[tauri::command] handlers below
├── models.rs                  # FastqSeqResult / FastaSeqResult (serde structs)
├── analysis/
│   ├── analysers.rs           # analyse_fastq_records / analyse_fasta_records — GC%, ORF count, Phred score
│   └── commands.rs            # tauri commands: *_sequences (raw text) and *_file (path) for both formats
└── services/
    └── io.rs                   # file readers (transparent .gz extraction), save_results / load_results (JSON)
```

## Key Types

- `FastqSeqResult` / `FastaSeqResult` (`models.rs`) — `id`, `desc`, `gc`, `n_orfs`, `is_valid`, `seq_len`,
  `result_type` (`"fastq"`/`"fasta"`); `FastqSeqResult` additionally carries `phred_score`.
- Tauri commands (`analysis/commands.rs`): `analyse_fastq_sequences`, `analyse_fastq_file`,
  `analyse_fasta_sequences`, `analyse_fasta_file`. The `_file` variants transparently gunzip `.gz` inputs and delete
  the extracted copy afterwards.

## Current Capabilities

- Both FASTA and FASTQ are supported end-to-end, from pasted text or a file, with transparent gzip decompression.
- Record analysis is single-threaded — `analyse_fastq_records`/`analyse_fasta_records` loop over records
  sequentially (tracked in an open issue for multithreaded analysis).
- Results round-trip through JSON via `services::io::save_results` / `load_results`.

## Coding Conventions

Rust (`src-tauri`):
- Types: `PascalCase`; functions/variables: `snake_case`
- `#[cfg(test)] mod tests` block at the bottom of each file; test functions prefixed `test_`
- Keep `#[tauri::command]` functions in `analysis/commands.rs` thin — parsing and scoring logic belongs in
  `analysers.rs` or `services/`

Frontend (`src`):
- Function components with hooks (`useState`/`useRef`) — no class components
- Chakra UI for all UI elements, pending the frontend migration decision

## Development Workflow

When implementing a feature or bug fix:

0. **Create a GitHub issue** if one describing the work doesn't already exist, using the body format from
   `.github/ISSUE_TEMPLATE.md`:
   ```bash
   gh issue create --title "<title>" --body "<description>"
   ```
1. **Ensure `main` is up to date** before branching:
   ```bash
   git checkout main && git pull
   ```
2. **Create a branch** with a descriptive name:
   ```bash
   git checkout -b feature/<name>   # new functionality
   git checkout -b fix/<name>       # bug fix
   ```
3. **Develop** the requested change.
4. **Commit** to the branch. Group commits by type and file:
   - App code per file
   - Tests per file, separate from app code
   - `CLAUDE.md`/`AGENTS.md` updates last
5. **Open a PR** using the `gh` CLI, with a body matching `.github/pull_request_template.md`:
   ```bash
   gh pr create --title "<title>" --body "..."
   ```

## Dependencies

Backend:
- `bio` — FASTA/FASTQ parsing, GC content, ORF finding
- `flate2` — gzip decompression
- `serde` / `serde_json` — result (de)serialization
- `uuid` — used in test fixtures
- `tauri` — desktop app shell and command bridge

Frontend:
- `@chakra-ui/react`, `@emotion/react`, `@emotion/styled`, `framer-motion` — UI
- `@tauri-apps/api` — `invoke()` bridge to the Rust commands
