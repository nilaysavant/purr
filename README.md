# purr (`cat` in Rust)

[![build](https://github.com/nilaysavant/purr/actions/workflows/build.yml/badge.svg)](https://github.com/nilaysavant/purr/actions/workflows/build.yml)

My version of linux `cat` command, written in Rust.

```sh
Purr(cat) FILE(s) to standard output

Usage: purr [OPTIONS] [FILES]...

Arguments:
  [FILES]...  File(s) to concatenate to standard output

Options:
  -n, --number   Number all output lines, starting with 1
  -h, --help     Print help information
  -V, --version  Print version information
```

## Supported Features

- Show concatenated `file(s)` + `standard input` to `standard output`.
- Show **line numbers** in output. (using `--number` or `-n`).
- Read **standard input** in a loop until `EOF`.
- Read **binary/non-utf8/non-string** files.

## Installation

### Build from source

- Clone this repository and open this dir in the terminal:

  ```sh
  cd purr/
  ```

- Compile and build:

  ```sh
  cargo build --release
  ```

- Run the binary located at `./target/release/purr`:

  ```sh
  ./target/release/purr --help
  ```

## Automated Testing

- Run tests using the following command:

  ```sh
  cargo test
  ```

- The following `unit` and `integration` tests are currently implemented:

  ```sh
  # integration tests
  test not_permitted_file_read ... ok
  test standard_input_only ... ok
  test single_file_read ... ok
  test single_binary_file_read ... ok
  test single_file_single_stdin ... ok
  test mixed_multi_file_read ... ok
  test multi_file_read ... ok
  test multi_file_read_show_lines ... ok
  # unit tests
  test validate_file_path_when_file_dne ... ok
  test read_file_basic ... ok
  test validate_file_path_when_file_exists ... ok
  ```

## Roadmap

- [x] Show help.
  - [x] Add arg parser.
- [x] Show concatenated file(s)/input to standard output.
  - [x] Show line numbers in output. (using `--number` or `-n`)
  - [x] Concatenate all files/inputs.
    - [x] Parse file paths as strings.
    - [x] Validate paths from strings to actual paths. Check path is `-` or if _no file_ is provided.
    - [x] **Read standard input**
    - [x] **Read file(s)**.
      - [x] Read binary/non-utf8 files.

## References

- `cat` [manual](https://www.gnu.org/software/coreutils/manual/html_node/cat-invocation.html#cat-invocation):

  ```bash
  Usage: cat [OPTION]... [FILE]...
    Concatenate FILE(s) to standard output.

    With no FILE, or when FILE is -, read standard input.

    -A, --show-all           equivalent to -vET
    -b, --number-nonblank    number nonempty output lines, overrides -n
    -e                       equivalent to -vE
    -E, --show-ends          display $ at end of each line
    -n, --number             number all output lines
    -s, --squeeze-blank      suppress repeated empty output lines
    -t                       equivalent to -vT
    -T, --show-tabs          display TAB characters as ^I
    -u                       (ignored)
    -v, --show-nonprinting   use ^ and M- notation, except for LFD and TAB
    --help     display this help and exit
    --version  output version information and exit

    Examples:
    cat f - g  Output f's contents, then standard input, then g's contents.
    cat        Copy standard input to standard output.

    GNU coreutils online help: <https://www.gnu.org/software/coreutils/>
    Full documentation at: <https://www.gnu.org/software/coreutils/cat>
    or available locally via: info '(coreutils) cat invocation'
  ```

- [Testing Rust CLI](https://rust-cli.github.io/book/tutorial/testing.html).
