# mdbook-emoji

[![build](https://github.com/almereyda/mdbook-emoji/workflows/build/badge.svg)](https://github.com/almereyda/mdbook-emoji/actions?query=workflow%3Abuild)
[![dependency status](https://deps.rs/repo/github/almereyda/mdbook-emoji/status.svg)](https://deps.rs/repo/github/almereyda/mdbook-emoji)
[![Crates.io](https://img.shields.io/crates/v/mdbook-emoji)](https://crates.io/crates/mdbook-emoji)

[mdBook](https://github.com/rust-lang/mdBook) preprocessor that replaces straight quotes with curly quotes, except within code blocks or code spans.

It adds an **emoji** option for the mdBook renderers.

## Usage

The following example configures `mdbook-emoji` as a preprocessor for the HTML renderer.

```toml
[book]
title = "Example book"
author = "John Doe"

# add the emoji preprocessor
[preprocessor.emoji]
# select renderers
renderer = ["html"]

[output.html]
```

More on configuring preprocessors can be found in the [mdBook Documentation](https://rust-lang.github.io/mdBook/format/config.html#configuring-preprocessors).
