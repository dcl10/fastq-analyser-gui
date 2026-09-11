# CLAUDE.md

## Project Overview

`fastq-analyser-gui` is a Tauri desktop app that analyses DNA sequence records. Users paste FASTA/FASTQ text or pick
a file (optionally gzip-compressed), and the app reports per-record statistics: GC content, open reading frame (ORF)
count, sequence length, validity, and — for FASTQ — a Phred quality score. Results can be saved to and reloaded from
JSON.

## Build & Test

Frontend (from repo root):

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
│   └── page.tsx                # thin server component rendering FastqAnalyserApp
├── components/
│   ├── fastq-analyser-app.tsx  # 'use client' — owns text/file input and results state (was App.jsx)
│   ├── file-input.tsx, text-input.tsx, format-toggle.tsx, loading-indicator.tsx,
│   │   results-dialog.tsx, fasta-result-panel.tsx, fastq-result-panel.tsx
│   └── ui/                      # shadcn/ui primitives (button, dialog, accordion, input, textarea, switch, label)
├── lib/
│   ├── analysis.ts             # invoke() wrappers calling into the Tauri commands below (was analysis.jsx)
│   └── utils.ts                  # shadcn's `cn()` class-merging helper
└── types/
    └── results.ts               # FastaSeqResult / FastqSeqResult TS interfaces mirroring models.rs

src-tauri/src/
├── main.rs / lib.rs           # registers the #[tauri::command] handlers below, plugins (dialog, log)
├── models.rs                  # FastqSeqResult / FastaSeqResult (serde structs)
├── analysis/
│   ├── analysers.rs           # analyse_fastq_records / analyse_fasta_records — GC%, ORF count, Phred score
│   └── commands.rs            # tauri commands: *_sequences (raw text) and *_file (path) for both formats
└── services/
    └── io.rs                   # file readers (transparent .gz extraction), save_results / load_results (JSON)
```

Tauri bundles a static frontend — no Node server ships in the app. `next.config.ts` sets `output: "export"` so
`next build` emits a static `out/` directory that `tauri.conf.json`'s `frontendDist` points at. This rules out
Next.js server actions/API routes/ISR; all app logic lives in Rust `#[tauri::command]`s invoked from the client.
`next.config.ts` also sets `agentRules: false` — otherwise `next dev`/`next build` injects its own block into
`AGENTS.md` on every run, which would fight the hand-written file kept identical to `CLAUDE.md` here.

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
- `uuid` — used in test fixtures
- `tauri` — desktop app shell and command bridge

Frontend:
- `next`, `react`, `react-dom` — app framework, statically exported for Tauri
- `tailwindcss`, `shadcn` (dev-time CLI), `lucide-react` — styling and UI primitives/icons
- `@tauri-apps/api` — `invoke()` bridge to the Rust commands
- `@tauri-apps/plugin-dialog` — native file-open dialog (paired with `tauri-plugin-dialog` on the Rust side and the
  `dialog:default` permission in `src-tauri/capabilities/default.json`)
