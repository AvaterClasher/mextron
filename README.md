<!-- @format -->

# Mextron

[![crates.io](https://img.shields.io/crates/v/mextron)](https://crates.io/crates/mextron)
[![Build & test](https://github.com/AvaterClasher/mextron/actions/workflows/build_test.yml/badge.svg)](https://github.com/AvaterClasher/mextron/actions/workflows/build_test.yml)
[![Publish to Pages](https://github.com/AvaterClasher/mextron/actions/workflows/static.yml/badge.svg)](https://github.com/AvaterClasher/mextron/actions/workflows/static.yml)

A blazing fast static site generator in Rust

> 🚧 This project is currently under development. Expect breaking changes. 🚧

A sleek and minimalist static site generator written in Rust. Designed with simplicity in mind, Mextron makes website creation a breeze. It supports Markdown files, allowing you to write content with ease.

### Installation

You can install Mextron using Cargo:

```bash
cargo install mextron
```

### Create new project

You can initialise a new project using `new` command.

```bash
mextron new <folder>
```

You can optionally specify a theme also.

```bash
mextron new <folder> -t pico
```

### Features

-   [x] Markdown support
-   [x] Custom Themes
-   [x] Syntax highlighting
-   [x] SEO

### Project Structure

The following folder structure is expected by Mextron:

```
.
├── pages
│  ├── page.md
│  └── path
│     ├── custom-url.md
│     └── page.md
├── public
│  └── favicon.ico
├── Settings.toml
└── theme
   ├── app.hbs
   ├── global.css
   └── post.hbs
```

The `docs` folder is the input directory of the project and is always specified while running dev server or building. You can specify a different input directory like this:

```bash
mextron dev <input-dir-path>
```

-   The `Settings.toml` file contains the settings of the website, you can customize the website by changing the values in this file.
-   The `public` folder contains all the static assets of the website, these files are copied as-is to the output directory.
-   The `pages` folder contains all the Markdown files, this is where you write your content.
-   The `theme` folder contains all site templates and styles. It is written using [handlebars](https://handlebarsjs.com/guide/) syntax.
-   The `global.css` file contains the global CSS of the website, you can write your own CSS in this file.
