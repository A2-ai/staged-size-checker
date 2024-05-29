use git2::Repository;
use staged_size_checker::check_files::check_files;
use std::fs;
use std::process::Command;
use tempfile::tempdir;
use walkdir::WalkDir;

#[test]
fn test_check_files_with_no_files() {
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

    let file_path = repo_path.join("50_file.txt");
    fs::write(&file_path, vec![0; 49_000_000]).unwrap();

    println!("The repo_path is {:?}", repo_path);

    for entry in WalkDir::new(repo_path).into_iter().filter_map(|e| e.ok()) {
        println!("{}", entry.path().display());
    }

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

    let file_path = repo_path.join("50_file.txt");
    fs::write(&file_path, vec![0; 49_000_000]).unwrap();

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

    assert!(check_files(file_tolerance, staged_tolerance, repo_path).is_ok());
}

#[test]
fn test_check_files_with_one_large_file_no_add() {
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

    let file_path_fifty = repo_path.join("500_mb_file.txt");
    fs::write(&file_path_fifty, vec![0; 500_000_000]).unwrap();

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
