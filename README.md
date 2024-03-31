# ms

**M**ini**S**ort

*Simple sort command, based on linux' sort*

Sort file content:
- alphanumerical [default]
- alphabetical
- numerical
- reverse

( --todo--
  - random
  - by month
  - remove duplicates
--todo--)

## Examples

todo!()

## Usage

### Short Usage

```
Usage: ms [OPTIONS] [PATH] [COMMAND]

Commands:
  log, -L, --log  Show content of the log file
  help            Print this message or the help of the given subcommand(s)

Arguments:
  [PATH]  The filepath to work with (reads filepath from stdin if left empty)

Options:
  -n, --numerical  Sort file content numerical
  -s, --string     Sort file content. Interpret everything as a literal string
  -r, --reverse    Reverse sort file content (can be combined with other flags)
  -h, --help       Print help (see more with '--help')
  -V, --version    Print version
```

### Long Usage

```
Usage: ms [OPTIONS] [PATH] [COMMAND]

Commands:
  log, -L, --log  Show content of the log file
  help            Print this message or the help of the given subcommand(s)

Arguments:
  [PATH]
          The filepath to work with (reads filepath from stdin if left empty)

Options:
  -n, --numerical
          Sort file content numerical
          WARNING: only sorts integers, no floating point numbers
          WARNING: only sorts integers up to 9223372036854775806

  -s, --string
          Sort file content. Interpret everything as a literal string

  -r, --reverse
          Reverse sort file content (can be combined with other flags)

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Installation

### Windows

via Cargo or get the ![binary](https://github.com/Phydon/ms/releases)
