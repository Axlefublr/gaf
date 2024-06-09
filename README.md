# gaf

> that's how dogs bark in russian

This terminal program will let you stage or unstage different types of git changes.

For example, you can `gaf stage new` to stage only the newly created files. Or `gaf unstaged modified` to unstage staged modifications.

Or, you could use `gaf unstage renamed` to unstage all renames.

The way that specific command works is that it unstages both the deletion of the old file and the addition of the new one.

Also, you get the more broad subcommand `print` to just print all the filepaths of a certain type of change.
Execute `gaf print deleted` to print all the filepaths that are unstaged deletions.

Overall, `gaf` works by parsing the output of `git status -s`.

## Usage

```
A way to stage/unstage a specific type of git change.

Usage: gaf <COMMAND>

Commands:
  stage    [aliases: s]
  unstage  [aliases: u]
  print    Print all the file paths of a type of a change, separated by newlines [aliases: p]
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### gaf stage
```
Usage: gaf stage <COMMAND>

Commands:
  new       [aliases: n]
  modified  [aliases: m]
  deleted   [aliases: d]
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

### gaf unstage
```
Usage: gaf unstage <COMMAND>

Commands:
  added     [aliases: a]
  modified  [aliases: m]
  renamed   [aliases: r]
  deleted   [aliases: d]
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

### gaf print
```
Print all the file paths of a type of a change, separated by newlines

Usage: gaf print <COMMAND>

Commands:
  new       [aliases: n]
  modified  [aliases: m]
  deleted   [aliases: d]
  added     [aliases: a]
  staged    Staged modified [aliases: s]
  renamed   [aliases: r]
  trashed   Staged deleted [aliases: t]
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

## Install

```
cargo install gaf
```

`cargo-binstall` and `cargo-quickinstall` are also supported.

## Uninstall

```
cargo uninstall gaf
```