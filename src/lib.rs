use std::fs;
use std::path::{Path, PathBuf};

pub struct DirStats {
    pub file_count: usize,
    pub total_size: u64,
    pub largest_files: Vec<(u64, PathBuf)>,
}

impl DirStats {
    pub fn new() -> Self {
        DirStats {
            file_count: 0,
            total_size: 0,
            largest_files: Vec::new(),
        }
    }
}

impl Default for DirStats {
    fn default() -> Self {
        Self::new()
    }
}

pub fn walk_dir(path: &Path, stats: &mut DirStats) -> bool {
    let entries = match fs::read_dir(path) {
        Ok(it) => it,
        Err(e) => {
            eprintln!("du-rs: cannot ready directory '{}': {}", path.display(), e);
            return true;
        }
    };

    let mut had_error = false;

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                eprintln!("du-rs: error reading entry in '{}': {}", path.display(), e);
                had_error = true;
                continue;
            }
        };

        let entry_path = entry.path();

        let metadata = match fs::symlink_metadata(&entry_path) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("du-rs: cannot access '{}': {}", entry_path.display(), e);
                had_error = true;
                continue;
            }
        };

        if metadata.is_dir() {
            had_error |= walk_dir(&entry_path, stats);
        } else {
            let size = metadata.len();
            stats.file_count += 1;
            stats.total_size += size;
            stats.largest_files.push((size, entry_path));
        }
    }

    had_error
}

pub fn top_largest(stats: &mut DirStats, n: usize) {
    stats.largest_files.sort_by_key(|f| std::cmp::Reverse(f.0));
    stats.largest_files.truncate(n);
}

pub fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(0), "0 B");
        assert_eq!(format_size(512), "512 B");
        assert_eq!(format_size(1024), "1.00 KB");
        assert_eq!(format_size(1_048_576), "1.00 MB");
        assert_eq!(format_size(1_073_741_824), "1.00 GB");
    }

    #[test]
    fn test_dirstats_new() {
        let stats = DirStats::new();
        assert_eq!(stats.file_count, 0);
        assert_eq!(stats.total_size, 0);
        assert!(stats.largest_files.is_empty());
    }
}
