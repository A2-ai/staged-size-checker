use clap::Parser;
use git2::{Oid, Repository};
use parse_size::parse_size;
use std::path::Path;
use std::process::exit;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(
        short,
        long,
        default_value_t = String::from("5 MB"),
        help = "Set size tolerance, default is 5 MB"
    )]
    threshold: String,

    #[arg(help = "Optional commit hash to compare against")]
    commit_hash: Option<String>,
}

fn main() -> Result<(), git2::Error> {
    let args = Args::parse();

    let threshold =
        parse_size(&args.threshold).map_err(|_| git2::Error::from_str("Invalid size"))?;

    let threshold = threshold as f64 / 1000000.0;

    println!("In main, threshold: {}", threshold);

    let repo = Repository::open(".")?;

    let commit_hash = if args.commit_hash.is_some() {
        Oid::from_str(&args.commit_hash.unwrap())?
    } else {
        let obj = repo.head()?.resolve()?.peel_to_commit()?;
        obj.id()
    };
    // Retrieve the commit hash from command-line arguments

    if run(commit_hash, threshold).is_err() {
        // exit code zero
        exit(1);
    } else {
        exit(0);
    }
}

fn run(oid: Oid, size: f64) -> Result<(), git2::Error> {
    println!("In function, threshold: {}", size);
    let repo = Repository::open(".")?;
    let commit = repo.find_commit(oid)?;

    // Find the commit specified by the hash
    let tree = commit.tree()?;

    // Get the index (staging area)
    let index = repo.index()?;

    // Compare the index against the specified commit's tree
    let diff = repo.diff_tree_to_index(Some(&tree), Some(&index), None)?;

    let mut flag = true;

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

            if net_change > size {
                println!("File size exceeds tolerance of {} MB", size);
                println!("File: {:?}", file_path);
                println!("Old Size: {} MB", old_size);
                println!("New Size: {} MB", new_size);
                println!("Net Change: {} MB", net_change);
                flag = false;
            }

            // You can implement additional logic to compare sizes or other attributes here
            true
        },
        None,
        None,
        None,
    )?;

    if !flag {
        return Err(git2::Error::from_str("File size exceeds tolerance"));
    }

    Ok(())
}
