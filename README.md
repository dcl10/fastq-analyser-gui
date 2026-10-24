# FastQ Analyser
A desktop app that analyses FASTA and FastQ records and reports statistics about each one: GC content, open reading frame (ORF) count, sequence length, validity and, for FASTQ, a Phred quality score. No code or scripting needed, and files many gigabytes in size are streamed rather than loaded into memory.

## Download
Get FastQ Analyser from the [releases page](https://github.com/dcl10/fastq-analyser-gui/releases/latest). Pick the file for your system:

| System | File |
|---|---|
| macOS (Apple Silicon and Intel) | `FastQ.Analyser_<version>_universal.dmg` |
| Windows | `FastQ.Analyser_<version>_x64-setup.exe` or `FastQ.Analyser_<version>_x64_en-US.msi` |
| Linux | `FastQ.Analyser_<version>_amd64.AppImage`, `.deb` (Debian/Ubuntu) or `.rpm` (Fedora/RHEL) |

### Opening an unsigned build
Releases aren't code-signed yet, so your system will warn you the first time you open the app:

- **macOS**: open the `.dmg` and drag FastQ Analyser to Applications, then open it. When macOS says it can't verify the developer, go to **System Settings → Privacy & Security**, scroll down and click **Open Anyway**.
- **Windows**: if SmartScreen shows "Windows protected your PC", click **More info → Run anyway**.
- **Linux**: make the AppImage executable with `chmod +x FastQ.Analyser_*.AppImage`, then run it.

## Build from source
To build this program from source, you will need to install [Node.js](https://nodejs.org/en/download/) and [Rust](https://www.rust-lang.org/tools/install). The frontend is a [Next.js](https://nextjs.org/) app (statically exported for Tauri), and the desktop shell is [Tauri](https://tauri.app/) v2.

```
git clone git@github.com:dcl10/fastq-analyser-gui.git
cd fastq-analyser-gui
npm install
npm run tauri build
```

While developing, `npm run tauri dev` runs the app with hot reload.

## How to use
1. On the **Import** page, choose FASTA or FASTQ, then either paste one or many records into the text area or select a file.
2. Click **Submit**. Files are analysed and saved as they're read, with progress shown as a record count and a percentage of the file.
3. Each analysis is saved as a run. The **Runs** page lists them; open one to page through its per-record results, or use the bin button to delete it.

Analysis runs in parallel across all CPU cores by default. To change the number of threads, click the settings gear in the title bar. Saving the setting restarts the app.

Accepted file extensions:

- FASTA: `.fa`, `.fna`, `.fasta`
- FASTQ: `.fq`, `.fastq`

Any of these may also be gzip-compressed with a trailing `.gz` (e.g. `reads.fq.gz`).
