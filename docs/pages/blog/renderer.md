---
template: post
title: "Renderer and Parser"
author: Soumyadip Moni
author_link: https://soumyadipmoni.netlify.app
date_published: 19th Nov 2023
---

# How Mextron's Renderer and Parser kind of works ?

### **Kind of** because i suck at explaining things 😄
### If somebody has a better way of explaining this please contact me 🙏

The Rust programming language, known for its performance and safety, is gaining popularity in the web development landscape. In this blog post, we'll delve into the `Render` struct and the `get_markdown_and_metadata` function.

## The `Render` Struct

The `Render` struct serves as a key component for rendering content, encapsulating various functionalities that contribute to the dynamic generation of web pages. Let's break down its main components:

- **file**: The path to the Markdown file that contains the content to be rendered.
- **theme_dir**: The directory where theme-related assets and templates are stored.
- **settings**: Configuration settings for the web application.
- **cache**: An optional cache for storing and retrieving assets, enhancing performance.
- **handlebars**: An instance of the Handlebars templating engine, configured with custom helpers.

## Initialization and Configuration

The `Render` struct is initialized with essential parameters, including the file path, theme directory, application settings, and an optional cache.

## Rendering Pages

The core functionality of the `Render` struct lies in its ability to render web pages. The `render_page` function orchestrates this process by handling Markdown content, metadata extraction, and template rendering. Let's explore the steps involved:

### Markdown and Metadata Extraction

The `get_markdown_and_metadata` function reads the content of the Markdown file and uses regular expressions to extract both the metadata and the Markdown body. If metadata is present, it is parsed into a YAML format for further use.

### Body Rendering

The `render_body` function takes the Markdown body, metadata, and site directory information to render the body content. It checks if a specific template is defined in the metadata and, if so, uses it to render the body. This provides flexibility in customizing the rendering process based on individual page requirements.

### Template Rendering

The final rendering step occurs in the `render_page` function. The Handlebars templating engine is employed to merge data, metadata, and other relevant information into a cohesive HTML output. This includes global styles, scripts, and various site-specific settings.

Certainly! Let's delve deeper into the `render_page` function, examining its steps and the logic behind each part of the process.

### Rendering Pages with `render_page`

The `render_page` function is the heart of the `Render` struct, orchestrating the rendering process for web pages. It takes the site directory information as a parameter and returns the final HTML output as a `Result<String>`.

```rust
pub fn render_page(&self, site_directory: &serde_yaml::Value) -> Result<String> {
    // Step 1: Extract Markdown content and metadata
    let (metadata, markdown) = self.get_markdown_and_metadata()?;

    // Step 2: Parse metadata into YAML format
    let metadata = if let Some(metadata) = metadata {
        let metadata = utils::parse_string_to_yaml(&metadata)?;
        Some(metadata)
    } else {
        None
    };

    // Step 3: Render the body content
    let content = if let Some(metadata) = &metadata {
        self.render_body(&markdown, metadata, site_directory)
            .with_context(|| format!("Failed to render page: {}", self.file))?
    } else {
        markdown
    };

    // Step 4: Render the entire page using Handlebars template
    let html = self.handlebars.render_template(
        &self.get_template("app").context("Failed to get template")?,
        &AppRenderData {
            title: self.settings.meta.title.clone(),
            description: self.settings.meta.description.clone(),
            open_graph_tags: seo::generate_open_graph_tags(&self.settings)?,
            content,
            styles: self.get_global_styles()?,
            scripts: self.get_global_scripts()?,
            links: self.settings.navigation.links.clone(),
            page_metadata: metadata,
            data: self.settings.data.clone(),
            remote_data: self.get_remote_data()?,
        },
    )
    .context("Failed to render page")?;

    // Step 5: Return the final HTML output
    Ok(html)
}
```

### Step-by-Step Breakdown of Render_page()

#### Step 1: Extract Markdown content and metadata

The function starts by calling the `get_markdown_and_metadata` method to extract both the metadata and the Markdown content from the file.

```rust
let (metadata, markdown) = self.get_markdown_and_metadata()?;
```

#### Step 2: Parse metadata into YAML format

If metadata is present, it is parsed into a YAML format for further processing. The `utils::parse_string_to_yaml` function is used for this conversion.

```rust
let metadata = if let Some(metadata) = metadata {
    let metadata = utils::parse_string_to_yaml(&metadata)?;
    Some(metadata)
} else {
    None
};
```

#### Step 3: Render the body content

The function then proceeds to render the body content using the `render_body` method. This method takes the Markdown body, parsed metadata, and site directory information as parameters.

```rust
let content = if let Some(metadata) = &metadata {
    self.render_body(&markdown, metadata, site_directory)
        .with_context(|| format!("Failed to render page: {}", self.file))?
} else {
    markdown
};
```

#### Step 4: Render the entire page using Handlebars template

The next step involves rendering the entire page using the Handlebars templating engine. The `AppRenderData` struct is populated with various data, including the title, description, open graph tags, content, styles, scripts, links, metadata, and remote data.

```rust
let html = self.handlebars.render_template(
    &self.get_template("app").context("Failed to get template")?,
    &AppRenderData {
        title: self.settings.meta.title.clone(),
        description: self.settings.meta.description.clone(),
        open_graph_tags: seo::generate_open_graph_tags(&self.settings)?,
        content,
        styles: self.get_global_styles()?,
        scripts: self.get_global_scripts()?,
        links: self.settings.navigation.links.clone(),
        page_metadata: metadata,
        data: self.settings.data.clone(),
        remote_data: self.get_remote_data()?,
    },
)
.context("Failed to render page")?;
```

#### Step 5: Return the final HTML output

Finally, the function returns the rendered HTML as a `Result<String>`.

```rust
Ok(html)
```

## The Markdown and Metadata parser

Certainly! Let's take a closer look at the `get_markdown_and_metadata` function, which is responsible for reading the content of a Markdown file, extracting metadata, and separating it from the Markdown body.

```rust
fn get_markdown_and_metadata(&self) -> Result<(Option<String>, String)> {
    // Step 1: Read the content of the Markdown file
    let markdown = fs::read_to_string(&self.file)?;

    // Step 2: Define a regular expression to capture metadata
    let metadata_regex = Regex::new(r"^(?s)---(.*?)---(.*)")
        .context("Failed to parse metadata from markdown file")?;

    // Step 3: Attempt to match the metadata regex with the Markdown content
    if let Some(captures) = metadata_regex.captures(&markdown) {
        // Step 4: Extract metadata and Markdown body
        let metadata = captures
            .get(1)
            .with_context(|| format!("Failed to get metadata from captures: {}", self.file))?
            .as_str();
        let markdown_body = captures
            .get(2)
            .with_context(|| format!("Failed to get markdown from captures: {}", self.file))?
            .as_str();

        // Step 5: Return metadata and Markdown body
        Ok((Some(metadata.to_string()), markdown_body.to_string()))
    } else {
        // Step 6: If no metadata is found, return Markdown content without metadata
        Ok((None, markdown))
    }
}
```

### Step-by-Step Breakdown of get_markdown_and_metadata()

#### Step 1: Read the content of the Markdown file

```rust
let markdown = fs::read_to_string(&self.file)?;
```

This step uses the `fs::read_to_string` function to read the content of the Markdown file specified by `self.file`. The result is a `String` containing the entire content of the Markdown file.

#### Step 2: Define a regular expression to capture metadata

```rust
let metadata_regex = Regex::new(r"^(?s)---(.*?)---(.*)")
    .context("Failed to parse metadata from markdown file")?;
```

A regular expression (`metadata_regex`) is defined using the `Regex::new` function. This regex is designed to capture metadata enclosed between `---` delimiters at the beginning of the Markdown content. The `(?s)` flag allows the dot (`.`) in the regex to match newline characters.

#### Step 3: Attempt to match the metadata regex with the Markdown content

```rust
if let Some(captures) = metadata_regex.captures(&markdown) {
```

This step attempts to match the defined regular expression (`metadata_regex`) with the entire content of the Markdown file. If a match is found, the `captures` variable will contain information about the matched groups.

#### Step 4: Extract metadata and Markdown body

```rust
let metadata = captures
    .get(1)
    .with_context(|| format!("Failed to get metadata from captures: {}", self.file))?
    .as_str();
let markdown_body = captures
    .get(2)
    .with_context(|| format!("Failed to get markdown from captures: {}", self.file))?
    .as_str();
```

If a match is found, this step extracts the metadata and Markdown body from the captured groups. The `captures.get(1)` corresponds to the first capturing group (metadata), and `captures.get(2)` corresponds to the second capturing group (Markdown body). The `as_str()` method converts the captured values to string slices.

#### Step 5: Return metadata and Markdown body

```rust
Ok((Some(metadata.to_string()), markdown_body.to_string()))
```

If metadata and Markdown body are successfully extracted, a tuple containing `Some(metadata)` and `markdown_body` is returned. Both values are converted to `String` for consistency.

#### Step 6: If no metadata is found, return Markdown content without metadata

```rust
Ok((None, markdown))
```

If the regular expression does not match (no metadata found), this step returns a tuple with `None` for metadata and the entire Markdown content as the Markdown body.

## Conclusion

By exploring the inner workings of the `Render` struct, we gain insights into how Rust's strengths in performance and safety extend to the domain of web development.
