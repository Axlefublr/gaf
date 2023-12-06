# gaf

*that's how dogs bark in russian*

This terminal program will let you stage or unstage different types of git changes.

For example, you can `gaf stage new` to stage only the newly created files. Or `gaf unstaged modified` to unstage staged modifications.

Or, you could use `gaf unstage renamed` to unstage all renames.

The way that specific command works is that it unstages both the deletion of the old file and the addition of the new one.

Overall, `gaf` works by parsing the output of `git status -s`.

## Usage

```
Usage: gaf <COMMAND>

Commands:
  stage
          
  unstage
          
  help
          Print this message or the help of the given subcommand(s)

Options:
  -h, --help
          Print help
```

### gaf stage
```
Usage: gaf stage <WHICH>

Arguments:
  <WHICH>
          [possible values: new, modified, deleted]
```

### gaf unstage
```
Usage: gaf unstage <WHICH>

Arguments:
  <WHICH>
          [possible values: added, modified, renamed, deleted]
```

## Installation

```
cargo install gaf
```

`cargo-binstall` and `cargo-quickinstall` are also supported.