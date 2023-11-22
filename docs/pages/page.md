<!-- @format -->
[![crates.io](https://img.shields.io/crates/v/mextron)](https://crates.io/crates/mextron)
[![Build & test](https://github.com/AvaterClasher/mextron/actions/workflows/build_test.yml/badge.svg)](https://github.com/AvaterClasher/mextron/actions/workflows/build_test.yml)
[![Publish to Pages](https://github.com/AvaterClasher/mextron/actions/workflows/static.yml/badge.svg)](https://github.com/AvaterClasher/mextron/actions/workflows/static.yml)

> 🚧 This project is currently under development. Expect breaking changes. 🚧

A sleek and minimalist static site generator written in Rust. Designed with simplicity in mind, Mextron makes website creation a breeze.


### Installation

You can install Mextron using Cargo:

```bash
cargo install mextron
```

### Features

-   [x] Markdown support
-   [x] Customizable
-   [x] Syntax Highlighting
-   [x] SEO

### Project Structure

The following folder structure is expected by Mextron:

```
docs/
├─ public/
│  ├─ favicon.ico
├─ pages/
│  ├─ page.md
│  ├─ about/
│  │  ├─ page.md
├─ Settings.toml
├─ global.css
```

The `docs` folder is the input directory of the project and is always specified while running dev server or building. You can specify a different input directory like this:

```bash
mextron dev <input-dir-path>
```

-   The `public` folder contains all the static assets of the website, these files are copied as-is to the output directory.
-   The `pages` folder contains all the Markdown files, this is where you write your content.
-   The `Settings.toml` file contains the settings of the website, you can customize the website by changing the values in this file.
-   The `global.css` file contains the global CSS of the website, you can write your own CSS in this file.
