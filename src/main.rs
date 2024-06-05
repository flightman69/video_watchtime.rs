// TODO
// - store the content of the path DONE
// - look for video files (mkv, mp4, webm, m4v, mov etcc) Done
// - calculate the watch time for those files
// - print out watch time based on each file Later
// - print out entire watch time Done
// - compare it with something random
//      - for nown just 3 (3 idiots, the iron man, ghilee)
//      - and print any random one

// start fresh counter: 3

use std::fs;
use std::io;
use std::process::Command;
use std::str;
use std::vec::Vec;

fn list_dir(path: &String) -> io::Result<Vec<String>> {
    let mut file_names: Vec<String> = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name = match file_name.into_string() {
            Ok(name) => name,
            Err(_) => continue,
        };

        file_names.push(file_name);
    }

    Ok(file_names)
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

fn main() -> io::Result<()> {
    let mut path = String::new();
    io::stdin()
        .read_line(&mut path)
        .expect("Failed to read line");
    let path = path.trim().to_string();

    let files = list_dir(&path)?;
    let video_files = get_video_files(files)?;
    let mut total_watch_time = 0.0;
    for video_file in video_files {
        let video_file = path.clone() + "/" + &video_file;
        match get_watch_time(&video_file) {
            Ok(val) => total_watch_time += val,
            Err(e) => println!("Error processing video {}: {}", video_file, e),
        };
    }

    let watch_time_str = fmt_watch_time(total_watch_time);
    println!("You have {} worth of contents", watch_time_str);

    Ok(())
}