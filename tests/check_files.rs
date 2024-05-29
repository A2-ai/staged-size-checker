use git2::Repository;
use staged_size_checker::check_files::check_files;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;
use tempfile::tempdir;
use walkdir::WalkDir;

#[test]
fn test_check_files_with_no_files_git_add() {
    let file_tolerance = 100;
    let staged_tolerance = 500;

    let dir = tempdir().unwrap();
    let repo_path = dir.path();
    Command::new("git")
        .arg("init")
        .current_dir(repo_path)
        .output()
        .unwrap();

    println!("The repo_path is {:?}", repo_path);

    for entry in WalkDir::new(repo_path).into_iter().filter_map(|e| e.ok()) {
        println!("{}", entry.path().display());
    }

    Command::new("git")
        .arg("add")
        .arg(repo_path)
        .output()
        .unwrap();

    let repo = Repository::open(repo_path).unwrap();
    let index = repo.index().unwrap();

    assert!(index.is_empty());

    let res = check_files(file_tolerance, staged_tolerance, repo_path);

    assert!(res.is_ok());
}

#[test]
fn test_check_files_with_one_small_file_no_add() {
    let file_tolerance = 100_000_000;
    let staged_tolerance = 500_000_000;

    let dir = tempdir().unwrap();
    let repo_path = dir.path();
    Command::new("git")
        .arg("init")
        .current_dir(repo_path)
        .output()
        .unwrap();

    let file_path = repo_path.join("50_mb_file.txt");
    fs::write(&file_path, vec![0; 50_000_000]).unwrap();

    println!("The repo_path is {:?}", repo_path);

    for entry in WalkDir::new(repo_path).into_iter().filter_map(|e| e.ok()) {
        println!("{}", entry.path().display());
    }

    let repo = Repository::open(repo_path).unwrap();
    let index = repo.index().unwrap();

    assert!(index.is_empty());

    assert!(check_files(file_tolerance, staged_tolerance, repo_path).is_ok());
    assert!(true);
}

#[test]
fn test_check_files_with_one_small_file_add() {
    let file_tolerance = 100_000_000;
    let staged_tolerance = 500_000_000;

    let dir = tempdir().unwrap();
    let repo_path = dir.path();
    Command::new("git")
        .arg("init")
        .current_dir(repo_path)
        .output()
        .unwrap();

    let file_path = repo_path.join("50_mb_file.txt");
    fs::write(&file_path, vec![0; 50_000_000]).unwrap();

    println!("The repo_path is {:?}", repo_path);

    for entry in WalkDir::new(repo_path).into_iter().filter_map(|e| e.ok()) {
        println!("{}", entry.path().display());
    }

    Command::new("git")
        .arg("add")
        .arg(".")
        .current_dir(repo_path)
        .output()
        .unwrap();

    let repo = Repository::open(repo_path).unwrap();
    let index = repo.index().unwrap();

    assert!(index.len() == 1);

    assert!(check_files(file_tolerance, staged_tolerance, repo_path).is_ok());
}

#[test]
fn test_check_files_with_500_mb_file_no_add() {
    let file_tolerance = 100_000_000;
    let staged_tolerance = 500_000_000;

    let dir = tempdir().unwrap();
    let repo_path = dir.path();
    Command::new("git")
        .arg("init")
        .current_dir(repo_path)
        .output()
        .unwrap();

    let file_path = repo_path.join("500_mb_file.txt");
    fs::write(&file_path, vec![0; 500_000_000]).unwrap();

    println!("The repo_path is {:?}", repo_path);

    for entry in WalkDir::new(repo_path).into_iter().filter_map(|e| e.ok()) {
        println!("{}", entry.path().display());
    }

    let repo = Repository::open(repo_path).unwrap();
    let index = repo.index().unwrap();

    assert!(index.is_empty());
    assert!(check_files(file_tolerance, staged_tolerance, repo_path).is_ok());
}

#[test]
fn test_check_files_with_500_mb_file_add() {
    let file_tolerance = 100_000_000;
    let staged_tolerance = 500_000_000;

    let dir = tempdir().unwrap();
    let repo_path = dir.path();
    Command::new("git")
        .arg("init")
        .current_dir(repo_path)
        .output()
        .unwrap();

    let file_path = repo_path.join("500_mb_file.txt");
    fs::write(&file_path, vec![0; 500_000_000]).unwrap();

    println!("The repo_path is {:?}", repo_path);

    for entry in WalkDir::new(repo_path).into_iter().filter_map(|e| e.ok()) {
        println!("{}", entry.path().display());
    }

    Command::new("git")
        .arg("add")
        .arg(".")
        .current_dir(repo_path)
        .output()
        .unwrap();

    let repo = Repository::open(repo_path).unwrap();
    let index = repo.index().unwrap();

    assert!(index.len() == 1);

    let res = check_files(file_tolerance, staged_tolerance, repo_path);
    assert!(res.is_err());
}

#[test]
fn test_check_files_with_100_mb_file_add() {
    let file_tolerance = 100_000_000;
    let staged_tolerance = 500_000_000;

    let dir = tempdir().unwrap();
    let repo_path = dir.path();
    Command::new("git")
        .arg("init")
        .current_dir(repo_path)
        .output()
        .unwrap();

    let file_path = repo_path.join("100_mb_file.txt");
    fs::write(&file_path, vec![0; 100_000_000]).unwrap();

    println!("The repo_path is {:?}", repo_path);

    for entry in WalkDir::new(repo_path).into_iter().filter_map(|e| e.ok()) {
        println!("{}", entry.path().display());
    }

    Command::new("git")
        .arg("add")
        .arg(".")
        .current_dir(repo_path)
        .output()
        .unwrap();

    let repo = Repository::open(repo_path).unwrap();
    let index = repo.index().unwrap();

    println!("index.len() = {}", index.len());

    let res = check_files(file_tolerance, staged_tolerance, repo_path);
    assert!(res.is_err());
}

#[test]
fn test_check_files_with_500_mb_file_add_fail_delete_add_check() {
    let file_tolerance = 100_000_000;
    let staged_tolerance = 500_000_000;

    let dir = tempdir().unwrap();
    let repo_path = dir.path();
    Command::new("git")
        .arg("init")
        .current_dir(repo_path)
        .output()
        .unwrap();

    let file_path = repo_path.join("500_mb_file.txt");
    fs::write(&file_path, vec![0; 500_000_000]).unwrap();

    println!("The repo_path is {:?}", repo_path);

    for entry in WalkDir::new(repo_path).into_iter().filter_map(|e| e.ok()) {
        println!("{}", entry.path().display());
    }

    Command::new("git")
        .arg("add")
        .arg(".")
        .current_dir(repo_path)
        .output()
        .unwrap();

    let repo = Repository::open(repo_path).unwrap();
    let index = repo.index().unwrap();

    assert!(index.len() == 1);

    let res = check_files(file_tolerance, staged_tolerance, repo_path);
    assert!(res.is_err());

    //delete the file
    fs::remove_file(&file_path).unwrap();

    Command::new("git")
        .arg("add")
        .arg(".")
        .current_dir(repo_path)
        .output()
        .unwrap();

    for entry in WalkDir::new(repo_path).into_iter().filter_map(|e| e.ok()) {
        println!("{}", entry.path().display());
    }

    let repo = Repository::open(repo_path).unwrap();
    let index = repo.index().unwrap();

    println!("after removing the file,index.len() = {}", index.len());

    let res = check_files(file_tolerance, staged_tolerance, repo_path);
    assert!(res.is_ok());
}

#[test]
fn test_check_files_with_100_mb_file_add_150_mb_file_add() {
    let file_tolerance = 100_000_000;
    let staged_tolerance = 500_000_000;

    let dir = tempdir().unwrap();
    let repo_path = dir.path();
    Command::new("git")
        .arg("init")
        .current_dir(repo_path)
        .output()
        .unwrap();

    let file_path40 = repo_path.join("40_mb_file.txt");
    fs::write(&file_path40, vec![0; 40_000_000]).unwrap();

    println!("The repo_path is {:?}", repo_path);

    for entry in WalkDir::new(repo_path).into_iter().filter_map(|e| e.ok()) {
        println!("{}", entry.path().display());
    }

    Command::new("git")
        .arg("add")
        .arg(".")
        .current_dir(repo_path)
        .output()
        .unwrap();

    let repo = Repository::open(repo_path).unwrap();
    let index = repo.index().unwrap();

    println!("index.len() = {}", index.len());

    let res = check_files(file_tolerance, staged_tolerance, repo_path);
    assert!(res.is_ok());

    let file_path50 = repo_path.join("50_mb_file.txt");
    fs::write(&file_path50, vec![0; 50_000_000]).unwrap();

    println!("The repo_path is {:?}", repo_path);

    for entry in WalkDir::new(repo_path).into_iter().filter_map(|e| e.ok()) {
        println!("{}", entry.path().display());
    }

    Command::new("git")
        .arg("add")
        .arg(".")
        .current_dir(repo_path)
        .output()
        .unwrap();

    let repo = Repository::open(repo_path).unwrap();
    let index = repo.index().unwrap();

    println!("index.len() = {}", index.len());

    let res = check_files(file_tolerance, staged_tolerance, repo_path);
    assert!(res.is_ok());
}

#[test]
fn test_check_files_with_40_mb_file_add_commit_50_mb_file_add() {
    let file_tolerance = 100_000_000;
    let staged_tolerance = 500_000_000;

    let dir = tempdir().unwrap();
    let repo_path = dir.path();
    Command::new("git")
        .arg("init")
        .current_dir(repo_path)
        .output()
        .unwrap();

    let file_path40 = repo_path.join("40_mb_file.txt");
    fs::write(&file_path40, vec![0; 40_000_000]).unwrap();

    println!("The repo_path is {:?}", repo_path);

    for entry in WalkDir::new(repo_path).into_iter().filter_map(|e| e.ok()) {
        println!("{}", entry.path().display());
    }

    let output = Command::new("git")
        .arg("add")
        .arg(".")
        .current_dir(repo_path)
        .output()
        .unwrap();

    if output.status.success() {
        println!("Git add successful.");
    } else {
        println!("Git add failed.");
    }

    let repo = Repository::open(repo_path).unwrap();
    let index = repo.index().unwrap();

    println!("index.len() = {}", index.len());

    let res = check_files(file_tolerance, staged_tolerance, repo_path);
    assert!(res.is_ok());

    let output = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("commit 40mb file")
        .current_dir(repo_path) // Set the working directory
        .output()
        .unwrap();

    if output.status.success() {
        println!("Git commit successful.");
    } else {
        println!("Git commit failed.");
    }

    let file_path50 = repo_path.join("50_mb_file.txt");
    fs::write(&file_path50, vec![0; 50_000_000]).unwrap();

    println!("The repo_path is {:?}", repo_path);

    for entry in WalkDir::new(repo_path).into_iter().filter_map(|e| e.ok()) {
        println!("{}", entry.path().display());
    }

    let output = Command::new("git")
        .arg("add")
        .arg(".")
        .current_dir(repo_path)
        .output()
        .unwrap();

    if output.status.success() {
        println!("Git add successful.");
    } else {
        println!("Git add failed.");
    }

    let repo = Repository::open(repo_path).unwrap();
    let index = repo.index().unwrap();

    println!("index.len() = {}", index.len());

    let res = check_files(file_tolerance, staged_tolerance, repo_path);
    assert!(res.is_ok());
}

#[test]
fn test_check_files_with_40_mb_file_add_commit_increase_40_mb_file_add() {
    let file_tolerance = 100_000_000;
    let staged_tolerance = 500_000_000;

    let dir = tempdir().unwrap();
    let repo_path = dir.path();
    Command::new("git")
        .arg("init")
        .current_dir(repo_path)
        .output()
        .unwrap();

    let file_path40 = repo_path.join("40_mb_file.txt");
    fs::write(&file_path40, vec![0; 40_000_000]).unwrap();

    println!("The repo_path is {:?}", repo_path);

    for entry in WalkDir::new(repo_path).into_iter().filter_map(|e| e.ok()) {
        println!("{}", entry.path().display());
    }

    let output = Command::new("git")
        .arg("add")
        .arg(".")
        .current_dir(repo_path)
        .output()
        .unwrap();

    if output.status.success() {
        println!("Git add successful.");
    } else {
        println!("Git add failed.");
    }

    let repo = Repository::open(repo_path).unwrap();
    let index = repo.index().unwrap();

    println!("index.len() = {}", index.len());

    let res = check_files(file_tolerance, staged_tolerance, repo_path);
    assert!(res.is_ok());

    let output = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("commit 40mb file")
        .current_dir(repo_path) // Set the working directory
        .output()
        .unwrap();

    if output.status.success() {
        println!("Git commit successful.");
    } else {
        println!("Git commit failed.");
    }

    let mut file = OpenOptions::new()
        .append(true)
        .open(&file_path40)
        .expect("Failed to open file in append mode");

    let additional_data = vec![0; 10_000_000];

    file.write_all(&additional_data).unwrap();

    println!("the file hase {} bytes", file.metadata().unwrap().len());

    //convert bytes to size in MB
    println!(
        "the file has size {} MB",
        file.metadata().unwrap().len() / 1000000
    );

    println!("The repo_path is {:?}", repo_path);

    for entry in WalkDir::new(repo_path).into_iter().filter_map(|e| e.ok()) {
        println!("{}", entry.path().display());
    }

    let output = Command::new("git")
        .arg("add")
        .arg(".")
        .current_dir(repo_path)
        .output()
        .unwrap();

    if output.status.success() {
        println!("Git add successful.");
    } else {
        println!("Git add failed.");
    }

    let repo = Repository::open(repo_path).unwrap();
    let index = repo.index().unwrap();

    println!("index.len() = {}", index.len());

    let res = check_files(file_tolerance, staged_tolerance, repo_path);
    assert!(res.is_ok());
}

#[test]
fn test_check_files_with_40_mb_file_add_commit_increase_40_mb_file_to_150_mb_add() {
    let file_tolerance = 100_000_000;
    let staged_tolerance = 500_000_000;

    let dir = tempdir().unwrap();
    let repo_path = dir.path();
    Command::new("git")
        .arg("init")
        .current_dir(repo_path)
        .output()
        .unwrap();

    let file_path40 = repo_path.join("40_mb_file.txt");
    fs::write(&file_path40, vec![0; 40_000_000]).unwrap();

    println!("The repo_path is {:?}", repo_path);

    for entry in WalkDir::new(repo_path).into_iter().filter_map(|e| e.ok()) {
        println!("{}", entry.path().display());
    }

    let output = Command::new("git")
        .arg("add")
        .arg(".")
        .current_dir(repo_path)
        .output()
        .unwrap();

    if output.status.success() {
        println!("Git add successful.");
    } else {
        println!("Git add failed.");
    }

    let repo = Repository::open(repo_path).unwrap();
    let index = repo.index().unwrap();

    println!("index.len() = {}", index.len());

    let res = check_files(file_tolerance, staged_tolerance, repo_path);
    assert!(res.is_ok());

    let output = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("commit 40mb file")
        .current_dir(repo_path) // Set the working directory
        .output()
        .unwrap();

    if output.status.success() {
        println!("Git commit successful.");
    } else {
        println!("Git commit failed.");
    }

    let mut file = OpenOptions::new()
        .append(true)
        .open(&file_path40)
        .expect("Failed to open file in append mode");

    let additional_data = vec![0; 110_000_000];

    file.write_all(&additional_data).unwrap();

    println!("the file hase {} bytes", file.metadata().unwrap().len());

    //convert bytes to size in MB
    println!(
        "the file has size {} MB",
        file.metadata().unwrap().len() / 1000000
    );

    println!("The repo_path is {:?}", repo_path);

    for entry in WalkDir::new(repo_path).into_iter().filter_map(|e| e.ok()) {
        println!("{}", entry.path().display());
    }

    let output = Command::new("git")
        .arg("add")
        .arg(".")
        .current_dir(repo_path)
        .output()
        .unwrap();

    if output.status.success() {
        println!("Git add successful.");
    } else {
        println!("Git add failed.");
    }

    let repo = Repository::open(repo_path).unwrap();
    let index = repo.index().unwrap();

    println!("index.len() = {}", index.len());

    let res = check_files(file_tolerance, staged_tolerance, repo_path);
    assert!(res.is_err());
}
