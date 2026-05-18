use clap::Parser;
use du_rs::{format_size, top_largest, walk_dir, DirStats};
use std::path::Path;
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "du-rs", about = "Disk usage analyzer", version)]
struct Cli {
    /// Directory to analyze
    #[arg(default_value = ".")]
    path: String,

    /// Number of largest files to display
    #[arg(long, default_value_t = 10)]
    top: usize,

    /// Show sizes in human-readable format (KB, MB, GB)
    #[arg(short = 'H', long)]
    human_readable: bool,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let path = Path::new(&cli.path);

    let mut stats = DirStats::new();

    let had_error = walk_dir(path, &mut stats);

    top_largest(&mut stats, cli.top);

    let size_display = if cli.human_readable {
        format_size(stats.total_size)
    } else {
        format!("{} bytes", stats.total_size)
    };

    println!("Total size : {}", size_display);
    println!("File count : {}", stats.file_count);
    println!("\nTop {} largest files:", cli.top);

    for (size, path) in &stats.largest_files {
        let size_str = if cli.human_readable {
            format_size(*size)
        } else {
            format!("{} bytes", size)
        };
        println!("  {:>12}  {}", size_str, path);
    }

    if had_error {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
