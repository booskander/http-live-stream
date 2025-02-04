pub mod file_manager {
    use std::path::Path;
    use std::process::Command;
    use std::process::Stdio;
    use tokio::fs;

    #[derive(Debug)]
    pub enum FileError {
        FileExists,
        FileNotFound,
        FolderCreationFailed,
        FfmpegError,
    }

    pub async fn create_hls_stream(video_title: &str) -> Result<String, FileError> {
        let out_dir = format!("video/{}", video_title);

        if Path::new(&out_dir).exists() {
            return Ok(out_dir);
        }

        let video_file_path = format!("assets/{}.webm", video_title);
        if !Path::new(&video_file_path).exists() {
            return Err(FileError::FileNotFound);
        }

        let _ = create_folder(&out_dir).await;

        if lunch_ffmpeg(&video_file_path, &out_dir).await {
            Ok(out_dir)
        } else {
            Err(FileError::FfmpegError)
        }
    }

    async fn create_folder(path: &str) -> String {
        if !Path::new(path).exists() {
            match fs::create_dir_all(path).await {
                Ok(_) => {
                    println!("Folder created successfully: {}", path);
                    return path.to_string();
                }
                Err(e) => {
                    eprintln!("Error creating folder: {}", e);
                    return String::new();
                }
            }
        }
        println!("Folder already exists: {}", path);
        path.to_string()
    }

    async fn lunch_ffmpeg(input_path: &str, output_path: &str) -> bool {
        let output_file = format!("{}/playlist.m3u8", output_path);
        let process = Command::new("ffmpeg")
            .arg("-i")
            .arg(input_path)
            .arg("-vf")
            .arg("scale=iw-mod(iw\\,2):ih-mod(ih\\,2)") // Correct scaling filter
            .arg("-codec:v")
            .arg("libx264")
            .arg("-codec:a")
            .arg("aac")
            .arg("-f")
            .arg("hls")
            .arg("-hls_time")
            .arg("4")
            .arg("-hls_playlist_type")
            .arg("vod")
            .arg(&output_file)
            .stderr(Stdio::piped())
            .output()
            .expect("Failed to execute ffmpeg");

        if process.status.success() {
            println!("FFmpeg process succeeded.");
        } else {
            eprintln!(
                "FFmpeg process failed. Stderr: {}",
                String::from_utf8_lossy(&process.stderr)
            );
        }

        process.status.success()
    }
}
