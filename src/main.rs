use clap::Parser;
use git2::Repository;
use parse_size::parse_size;
use std::collections::HashSet;
use humansize::{format_size, DECIMAL};

use std::process::exit;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(
        short,
        long,
        default_value_t = String::from("100 MiB"),
        help = "Set individual file size tolerance, default is 100 MiB"
    )]
    file_tolerance: String,

    #[arg(
        short,
        long,
        default_value_t = String::from("250 MiB"),
        help = "Set commit size tolerance, default is 250 MiB"
    )]
    staged_tolerance: String,

    #[arg(short, long, help = "Verbose output", default_value_t = false)]
    verbose: bool,
}

fn main() -> Result<(), git2::Error> {
    let args = Args::parse();

    let file_tolerance =
        parse_size(&args.file_tolerance).map_err(|_| git2::Error::from_str("Invalid size"))?;

    let staged_tolerance =
        parse_size(&args.staged_tolerance).map_err(|_| git2::Error::from_str("Invalid size"))?;
    let verbose = args.verbose;

    let staged_size = check_files(file_tolerance, verbose)?;
    let has_large_files = staged_size.large_files.len() > 0;
    if has_large_files {
        // we want to inform the user of the large files now, but not actually
        // exit since we want to do some more checks to give the user more information
        // on additional actions they might need to take to fully clean things up,
        // particularly around entire staged files size
        for file in &staged_size.large_files {
            eprintln!("{}: {}", file.path, format_size(file.size, DECIMAL));
        }
    }

    if staged_size.total_size < staged_tolerance {
        if has_large_files {
            exit(100);
        } 
        // this is the happy path, no large files and under the limit
        exit(0);
    }

    // at this point we know the staged files are over the limit
    eprintln!(
        "The staged files exceed the commit size tolerance of {}",
        &args.file_tolerance
    );
    if has_large_files {
        // both large files and over staged limit, we should check if once the large files were
        // removed, the commit size is still over the limit so we can inform the user of that as well
        let total_large_file_size = staged_size
            .large_files
            .iter()
            .fold(0, |acc, entry| acc + entry.size);
        let removed_total_size = staged_size.total_size - total_large_file_size;
        if removed_total_size > staged_tolerance {
            eprintln!("After removing large files, the commit size {} is still over the limit of {}",
                      format_size(removed_total_size, DECIMAL),
                      &args.staged_tolerance);
            exit(101)
        } else {
            eprintln!("After removing large files, the commit size {} will be within limit of {}",
                      format_size(removed_total_size, DECIMAL),
                      &args.staged_tolerance);
            exit(102)
        }
    }
    // no large files, just over the limit
    exit(103);
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct NewFile {
    path: String,
    size: u64,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct StagedFileStatus {
    large_files: Vec<NewFile>,
    total_size: u64,
}

/// check_files will check the files staged for commit and return a Vector of files and their size
/// that are over the file_tolerance size. 
/// It will also return the total size of all the files staged for commit that
/// can be used to check if the total size is over the staged_tolerance size.
fn check_files(file_tolerance: u64, verbose: bool) -> Result<StagedFileStatus, git2::Error> {
    let repo = Repository::open(".")?;

    let index = repo.index()?;

    let head_commit = repo.head()?.peel_to_commit()?;
    let head_tree = head_commit.tree()?;

    let diff = repo.diff_tree_to_index(Some(&head_tree), Some(&index), None)?;

    // we'll use this to collect the files from the diff. It's a little unclear if the diffs
    // are unique to the file or if they could be chunks of the same file where we end up
    // hitting the same file in the callback multiple times. We'll use a HashSet to ensure
    // we only get unique files
    let mut diff_files: HashSet<NewFile> = HashSet::new();

    // diff can't be turned into an iterator, so we have to use foreach
    // and collect the results ourselves
    let res = diff.foreach(
        &mut |delta, _| {
            let file = delta.new_file();
            if let Some(path) = delta.new_file().path() {
                let path_str = path.to_string_lossy().to_string();
                let size = file.size();
                if verbose {
                    println!("{}: {}", path_str, format_size(size, DECIMAL));
                }
                diff_files.insert(NewFile {
                    path: path_str,
                    size: file.size(),
                });
            }
            true
        },
        None,
        None,
        None,
    );

    // I'm not sure of a case where this could fail, so lets also dump the error out
    // so if someone reports this we can see what the error was
    if !res.is_ok() {
        dbg!(res.err());
        return Err(git2::Error::from_str("Error while iterating over diff"));
    }

    let large_files: Vec<NewFile> = diff_files
        .iter()
        .filter(|file| file.size > file_tolerance)
        .cloned()
        .collect();
    let total_size = diff_files.iter().fold(0, |acc, entry| acc + entry.size);

    Ok(StagedFileStatus {
        large_files,
        total_size,
    })
}
