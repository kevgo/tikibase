<img src="doc/tiki_head.jpg" width="154" height="223" align="right">

# Tikibase

_the un-database_

Whatever note taking or knowledge base application you love and use today, it
will be dated in 5 years and unsupported/unavailable in 10 years. All your data
will be gone with it.

Tikibase is a knowledge base available on all current and future computers
because it isn't based on a dedicated server or viewer application that might be
unavailable on future devices. A Tikibase is pure human readable and machine
parsable data, a collection of Markdown files in a folder. You view, change, and
organize these files using the text or Markdown editor of your choice. If your
computer can display and edit text files, you can use your Tikibase on it. A
powerful option for efficiently working on large Tikibases in the 2020s is
[VSCode Markdown IDE](https://github.com/kevgo/vscode-markdown-ide).

You manage changes to your knowledge base using a version control system like
Git, Mercurial, or whatever they use in the future. The open-source CLI
application in this repository is an optional linter that helps keep a Tikibase
consistent by finding and fixing these issues:

- **broken links/images:** links pointing to non-existing local files
- **orphaned resources:** non-Markdown files not referenced by a Markdown
  document
- **missing backlinks:** if document A links to document B, document B must also
  link to document A
- **links to the same document:** document A should not contain links to
  document A
- **inconsistent heading capitalization:** when you use `## FooBar` and
  `## Foobar` in different documents
- **duplicate headings in a file**
- **empty sections**
- **missing footnote definitions and references**

When the config file defines the allowed section names, Tikibase verifies these
additional properties:

- **unknown headings**
- **heading order**

![build status](https://github.com/kevgo/tikibase/actions/workflows/ci.yml/badge.svg)

### installation

Download the [binary](https://github.com/kevgo/tikibase/releases/latest) for
your platform or install from source:

- [install Rust](https://rustup.rs) stable
- `cargo install --git https://github.com/kevgo/tikibase.git`
- add `~/.cargo/bin` to your shell's `$PATH`

### usage

- while working on Markdown files, run `tikibase p` ("pitstop"). The pitstop
  command fixes all auto-fixable issues and lists the remaining ones.
- in your tests/CI: run `tikibase check` (lists all issues)
- to see all available commands: `tikibase help`

### configuration

Create a file `tikibase.json` in your Tikibase directory. Use the linked JSON
Schema for documentation and auto-completion of the various options. Here is an
example:

```json
{
  "$schema": "https://raw.githubusercontent.com/kevgo/tikibase/main/doc/tikibase.schema.json",
  "bidiLinks": true,
  "globs": ["!Makefile"],
  "sections": ["foo", "bar"]
}
```

- **bidiLinks:** enables the bi-directional links feature
- **globs:** by default Tikibase looks at all files that aren't `.gitignore`d.
  With this section you can fine-tune the files Tikibase looks at using glob
  expressions. To exclude files, precede the glob with a `!`.
- **sections:** if provided, accepts only sections with the given names, in the
  given order

### related

- [VSCode Markdown IDE](https://github.com/kevgo/vscode-markdown-ide) provides
  IDE-grade refactoring for Tikibases and runs this `tikibase` linter for you.
- [Obsidian](https://obsidian.md): a more fully featured Markdown-based
  knowledge base including its own editor/viewer and many plugins. Easier and to
  get started but - like all application-based solutions - will be outdated at
  some point and unavailable on future platforms.
- [TiddlyWiki](https://tiddlywiki.com): nice non-linear micro-wiki, similar
  concerns about long-term durability
