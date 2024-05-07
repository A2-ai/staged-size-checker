use clap::Parser;
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 5, help = "Set size tolerance in MB")]
    size: u64,
}

fn main() {
    let args = Args::parse();
    //  maximum file size allowed (5 MB)
    let max_size: u64 = args.size * 1024 * 1024; // default is 5 MB

    // Retrieve the list of staged files
    let output = Command::new("git")
        .args(["diff", "--cached", "--name-only", "--diff-filter=ACM"])
        .output()
        .expect("Failed to execute git command");

    if !output.status.success() {
        eprintln!("Failed to retrieve staged files");
        std::process::exit(1);
    }

    // Convert the output to a readable format
    let file_names = String::from_utf8_lossy(&output.stdout);

    for file_name in file_names.split_whitespace() {
        let path = Path::new(file_name);
        if path.exists() && path.is_file() {
            let size = fs::metadata(path)
                .expect("Failed to read file metadata")
                .len();

            if size > max_size {
                eprintln!(
                    "Error: File '{}' exceeds the maximum allowed size of {} bytes",
                    file_name, max_size
                );
                std::process::exit(1);
            }
        }
    }

    std::process::exit(0);
}
