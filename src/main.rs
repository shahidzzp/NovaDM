use futures_util::StreamExt;
use reqwest::Client;
use std::env;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;

fn filename_from_url(url: &str) -> String {
    url.trim_end_matches('/')
        .rsplit('/')
        .next()
        .filter(|name| !name.is_empty())
        .unwrap_or("download")
        .split('?')
        .next()
        .unwrap_or("download")
        .to_string()
}

fn format_bytes(bytes: u64) -> String {
    if bytes >= 1_073_741_824 {
        format!("{:.2} GB", bytes as f64 / 1_073_741_824.0)
    } else if bytes >= 1_048_576 {
        format!("{:.2} MB", bytes as f64 / 1_048_576.0)
    } else if bytes >= 1024 {
        format!("{:.2} KB", bytes as f64 / 1024.0)
    } else {
        format!("{} B", bytes)
    }
}

fn format_duration(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;

    if hours > 0 {
        format!("{:02}:{:02}:{:02}", hours, minutes, secs)
    } else {
        format!("{:02}:{:02}", minutes, secs)
    }
}

fn unique_path(directory: &PathBuf, filename: &str) -> PathBuf {
    let original = directory.join(filename);

    if !original.exists() {
        return original;
    }

    let path = std::path::Path::new(filename);

    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("download");

    let extension = path.extension().and_then(|s| s.to_str());

    for number in 1.. {
        let candidate_name = match extension {
            Some(ext) => format!("{} ({}).{}", stem, number, ext),
            None => format!("{} ({})", stem, number),
        };

        let candidate = directory.join(candidate_name);

        if !candidate.exists() {
            return candidate;
        }
    }

    unreachable!()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("NovaDM");
        println!("Usage: novadm <URL>");
        return Ok(());
    }

    let url = &args[1];

    println!("╔══════════════════════════════════╗");
    println!("║             NovaDM               ║");
    println!("╚══════════════════════════════════╝");
    println!();
    println!("URL: {}", url);

    let client = Client::builder()
        .user_agent("NovaDM/0.1")
        .build()?;

    let response = client.get(url).send().await?;

    if !response.status().is_success() {
        return Err(format!(
            "Download failed: HTTP {}",
            response.status()
        )
        .into());
    }

    let total_size = response.content_length();

    let filename = response
        .headers()
        .get(reqwest::header::CONTENT_DISPOSITION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| {
            value
                .split("filename=")
                .nth(1)
                .map(|name| name.trim_matches('"').trim_matches('\'').to_string())
        })
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| filename_from_url(url));

    let home = env::var("HOME")
        .map(PathBuf::from)
        .map_err(|_| "Could not determine HOME directory")?;

    let download_directory = home.join("Downloads");

    fs::create_dir_all(&download_directory).await?;

    let output_path = unique_path(&download_directory, &filename);

    println!("File: {}", output_path.display());

    match total_size {
        Some(size) => println!("Size: {}", format_bytes(size)),
        None => println!("Size: unknown"),
    }

    println!();

    let mut file = File::create(&output_path).await?;

    let mut stream = response.bytes_stream();

    let mut downloaded: u64 = 0;
    let start = Instant::now();
    let mut last_display = Instant::now();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;

        file.write_all(&chunk).await?;

        downloaded += chunk.len() as u64;

        if last_display.elapsed() >= Duration::from_millis(250) {
            let elapsed = start.elapsed().as_secs_f64();

            let speed = if elapsed > 0.0 {
                downloaded as f64 / elapsed
            } else {
                0.0
            };

            match total_size {
                Some(total) => {
                    let percent = downloaded as f64 / total as f64 * 100.0;

                    let remaining = total.saturating_sub(downloaded);

                    let eta = if speed > 0.0 {
                        (remaining as f64 / speed) as u64
                    } else {
                        0
                    };

                    print!(
                        "\rProgress: {:6.2}% | {} / {} | {}/s | ETA {}",
                        percent,
                        format_bytes(downloaded),
                        format_bytes(total),
                        format_bytes(speed as u64),
                        format_duration(eta)
                    );
                }

                None => {
                    print!(
                        "\rDownloaded: {} | {}/s",
                        format_bytes(downloaded),
                        format_bytes(speed as u64)
                    );
                }
            }

            std::io::Write::flush(&mut std::io::stdout())?;

            last_display = Instant::now();
        }
    }

    file.flush().await?;

    println!();
    println!();
    println!("✓ Download complete!");
    println!("✓ Saved to: {}", output_path.display());

    Ok(())
}
