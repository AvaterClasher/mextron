<!-- @format -->

# Mextron

[![crates.io](https://img.shields.io/crates/v/mextron)](https://crates.io/crates/mextron)
![Crates.io](https://img.shields.io/crates/d/mextron)
[![Build & test](https://github.com/AvaterClasher/mextron/actions/workflows/build_test.yml/badge.svg)](https://github.com/AvaterClasher/mextron/actions/workflows/build_test.yml)
[![Publish to Pages](https://github.com/AvaterClasher/mextron/actions/workflows/static.yml/badge.svg)](https://github.com/AvaterClasher/mextron/actions/workflows/static.yml)
<br/>
![Art by Crayon](https://mextron.vercel.app/images/og.png)


A blazing fast static site generator in Rust

> 🚧 This project is currently under development. Expect breaking changes. 🚧

> 🚧 This Project is currently POSIX friendly. For windows machines please use [WSL](https://learn.microsoft.com/en-us/windows/wsl/install)🚧

A sleek and minimalist static site generator written in Rust. Designed with simplicity in mind, Mextron makes website creation a breeze. It supports Markdown files, allowing you to write content with ease.

### DEMO

Here is a live [DEMO](https://mextron.vercel.app) my blog is built using Mextron.

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

You can optionally specify a theme also with the feature flag `-t`.

```bash
mextron new <folder> -t pico
```
> **Disclaimer** - Using this command can return an error due to github api rate limits. Please try again in such situations.

### Running Mextron in Dev mode

You can run mextron in the dev mode by using the `dev` mode
There is also a `-w` i.e `watch` feature flag in dev mode for using hot reloading. 

```bash
mextron dev -w # If you are in the Project Directory 
# OR
mextron dev <INPUT_DIRECTORY> -w # If you wanna specify which folder to run
```

### Running Mextron in Build mode

You can run mextron in the build mode by using the `build` mode

```bash
mextron build # If you are in the Project Directory 
# OR
mextron build <INPUT_DIRECTORY> # If you wanna specify which folder to run
```

### Features

-   [x] Markdown support
-   [x] Custom Themes
-   [x] Syntax highlighting
-   [x] SEO
-   [x] Custom Metadata Passthrough

### Project Structure

The following folder structure is expected by Mextron:

```
.
├── pages
│  ├── about
│  │  └── page.md
│  ├── blog
│  │  ├── page.md
│  │  └── why-learn-rust.md
│  └── page.md
├── public
│  ├── favicon.ico
│  ├── flamethrower.js
│  └── images
│     └── og.png
├── Settings.toml
└── theme
   ├── app.hbs
   ├── blog.hbs
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

## The `Settings.toml` file

The `Settings.toml` file contains the settings of the website, you can customize the website by changing the values in this file.

```toml
[dev]
port = 3000 # The port on which the dev server runs
ws_port = 3001 # The port on which the dev server websocket runs, for hot reloading

[site]
script_urls = [] # List of script urls to be included in the site
style_urls = [ # List of style urls to be included in the site
  'https://cdn.jsdelivr.net/npm/@picocss/pico@1/css/pico.min.css',
  'https://cdn.jsdelivr.net/npm/prismjs@1.29.0/themes/prism-tomorrow.min.css',
]

[meta]
title = "~/Mextron" # The title of the website
description = "Blazing fast static site generator written in Rust" # The description of the website
og_image_url = "https://mextron.vercel.app/images/og.png" # The og image url of the website
base_url = "https://mextron.vercel.app" # The base url of the website, used for building sitemap

[navigation] # The navigation links of the website
links = [
  { label = "~/", url = "/" },
  { label = "GitHub", url = "https://github.com/AvaterClasher/mextron" },
  { label = "Website", url = "https://soumyadipmoni.netlify.app" },
  { label = "Blog", url = "/blog/" },
  { label = "About", url = "/about/" },
]

[data] # The data to be passed to every page, can be accessed using `data` object in every page
author = "Soumyadip Moni"
author_link = "https://github.com/AvaterClasher"

[remote_data] # The remote data to be fetched and passed to every page, can be accessed using `remote_data` object
repo_meta = "https://api.github.com/repos/AvaterClasher/mextron" # The url of the remote data
```

## Handlebars Helpers

Mextron provides a few handlebars helpers to make your life easier. This project uses [handlebars-rust](https://crates.io/crates/handlebars) and hence all the helpers provided by it are available. Apart from that, Mextron provides the following helpers:

-   `slice`: Slices an array and returns the sliced array.
-   `sort-by`: Sorts an array of objects by a key.
-   `format-date`: Formats a date using the given format.
-   `stringify`: Converts a value to string, this is useful for debugging.

You can find examples of these helpers in the [demo project](https://mextron.vercel.app/blog).

## Deployment

You can build the site using the build command:

```bash
mextron build <input-dir-path>
```

The build outputs are saved to `_site` folder. So, you can deploy the website by copying the `_site` folder to your web server. You can also use GitHub pages to host your website. Here is an example GitHub action to deploy your website to GitHub pages:

```yaml
# Simple workflow for deploying static content to GitHub Pages
name: Publish to Pages

on:
    # Runs on pushes targeting the default branch
    push:
        branches: ["master"]

    # Allows you to run this workflow manually from the Actions tab
    workflow_dispatch:

# Sets permissions of the GITHUB_TOKEN to allow deployment to GitHub Pages
permissions:
    contents: read
    pages: write
    id-token: write

# Allow only one concurrent deployment, skipping runs queued between the run in-progress and latest queued.
# However, do NOT cancel in-progress runs as we want to allow these production deployments to complete.
concurrency:
    group: "pages"
    cancel-in-progress: false

jobs:
    # Single deploy job since we're just deploying
    deploy:
        environment:
            name: github-pages
            url: ${{ steps.deployment.outputs.page_url }}
        runs-on: ubuntu-latest
        steps:
            - name: Checkout
              uses: actions/checkout@v3
            - name: Setup Pages
              uses: actions/configure-pages@v3
            - name: Install
              run: cargo install mextron
            - name: Build
              run: mextron build src # Replace src with your input directory
            - name: Upload artifact
              uses: actions/upload-pages-artifact@v1
              with:
                  # Upload entire repository
                  path: "./_site"
            - name: Deploy to GitHub Pages
              id: deployment
              uses: actions/deploy-pages@v2
```

# LICENSE

You can find the license [here](https://github.com/AvaterClasher/mextron/blob/master/LICENSE).
