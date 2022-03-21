<img src="doc/tiki_head.jpg" width="154" height="223" align="right">

# Tikibase

![build status](https://github.com/kevgo/tikibase/actions/workflows/ci.yml/badge.svg)

_the un-database_

Whatever note taking or knowledge base application you use and love today, it
will be outdated in 5 years and unsupported/unavailable in 10 years. All your
data will be gone with it.

Tikibase solves this problem. It is a knowledge base available on all current
and future compute platforms. Tikibase works without any particular database
server or viewer application. If your computer can display/edit text files, you
can use it to work on your Tikibase.

A Tikibase is just a set of normal Markdown files in a folder. You view, change,
and organize these files using the text or Markdown editor of your choice. You
manage changes to the files in your knowledge base using a version control
system like Git or Mercurial. The CLI application in this repository is an
optional linter that helps maintain a Tikibase by finding and fixing a number of
issues:

- **broken links/images:** links pointing to non-existing local files
- **orphaned resources:** non-Markdown files not referenced by a Markdown
  document
- **missing backlinks:** if document A links to document B, document B must also
  link to document A
- **links to the same document:** document A should not contain links to
  document A
- **inconsistent section capitalization**
- **duplicate sections**
- **empty sections**
- **missing sources:** you can name sources by creating an ordered list in the
  `### links` section. You can reference sources like `[1]` in the document.

When the config file defines the allowed section names, Tikibase verifies these
additional properties:

- **unknown sections**
- **section order**

### installation

Download the [binary](https://github.com/kevgo/tikibase/releases/latest) for
your platform or install from source:

- [install Rust](https://rustup.rs) nightly
- `cargo install --git https://github.com/kevgo/tikibase.git`
- add `~/.cargo/bin` to your shell's `$PATH`

### usage

- on your developer machine: run `tikibase ps` ("pitstop") in the folder with
  the Markdown files. The pitstop command does everything it can after changes
  have been made: it finds all issues, fixes as many as it can, and lists the
  remaining ones.
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

- [VSCode Markdown IDE](https://github.com/kevgo/vscode-markdown-ide) allows
  convenient editing of Tikibase content using VSCode.
- [Obsidian](https://obsidian.md): a more fully featured Markdown-based
  knowledge base including its own editor/viewer and many plugins. Easier and to
  get started but - like all application-based solutions - will be outdated at
  some point and unavailable on future platforms.
- [TiddlyWiki](https://tiddlywiki.com): nice non-linear micro-wiki, similar
  concerns about long-term durability
