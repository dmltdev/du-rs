use du_rs::{top_largest, walk_dir, DirStats};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_walk_counts_files_and_sizes() {
    let dir = tempdir().unwrap();

    fs::write(dir.path().join("small.txt"), b"hello").unwrap(); //  5 bytes
    fs::write(dir.path().join("medium.txt"), vec![0u8; 1024]).unwrap(); // 1024 bytes
    fs::write(dir.path().join("large.txt"), vec![0u8; 4096]).unwrap(); // 4096 bytes

    let mut stats = DirStats::new();
    walk_dir(dir.path(), &mut stats).unwrap();

    assert_eq!(stats.file_count, 3);
    assert_eq!(stats.total_size, 5 + 1024 + 4096);
}

#[test]
fn test_top_largest_returns_correct_order() {
    let dir = tempdir().unwrap();

    fs::write(dir.path().join("small.txt"), b"hello").unwrap();
    fs::write(dir.path().join("large.txt"), vec![0u8; 4096]).unwrap();

    let mut stats = DirStats::new();
    walk_dir(dir.path(), &mut stats).unwrap();
    top_largest(&mut stats, 1);

    assert_eq!(stats.largest_files.len(), 1);
    assert_eq!(stats.largest_files[0].0, 4096); // largest file is first
}
