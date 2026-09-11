# FastQ Analyser
A desktop app that analyses FASTA and FastQ records and displays statistics about each one.

## Download
Get FastQ Analyser from https://github.com/dcl10/fastq-analyser-gui/releases

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
Copy and paste one or many FASTA/FastQ records into the text area, or select a FASTA/FastQ file (optionally gzip-compressed) and click "Submit". The results for each record will then appear on the screen.
