use du_rs::{top_largest, walk_dir, DirStats};
use std::fs;
use std::os::unix::fs::{symlink, PermissionsExt};
use tempfile::tempdir;

#[test]
fn test_walk_counts_files_and_sizes() {
    let dir = tempdir().unwrap();

    fs::write(dir.path().join("small.txt"), b"hello").unwrap(); //  5 bytes
    fs::write(dir.path().join("medium.txt"), vec![0u8; 1024]).unwrap(); // 1024 bytes
    fs::write(dir.path().join("large.txt"), vec![0u8; 4096]).unwrap(); // 4096 bytes

    let mut stats = DirStats::new();
    walk_dir(dir.path(), &mut stats);

    assert_eq!(stats.file_count, 3);
    assert_eq!(stats.total_size, 5 + 1024 + 4096);
}

#[test]
fn test_top_largest_returns_correct_order() {
    let dir = tempdir().unwrap();

    fs::write(dir.path().join("small.txt"), b"hello").unwrap();
    fs::write(dir.path().join("large.txt"), vec![0u8; 4096]).unwrap();

    let mut stats = DirStats::new();
    walk_dir(dir.path(), &mut stats);
    top_largest(&mut stats, 1);

    assert_eq!(stats.largest_files.len(), 1);
    assert_eq!(stats.largest_files[0].0, 4096); // largest file is first
}

#[cfg(unix)]
#[test]
fn test_walk_continues_on_unreadable_subdir() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("top.txt"), b"hello").unwrap();

    let subdir = dir.path().join("locked");
    fs::create_dir(&subdir).unwrap();
    fs::write(subdir.join("hidden.txt"), b"world").unwrap();

    fs::set_permissions(&subdir, fs::Permissions::from_mode(0o000)).unwrap();

    let mut stats = DirStats::new();
    let had_error = walk_dir(dir.path(), &mut stats);

    fs::set_permissions(&subdir, fs::Permissions::from_mode(0o755)).unwrap();

    assert!(had_error, "expected walk to flag the permission error");
    assert_eq!(
        stats.file_count, 1,
        "top-level file should still be counted"
    );
}

#[cfg(unix)]
#[test]
fn test_walk_does_not_follow_symlink_loop() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("real.txt"), b"hello").unwrap();

    // self-referential symlink: dir/loop -> dir
    symlink(dir.path(), dir.path().join("loop")).unwrap();

    let mut stats = DirStats::new();
    let _had_error = walk_dir(dir.path(), &mut stats);

    assert_eq!(stats.file_count, 2);
}
