<!-- @format -->

> 🚧 This project is currently under development. Expect breaking changes. 🚧

A sleek and minimalist static site generator written in Rust. Designed with simplicity in mind, Mextron makes website creation a breeze.It supports Markdown files, allowing you to write content with ease.

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
-   [x] Custom Metadata Passthrough

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

### Building custom pages

A great example would be a blog index page where you show a list of posts and link to them. This can be achieved by accessing the site directory that is passed to every page.
The site directory can be accessed through the root object, this is available in every page and it represents the entire site structure including its metadata, so I can render a blog index page like this:

A custom template say `blog`, with lists all pages under `blog` folder.

```handlebars
<ul>
	{{#each root.blog}}
		{{#if (not (eq @key "_self"))}}
			<hgroup>
				<h4><a href="{{@key}}/">{{this.title}}</a></h4>
				<h2>{{this.author}}</h2>
			</hgroup>
		{{/if}}
	{{/each}}
</ul>
```

Then define a new page under blog folder and specify the template as `blog` which we have created as shown above.

```md
--
template: blog
title: ~/Mextron/blog
--

### This is a blog index
```
