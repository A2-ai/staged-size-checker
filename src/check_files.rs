use git2::Repository;
use std::{ffi::CString, path::Path};

#[derive(Debug)]
struct LargeFile {
    path: String,
    size: u32,
}

pub fn check_files(
    file_tolerance: u32,
    staged_tolerance: u64,
    directory: &Path,
) -> Result<(), git2::Error> {
    let repo = Repository::open(directory)?;

    // Get the index (staging area)
    let index = repo.index()?;

    if index.is_empty() {
        println!("Index is empty");
    }

    index.iter().for_each(|entry| {
        let file_path = unsafe {
            CString::from_vec_unchecked(entry.path)
                .to_str()
                .unwrap()
                .to_string()
        };
        println!(
            "file {} in index has size {}",
            file_path,
            (entry.file_size as f64 / 10000.0).round() / 100.0
        )
    });

    // Iterate over index, collect large files
    let large_files: Vec<LargeFile> = index
        .iter()
        .filter(|entry| entry.file_size > file_tolerance)
        .map(|entry| {
            let file_path = unsafe {
                CString::from_vec_unchecked(entry.path)
                    .to_str()
                    .unwrap()
                    .to_string()
            };
            LargeFile {
                path: file_path,
                size: entry.file_size,
            }
        })
        .collect();

    let has_large_files = !large_files.is_empty();

    if !large_files.is_empty() {
        large_files.into_iter().for_each(|file| {
            let file_size = (file.size as f64 / 10000.0).round() / 100.0;
            let threshold = (file_tolerance as f64 / 10000.0).round() / 100.0;
            println!(
                "File {} has size {} MB, which is greater than the tolerance {} MB.",
                file.path, file_size, threshold
            )
        });
    }

    let staged_size = index
        .iter()
        .fold(0, |acc, entry| acc + (entry.file_size as u64));

    let mut staged_size_large = false;
    if staged_size > staged_tolerance {
        println!(
            "Total staged size is {} MB, which is greater than the tolerance {} MB.",
            bytes_to_mb(staged_size),
            bytes_to_mb(staged_tolerance)
        );

        staged_size_large = true;
    }

    if has_large_files && staged_size_large {
        return Err(git2::Error::from_str(
            "File size exceeds tolerance\nTotal staged size exceeds tolerance",
        ));
    } else if has_large_files {
        return Err(git2::Error::from_str("File size exceeds tolerance"));
    } else if staged_size_large {
        return Err(git2::Error::from_str("Total staged size exceeds tolerance"));
    }

    Ok(())
}

pub fn bytes_to_mb(bytes: u64) -> f64 {
    (bytes as f64 / 10000.0).round() / 100.0
}

// fn run(oid: Oid, size: f64) -> Result<(), git2::Error> {
//     let repo = Repository::open(".")?;
//     let commit = repo.find_commit(oid)?;

//     // Find the commit specified by the hash
//     let tree = commit.tree()?;

//     // Get the index (staging area)
//     let index = repo.index()?;

//     // Compare the index against the specified commit's tree
//     let diff = repo.diff_tree_to_index(Some(&tree), Some(&index), None)?;

//     let mut flag = true;

//     // Iterate over differences
//     diff.foreach(
//         &mut |delta, _progress| {
//             let file_path = match delta.new_file().path() {
//                 Some(path) => path,
//                 None => Path::new(""),
//             };

//             let new_size: f64 = (delta.new_file().size() as f64 / 10000.0).round() / 100.0;
//             let old_size: f64 = (delta.old_file().size() as f64 / 10000.0).round() / 100.0;

//             let net_change = new_size - old_size;

//             if net_change > size {
//                 println!("File size exceeds tolerance of {} MB", size);
//                 println!("File: {:?}", file_path);
//                 println!("Old Size: {} MB", old_size);
//                 println!("New Size: {} MB", new_size);
//                 println!("Net Change: {} MB", net_change);
//                 flag = false;
//             }

//             // You can implement additional logic to compare sizes or other attributes here
//             true
//         },
//         None,
//         None,
//         None,
//     )?;

//     if !flag {
//         return Err(git2::Error::from_str("File size exceeds tolerance"));
//     }

//     Ok(())
// }
