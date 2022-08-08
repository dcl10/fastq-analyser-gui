# Tauri React + Vite Template
This repo serves as a template for making desktop apps using [Tauri](https://tauri.app/). The backend uses [Rust](https://www.rust-lang.org/) and the frontend uses [React](https://reactjs.org/) + [Vite](https://vitejs.dev/).

Unlike [Electron](https://www.electronjs.org/), Tauri takes advantage of Rust being a fast, memory-safe and minimalistic language, leading to smaller, less resource greedy applications. Also, it does not ship with chromium is the renderer, instead using [webview](https://webview.dev/). However, like Electron, you can still build beautiful frontends using your favourite Javascript frameworks. You can also invoke custom commands you write in Rust from the frontend. This way, you can leverage the power of Rust for more computationally intense tasks, for which Javascript is less well suited.

## Before you get started
Download Rust from https://www.rust-lang.org/tools/install. The page should display appropriate instructions for your OS.
  * NB: You will also need a C-compiler. The instructions will be on the page.

Download Node.js (LTS version) from https://nodejs.org/en/download/.

## Create a repo on Github
Go to https://github.com/dcl10/tauri-app-template and click "Use this template" to create a repo using this template.

## Adding new dependencies
To add new Node.js dependencies, in the root of the directory (where `package.json` is) use:
```
npm install <dep1> [<dep2> ...]
``` 
-- or -- 
```
yarn add <dep1> [<dep2> ...]
```
or any other Node.js package manager.

To add Rust dependencies, in the directory `src-tauri` run:
```
cargo add <dep1> [<dep2> ...]
```