use std::fs;
use std::path::Path;

pub struct DirStats {
    pub file_count: usize,
    pub total_size: u64,
    pub largest_files: Vec<(u64, String)>,
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

pub fn walk_dir(path: &Path, stats: &mut DirStats) -> std::io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        let entry_path = entry.path();

        if metadata.is_dir() {
            walk_dir(&entry_path, stats)?;
        } else {
            let size = metadata.len();
            stats.file_count += 1;
            stats.total_size += size;
            stats
                .largest_files
                .push((size, entry_path.to_string_lossy().into_owned()));
        }
    }
    Ok(())
}

pub fn top_largest(stats: &mut DirStats, n: usize) {
    stats.largest_files.sort_by(|a, b| b.0.cmp(&a.0));
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
