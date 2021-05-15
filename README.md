<img src="doc/tiki_head.jpg" width="154" height="223" align="right">

# Tikibase

![build status](https://github.com/kevgo/tikibase/actions/workflows/ci.yml/badge.svg)

_the un-database_

Whatever knowledge base you use and love today, it will be outdated in 5 years
and unsupported or unavailable in 10 years. All your content will be gone with
it. Tikibase is a knowledge base with focus on longetivity and flexibilty. Using
a Tikibase doesn't require a special database server or viewer application. It
consists of a set of standard Markdown files. You read, write, and organize
these files using the text or Markdown viewer/editor of your choice. You manage
the files using a version control system like Git or Mercurial. A Tikibase is
available on all current and future compute platforms that can display text.

The CLI application in this repository helps maintain a Tikibase by finding and
sometimes fixing various issues:

- broken links
- broken images
- images not referenced in a Markdown document
- missing backlinks
- inconsistent section capitalization
- duplicate sections
- empty sections
- unknown sections
- section order

### installation

Download the [binary](https://github.com/kevgo/tikibase/releases/latest) for
your platform or install from source:

- [install Rust](https://rustup.rs) nightly
- `cargo install --git https://github.com/kevgo/tikibase.git`
- add `~/.cargo/bin` to your shell's `$PATH`

### usage

- on your developer machine: run `tikibase ps` in the folder with the Markdown
  files
- in your tests/CI: run `tikibase check`
- to see all available commands: `tikibase help`

### configuration

To configure this tool, create a file `tikibase.json` in the directory
containing your Tikibase. The available options are:

- **allowed_sections**: an array of section names that are only allowed

### related

You might want to also use
[VSCode Markdown IDE](https://github.com/kevgo/vscode-markdown-ide).
