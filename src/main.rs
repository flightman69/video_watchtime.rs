use std::env;
use std::io;
use std::process::Command;
use std::str;
use std::vec::Vec;
use walkdir::WalkDir;

fn list_dir_recursive(path: &str) -> io::Result<Vec<String>> {
    let mut files: Vec<String> = Vec::new();
    let walker = WalkDir::new(path).into_iter();

    for entry in walker {
        match entry {
            Ok(entry) => {
                let file_name = entry.path().display().to_string();
                files.push(file_name);
            }
            Err(e) => {
                eprintln!("Error reading file {}", e);
            }
        }
    }
    Ok(files)
}

fn get_video_files(files: Vec<String>) -> io::Result<Vec<String>> {
    let video_extentions = vec!["mp4", "mkv", "mov", "webm"];

    let video_files: Vec<String> = files
        .into_iter()
        .filter(|file_name| video_extentions.iter().any(|ext| file_name.ends_with(ext)))
        .collect();

    Ok(video_files)
}

fn get_watch_time(video_file: &str) -> Result<f64, String> {
    let output = Command::new("ffprobe")
        .args(&[
            "-v",
            "error",
            "-show_entries",
            "format=duration",
            "-of",
            "default=noprint_wrappers=1:nokey=1",
            video_file,
        ])
        .output()
        .map_err(|e| format!("Failed to execute ffprobe: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "ffprobe failed with error: {}",
            str::from_utf8(&output.stderr).unwrap_or("unknown error")
        ));
    }

    let duration_str = str::from_utf8(&output.stdout)
        .map_err(|e| format!("Failed to parse ffprobe output: {}", e))?;

    let duration_secs: f64 = duration_str
        .trim()
        .parse::<f64>()
        .map_err(|e| format!("Failed to parse duration as f64: {}", e))?;

    Ok(duration_secs)
}

fn fmt_watch_time(watch_time_in_sec: f64) -> String {
    let hours = (watch_time_in_sec / 3600.0) as u32;
    let minutes = ((watch_time_in_sec % 3600.0) / 60.0) as u32;
    let seconds = (watch_time_in_sec % 60.0) as u32;

    format!(
        "{:02} Hours, {:02} Minutes, {:02} Seconds",
        hours, minutes, seconds
    )
}
fn help_message() {
    println!("[Video Watch Time]");
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    println!("Usage: wt <video folder>");
    println!("Options:");
    println!("    -h, --help     Display help message");
    println!("    -v, --version  Display version");
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        help_message();
        return Ok(());
    }

    match args[1].as_str() {
        "-h" => {
            help_message();
            return Ok(());
        }
        "--help" => {
            help_message();
            return Ok(());
        }
        "-v" => {
            println!("Version: {}", env!("CARGO_PKG_VERSION"));
            return Ok(());
        }
        "--version" => {
            println!("Version: {}", env!("CARGO_PKG_VERSION"));
            return Ok(());
        }
        _ => (),
    }

    let path = &args[1];
    let files = list_dir_recursive(path)?;
    let video_files = get_video_files(files)?;
    let mut total_watch_time = 0.0;
    for video_file in video_files {
        match get_watch_time(&video_file) {
            Ok(val) => total_watch_time += val,
            Err(e) => println!("Error processing video {}: {}", video_file, e),
        };
    }

    let watch_time_str = fmt_watch_time(total_watch_time);
    println!("You have {} worth of contents", watch_time_str);

    Ok(())
}
