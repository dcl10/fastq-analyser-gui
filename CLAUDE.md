# CLAUDE.md

## Project Overview

`fastq-analyser-gui` is a Tauri desktop app that analyses DNA sequence records. Users paste FASTA/FASTQ text or pick
a file (optionally gzip-compressed), and the app reports per-record statistics: GC content, open reading frame (ORF)
count, sequence length, validity, and — for FASTQ — a Phred quality score. Analysis runs can be saved to, listed from,
reloaded from and deleted from a local SQLite database.

## Build & Test

Frontend (from repo root):

The npm version is pinned via `"packageManager"` in `package.json` (currently `npm@11.6.2`) and enforced by
Corepack (`corepack enable`), so local installs and CI resolve to the exact same npm and `package-lock.json` doesn't
drift between npm major versions.

```bash
npm install
npm run dev          # next.js dev server only
npm run tauri dev    # full Tauri app in dev mode
npm run tauri build  # production build (static export + Rust release build + bundling)
npm run lint         # eslint
```

Backend (from `src-tauri/`):

```bash
cargo build
cargo test
```

CI: `.github/workflows/build_for_release.yml` predates the Tauri v2 migration and needs reworking into a manual
(`workflow_dispatch`) release workflow before it's usable again (tracked in an open issue). A `build_and_test`
workflow running backend `cargo build`/`cargo test` on PRs into `main` is tracked separately; add a frontend
`next build`/`npm run lint` step to it once this frontend exists.

## Architecture

```
src/
├── app/
│   ├── layout.tsx             # root layout, fonts, metadata
│   ├── page.tsx                # "/" — splash/welcome screen: motif, product blurb,
│   │   "Get started" (routes to /import via next/navigation's useRouter)
│   ├── import/page.tsx          # "/import" — windowed shell + FastqAnalyserApp
│   └── runs/
│       ├── page.tsx             # "/runs" — windowed shell + RunsList
│       └── detail/page.tsx      # "/runs/detail?id=<id>" — windowed shell + RunDetail (query param, not
│                                #   /runs/[id]: static export can't prerender ids that only exist in the DB)
├── components/
│   ├── app-shell.tsx, rail-nav.tsx, title-bar.tsx, toolbar.tsx, theme-switch.tsx,
│   │   theme-provider.tsx        # windowed shell chrome (see PR #38); rail items route to /<id>
│   ├── brand/                    # dna-motif.tsx, wordmark.tsx — brand components
│   ├── fastq-analyser-app.tsx  # 'use client' — text/file input; Submit analyses, saves the run, routes to /runs
│   ├── runs-list.tsx            # 'use client' — table of saved runs from list_runs; rows open the detail
│   │                            #   page, bin button confirms then deletes
│   ├── run-detail.tsx           # 'use client' — one run's records (load_run) as a table
│   ├── file-input.tsx, text-input.tsx, format-toggle.tsx, loading-indicator.tsx
│   └── ui/                      # shadcn/ui primitives (button, dialog, accordion, input, textarea, switch, label, table, alert-dialog)
├── lib/
│   ├── analysis.ts             # invoke() wrappers for the analyse_* commands, returning RunRecords
│   ├── runs.ts                 # invoke() wrappers for save_run / list_runs / load_run / delete_run
│   └── utils.ts                  # shadcn's `cn()` class-merging helper; `pluralise()` for count labels
└── types/
    ├── results.ts               # FastaSeqResult / FastqSeqResult TS interfaces mirroring models.rs
    └── runs.ts                  # Run / RunRecords TS types matching models.rs's serde JSON shape

src-tauri/src/
├── main.rs / lib.rs           # registers the #[tauri::command] handlers below, plugins (dialog, log), AppState (db pool)
├── models.rs                  # FastqSeqResult / FastaSeqResult / Run / RunRecords (serde structs)
├── analysis/
│   └── analysers.rs           # analyse_fastq_records / analyse_fasta_records — GC%, ORF count, Phred score
├── commands/
│   ├── analysis.rs            # tauri commands: *_sequences (raw text) and *_file (path) for both formats
│   └── run.rs                 # tauri commands: save_run / load_run / list_runs / delete_run
├── data/
│   ├── db.rs                  # init_db — SQLite pool (WAL, foreign keys on) + runs migrations
│   └── entities.rs            # Run / Record / ResultType row types (sqlx::FromRow)
└── services/
    ├── io.rs                  # file readers (transparent .gz extraction)
    ├── run.rs                 # create_run / list_runs / get_run_with_records / delete_run
    └── record.rs              # batch record inserts (per format) / list_records_for_run

src-tauri/migrations/          # sqlx migrations (runs, records with ON DELETE CASCADE)
```

Tauri bundles a static frontend — no Node server ships in the app. `next.config.ts` sets `output: "export"` so
`next build` emits a static `out/` directory that `tauri.conf.json`'s `frontendDist` points at. This rules out
Next.js server actions/API routes/ISR; all app logic lives in Rust `#[tauri::command]`s invoked from the client.
`next.config.ts` also sets `agentRules: false` — otherwise `next dev`/`next build` injects its own block into
`AGENTS.md` on every run, which would fight the hand-written file kept identical to `CLAUDE.md` here.

## Key Types

- `FastqSeqResult` / `FastaSeqResult` (`models.rs`) — `id`, `desc`, `gc`, `n_orfs`, `seq_len`,
  `result_type` (`"fastq"`/`"fasta"`); `FastqSeqResult` additionally carries `phred_score`.
- `Run` (`models.rs`) — `id`, `created_at`, `result_type`, and `records: RunRecords` (`FastaRecords(Vec<...>)` /
  `FastqRecords(Vec<...>)`). `data/entities.rs` holds the matching row types; services convert between the two with
  `From` impls so callers only see `models.rs` types.
- Analysis commands (`commands/analysis.rs`): `analyse_fastq_sequences`, `analyse_fastq_file`,
  `analyse_fasta_sequences`, `analyse_fasta_file`. The `_file` variants transparently gunzip `.gz` inputs and delete
  the extracted copy afterwards. They're `#[tauri::command(async)]` so analysis runs off the main thread and doesn't
  freeze the window.
- Run commands (`commands/run.rs`): `save_run` (returns the new run id), `load_run`, `list_runs`, `delete_run`.

## Current Capabilities

- Both FASTA and FASTQ are supported end-to-end, from pasted text or a file, with transparent gzip decompression.
- Record analysis is single-threaded — `analyse_fastq_records`/`analyse_fasta_records` loop over records
  sequentially (tracked in an open issue for multithreaded analysis).
- Runs persist to SQLite via `services::run`. `create_run` inserts the run and its records in a single transaction;
  `delete_run` relies on `ON DELETE CASCADE` to remove records.

## Coding Conventions

Rust (`src-tauri`):

- Types: `PascalCase`; functions/variables: `snake_case`
- `#[cfg(test)] mod tests` block at the bottom of each file; test functions prefixed `test_`
- Keep `#[tauri::command]` functions in `commands/` thin — parsing and scoring logic belongs in `analysers.rs`, and
  database/file logic in `services/`

Frontend (`src`):

- TypeScript function components with hooks (`useState`/`useRef`) — no class components
- Tailwind CSS + shadcn/ui for UI elements; add new primitives with `npx shadcn@latest add <component>` rather than
  hand-rolling them
- Keep generated `src/components/ui/*` files as shadcn produces them — customize via `className`/Tailwind at the
  call site, not by hand-editing the primitives

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
- `sqlx` (SQLite, tokio runtime, migrations) / `chrono` — local run storage and timestamps
- `uuid` — used in test fixtures
- `tauri` — desktop app shell and command bridge

Frontend:

- `next`, `react`, `react-dom` — app framework, statically exported for Tauri
- `tailwindcss`, `shadcn` (dev-time CLI), `lucide-react` — styling and UI primitives/icons
- `@tauri-apps/api` — `invoke()` bridge to the Rust commands
- `@tauri-apps/plugin-dialog` — native file-open dialog (paired with `tauri-plugin-dialog` on the Rust side and the
  `dialog:default` permission in `src-tauri/capabilities/default.json`)
