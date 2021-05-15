<img src="doc/tiki_head.jpg" width="154" height="223" align="right">

# Tikibase

![build status](https://github.com/kevgo/tikibase/actions/workflows/ci.yml/badge.svg)

_the un-database_

Whatever note taking or knowledge base application you use and love today, it
will be outdated in 5 years and unsupported/unavailable in 10 years. All your
data will be gone with it.

Tikibase solves this problem. It is a knowledge base solution available on all
current and future compute platforms. Tikibase works without any particular
database server or viewer application. If your computer can display/edit text,
you can use a Tikibase on it.

A Tikibase is just a set of normal Markdown files. You view, change, and
organize these files using the text or Markdown viewer/editor of your choice.
You manage the files using a version control system like Git or Mercurial. The
CLI application in this repository is an optional linter that helps maintain a
Tikibase by finding/fixing a number of issues:

- **broken links:** Markdown or HTML links pointing to non-existing local files
- **broken images:** Markdown or HTML image tags pointing to non-existing files
- **orphaned resources:** non-Markdown files not referenced in a Markdown
  document
- **missing backlinks:** if document A links to document B, document B must also
- **links to the same document**
- **inconsistent section capitalization** link to document A
- **duplicate sections**
- **empty sections**
- **unknown sections:** when the config file contains a `sections` key
- **section order:** when the config file contains a `sections` key

### installation

Download the [binary](https://github.com/kevgo/tikibase/releases/latest) for
your platform or install from source:

- [install Rust](https://rustup.rs) nightly
- `cargo install --git https://github.com/kevgo/tikibase.git`
- add `~/.cargo/bin` to your shell's `$PATH`

### usage

- on your developer machine: run `tikibase ps` in the folder with the Markdown
  files (finds all issues, fixes the auto-fixable ones, and lists the rest)
- in your tests/CI: run `tikibase check` (lists all issues)
- to see all available commands: `tikibase help`

### configuration

Create a file `tikibase.json` in your Tikibase directory. Here is an example:

```json
{
  "ignore": ["Makefile"],
  "sections": ["foo", "bar"]
}
```

- **ignore:** list of files to ignore
- **sections:** if provided, accepts only sections with the given names, in the
  given order

### related

[VSCode Markdown IDE](https://github.com/kevgo/vscode-markdown-ide) allows
convenient editing of Tikibase content using VSCode.
