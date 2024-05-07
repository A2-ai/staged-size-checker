use clap::Parser;
use git2::{Oid, Repository};
use std::path::Path;
use std::process::exit;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(
        short,
        long,
        default_value_t = 5,
        help = "Set size tolerance in MB, default is 5 MB"
    )]
    size: u8,

    #[arg(help = "Optional commit hash to compare against")]
    commit_hash: Option<String>,
}

fn main() -> Result<(), git2::Error> {
    let args = Args::parse();

    let repo = Repository::open(".")?;

    let commit_hash = if args.commit_hash.is_some() {
        Oid::from_str(&args.commit_hash.unwrap())?
    } else {
        let obj = repo.head()?.resolve()?.peel_to_commit()?;
        obj.id()
    };
    // Retrieve the commit hash from command-line arguments

    if run(commit_hash, args.size).is_err() {
        // exit code zero
        exit(0);
    } else {
        exit(1);
    }
}

fn run(oid: Oid, size: u8) -> Result<(), git2::Error> {
    let repo = Repository::open(".")?;
    let commit = repo.find_commit(oid)?;

    // Find the commit specified by the hash
    let tree = commit.tree()?;

    // Get the index (staging area)
    let index = repo.index()?;

    // Compare the index against the specified commit's tree
    let diff = repo.diff_tree_to_index(Some(&tree), Some(&index), None)?;

    // Iterate over differences
    diff.foreach(
        &mut |delta, _progress| {
            let file_path = match delta.new_file().path() {
                Some(path) => path,
                None => Path::new(""),
            };

            let new_size: f64 = (delta.new_file().size() as f64 / 10000.0).round() / 100.0;
            let old_size: f64 = (delta.old_file().size() as f64 / 10000.0).round() / 100.0;

            let net_change = new_size - old_size;

            if net_change > size as f64 {
                println!("File size exceeds tolerance of {} MB", size);
                println!("File: {:?}", file_path);
                println!("Old Size: {} MB", old_size);
                println!("New Size: {} MB", new_size);
                println!("Net Change: {} MB", net_change);
            }

            // You can implement additional logic to compare sizes or other attributes here
            true
        },
        None,
        None,
        None,
    )?;

    Ok(())
}
