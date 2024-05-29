use clap::Parser;
use parse_size::parse_size;
use std::path::Path;
//use std::process::exit;

pub mod check_files;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(
        short,
        long,
        default_value_t = String::from("100 MB"),
        help = "Set individual file size tolerance, default is 100 MB"
    )]
    file_tolerance: String,

    #[arg(
        short,
        long,
        default_value_t = String::from("500 MB"),
        help = "Set commit size tolerance, default is 500 MB"
    )]
    staged_tolerance: String,

    // optional argument to specificy the directory passed to check_files()
    #[arg(
        default_value_t = String::from("."),
        help = "Directory to compare against. Default is the current directory"
    )]
    directory: String,

    #[arg(short, long, help = "Optional commit hash to compare against")]
    commit_hash: Option<String>,
}

fn main() -> Result<(), git2::Error> {
    let args = Args::parse();

    let directory = Path::new(&args.directory);

    let file_tolerance =
        parse_size(&args.file_tolerance).map_err(|_| git2::Error::from_str("Invalid size"))?;

    let staged_tolerance =
        parse_size(&args.staged_tolerance).map_err(|_| git2::Error::from_str("Invalid size"))?;

    let file_tolerance = u32::try_from(file_tolerance)
        .map_err(|_| git2::Error::from_str("Could not convert to u32"))?;

    // let repo = Repository::open(".")?;

    // let commit_hash = if args.commit_hash.is_some() {
    //     Oid::from_str(&args.commit_hash.unwrap())?
    // } else {
    //     let obj = repo.head()?.resolve()?.peel_to_commit()?;
    //     obj.id()
    // };

    if check_files::check_files(file_tolerance, staged_tolerance, directory).is_err() {
        // exit code zero
        return Err(git2::Error::from_str("Files exceed tolerance"));
    }

    Ok(())
}
