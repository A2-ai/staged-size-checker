# Staged-size-checker

Staged-size-checker is a tool designed to prevent commits with files that exceed size limitations. It can be set up as a pre-commit hook in a git repository to enforce size constraints on both individual files and the total size of all staged files.

## Installation

For now, to install you must clone the repository and run the below command:
```shell
cargo install --path .
```

This will install the binary to `$HOME/.cargo/bin`.

### Triggering automatically
If you want to automatically check file sizes before each commit using [Lefthook](https://github.com/evilmartians/lefthook), create a `lefthook.yaml` with the following contents:

```yaml
pre-commit:
  commands:
    size-check:
      run: staged-size-checker
```


<!-- ```shell
cargo install staged-size-checker
``` -->

## Usage

```shell
❯ staged-size-checker --help
Usage: staged-size-checker [OPTIONS] [COMMIT_HASH]

Arguments:
  [COMMIT_HASH]  Optional commit hash to compare against

Options:
  -f, --file-tolerance <FILE_TOLERANCE>
          Set individual file size tolerance, default is 100 MB [default: "100 MB"]
  -s, --staged-tolerance <STAGED_TOLERANCE>
          Set commit size tolerance, default is 500 MB [default: "500 MB"]
  -h, --help
          Print help
  -V, --version
          Print version
```

### Exit status

`0`: Success. All file sizes are within the specified tolerances.

`1`: Failure. One or more files exceed the specified size limits.


## Contributing

To build,
```shell
just build
```

## See also

git(1), git-hooks(5)
