# FastQ Analyser
A desktop app that will analyse FastQ records and display statistics about each one.

## Download
Get FastQ Analyser from https://github.com/dcl10/fastq-analyser-gui/releases

## Build from source
To build this program from source, you will need to install [Node.js](https://nodejs.org/en/download/) and [Rust](https://www.rust-lang.org/tools/install).

```
git clone git@github.com:dcl10/fastq-analyser-gui.git
cd fastq-analyser-gui
npm install
npm run tauri build
```

## How to use
Copy and paste one or many FastQ records into the text area, or select a FastQ file and click "Submit". The results for each record will then appear on the screen.
