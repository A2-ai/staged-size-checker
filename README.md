# Staged-size-checker

Staged-size-checker is a tool designed to prevent commits with files that exceed size limitations. It can be set up as a pre-commit hook in a git repository to enforce size constraints on both individual files and the total size of all staged files.

## Background and Scope

Github has a couple limitations around pushing files into remote repositories. 
If a file is larger than 100MiB, it will be rejected. 

If the total size of the **push** is greater than 2GB, it will be rejected. 
This tool is designed to help prevent these issues by checking the size of files 
in the staging area before a commit is made.

The ability to guesstimate the size of a push is a bit trickier. You must understand
what blobs do not exist on the remote repository, and also take into account the fact
that git will perform compression. This tool will not be able to accurately predict
the size of a push, so simplifies the situation to address the fact that in most
cases a data scientist should potentially seek alternative storage strategies if they are checking
in more than a couple hundred megabytes of content in a given commit. This is well
below the threshold, but can help be a preliminary check that the person committing
would have to take explicit action (if used as a precommit hook) to acknowledge the
size of the content they are committing.

We have found that there are many situations where the person is not even aware of the
total size/contents they are committing, for example, when a new folder is created `git status`
just shows the folder and not all the files inside. Situations where people have thought
they git ignored large files or batches of files and did not were not clearly shown
in the status output.

## Installation

To install the latest version, see the [Releases](https://github.com/a2-ai/staged-size-checker/releases/) page.

### Triggering automatically

If you want to automatically check file sizes before each commit using [Lefthook](https://github.com/evilmartians/lefthook), create a `lefthook.yaml` with the following contents:

```yaml
pre-commit:
  commands:
    size-check:
      run: staged-size-checker
```

## Usage

```shell
Usage: staged-size-checker [OPTIONS]

Options:
  -f, --file-tolerance <FILE_TOLERANCE>
          Set individual file size tolerance, default is 100 MiB [default: "100 MiB"]
  -s, --staged-tolerance <STAGED_TOLERANCE>
          Set commit size tolerance, default is 250 MiB [default: "250 MiB"]
  -v, --verbose
          Verbose output
  -h, --help
          Print help
  -V, --version
          Print version
```

### Exit status

Different exit codes can allow other programs to respond differently without parsing error messages:

- `0`: Success. All file sizes are within the specified tolerances.
- `100`: Failure. One or more files exceed the specified size limits.
- `101`: Failure. Total staged size exceeds the specified limits and has large files. When large files are removed, the total staged size will still be over
the specified limit 
- `102`: Total staged size exceeds the specified limits and has large files. When large files are removed, the total staged size will be within the specified limits.
- `103`: Failure. The aggregate size of the commit exceeds the total threshold.

## See also

git(1), git-hooks(5)
