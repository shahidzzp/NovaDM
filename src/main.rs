use futures_util::StreamExt;
use reqwest::header::{CONTENT_DISPOSITION, RANGE};
use reqwest::Client;
use std::env;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::fs::{self, File, OpenOptions};
use tokio::io::{AsyncSeekExt, AsyncWriteExt, SeekFrom};
use tokio::sync::Mutex;

const CONNECTIONS: u64 = 4;
const MAX_RETRIES: u32 = 3;
const MULTI_CONNECTION_MIN_SIZE: u64 = 10 * 1024 * 1024;

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

fn unique_path(directory: &Path, filename: &str) -> PathBuf {
    let original = directory.join(filename);

    if !original.exists() {
        return original;
    }

    let path = Path::new(filename);

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

async fn download_single(
    response: reqwest::Response,
    final_path: &Path,
    existing_size: u64,
    part_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let remaining_size = response.content_length();

    let total_size = if existing_size > 0 {
        remaining_size.map(|remaining| existing_size + remaining)
    } else {
        remaining_size
    };

    println!("Mode: Single connection");

    match total_size {
        Some(size) => println!("Size: {}", format_bytes(size)),
        None => println!("Size: unknown"),
    }

    if existing_size > 0 {
        println!(
            "Already downloaded: {}",
            format_bytes(existing_size)
        );
    }

    println!();
    println!("Press Ctrl+C to interrupt.");
    println!("Run the same command again to resume.");
    println!();

    let mut file = if existing_size > 0 {
        let mut file = OpenOptions::new()
            .write(true)
            .open(part_path)
            .await?;

        file.seek(SeekFrom::Start(existing_size)).await?;
        file
    } else {
        File::create(part_path).await?
    };

    let mut downloaded = existing_size;
    let mut stream = response.bytes_stream();

    let start_time = Instant::now();
    let mut last_display = Instant::now();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;

        file.write_all(&chunk).await?;

        downloaded += chunk.len() as u64;

        if last_display.elapsed() >= Duration::from_millis(250) {
            let elapsed = start_time.elapsed().as_secs_f64();

            let speed = if elapsed > 0.0 {
                downloaded as f64 / elapsed
            } else {
                0.0
            };

            match total_size {
                Some(total) if total > 0 => {
                    let percent =
                        downloaded as f64 / total as f64 * 100.0;

                    let remaining =
                        total.saturating_sub(downloaded);

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

                _ => {
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
    drop(file);

    if let Some(total) = total_size {
        let metadata = fs::metadata(part_path).await?;

        if metadata.len() != total {
            return Err(format!(
                "Download incomplete: {} / {}",
                format_bytes(metadata.len()),
                format_bytes(total)
            )
            .into());
        }
    }

    fs::rename(part_path, final_path).await?;

    println!();
    println!();
    println!("✓ Download complete!");
    println!("✓ Saved to: {}", final_path.display());

    Ok(())
}

async fn download_segment(
    client: Client,
    url: String,
    start: u64,
    end: u64,
    part_path: PathBuf,
    progress: Arc<Mutex<u64>>,
    connection_number: u64,
) -> Result<(), String> {
    let expected_size = end - start + 1;

    for attempt in 1..=MAX_RETRIES {
        if attempt > 1 {
            println!(
                "Connection {} retry {}/{}...",
                connection_number,
                attempt,
                MAX_RETRIES
            );

            tokio::time::sleep(Duration::from_secs(1)).await;
        }

        let response = match client
            .get(&url)
            .header(
                RANGE,
                format!("bytes={}-{}", start, end),
            )
            .send()
            .await
        {
            Ok(response) => response,

            Err(error) => {
                if attempt == MAX_RETRIES {
                    return Err(format!(
                        "Connection {} request failed: {}",
                        connection_number, error
                    ));
                }

                continue;
            }
        };

        if response.status()
            != reqwest::StatusCode::PARTIAL_CONTENT
        {
            if attempt == MAX_RETRIES {
                return Err(format!(
                    "Connection {}: expected HTTP 206, got HTTP {}",
                    connection_number,
                    response.status()
                ));
            }

            continue;
        }

        let _ = fs::remove_file(&part_path).await;

        let mut file = match File::create(&part_path).await {
            Ok(file) => file,

            Err(error) => {
                return Err(format!(
                    "Connection {} could not create segment: {}",
                    connection_number, error
                ));
            }
        };

        let mut stream = response.bytes_stream();

        let mut received: u64 = 0;

        while let Some(chunk_result) = stream.next().await {
            let chunk = match chunk_result {
                Ok(chunk) => chunk,

                Err(error) => {
                    let _ = fs::remove_file(&part_path).await;

                    if attempt == MAX_RETRIES {
                        return Err(format!(
                            "Connection {} stream failed: {}",
                            connection_number, error
                        ));
                    }

                    continue;
                }
            };

            let chunk_len = chunk.len() as u64;

            if received + chunk_len > expected_size {
                let allowed =
                    expected_size.saturating_sub(received);

                if allowed > 0 {
                    if let Err(error) =
                        file.write_all(&chunk[..allowed as usize]).await
                    {
                        return Err(format!(
                            "Connection {} write failed: {}",
                            connection_number, error
                        ));
                    }

                    received += allowed;

                    let mut current = progress.lock().await;
                    *current += allowed;
                }

                break;
            }

            if let Err(error) = file.write_all(&chunk).await {
                return Err(format!(
                    "Connection {} write failed: {}",
                    connection_number, error
                ));
            }

            received += chunk_len;

            let mut current = progress.lock().await;
            *current += chunk_len;
        }

        if let Err(error) = file.flush().await {
            return Err(format!(
                "Connection {} flush failed: {}",
                connection_number, error
            ));
        }

        drop(file);

        if received == expected_size {
            println!(
                "✓ Connection {} complete ({})",
                connection_number,
                format_bytes(received)
            );

            return Ok(());
        }

        {
            let mut current = progress.lock().await;
            *current = current.saturating_sub(received);
        }

        println!(
            "Connection {} incomplete: {} / {}",
            connection_number,
            format_bytes(received),
            format_bytes(expected_size)
        );

        let _ = fs::remove_file(&part_path).await;

        if attempt == MAX_RETRIES {
            return Err(format!(
                "Connection {} failed after {} attempts",
                connection_number, MAX_RETRIES
            ));
        }
    }

    Err(format!(
        "Connection {} failed",
        connection_number
    ))
}

async fn combine_parts(
    part_paths: &[PathBuf],
    final_path: &Path,
    total_size: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Verifying segments...");

    let mut combined_size: u64 = 0;

    for (index, part_path) in part_paths.iter().enumerate() {
        let metadata = fs::metadata(part_path).await?;

        println!(
            "Segment {}: {}",
            index + 1,
            format_bytes(metadata.len())
        );

        combined_size += metadata.len();
    }

    if combined_size != total_size {
        return Err(format!(
            "Segment verification failed: {} / {}",
            format_bytes(combined_size),
            format_bytes(total_size)
        )
        .into());
    }

    println!("✓ All segments verified.");
    println!("Combining segments...");

    let mut final_file = File::create(final_path).await?;

    for part_path in part_paths {
        let mut part_file = File::open(part_path).await?;

        tokio::io::copy(
            &mut part_file,
            &mut final_file,
        )
        .await?;
    }

    final_file.flush().await?;
    drop(final_file);

    let metadata = fs::metadata(final_path).await?;

    if metadata.len() != total_size {
        let _ = fs::remove_file(final_path).await;

        return Err(format!(
            "Final file verification failed: {} / {}",
            format_bytes(metadata.len()),
            format_bytes(total_size)
        )
        .into());
    }

    for part_path in part_paths {
        let _ = fs::remove_file(part_path).await;
    }

    println!("✓ Final file verified.");

    Ok(())
}

async fn download_multi(
    client: &Client,
    url: &str,
    final_path: &Path,
    total_size: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Mode: Multi-connection");
    println!("Connections: {}", CONNECTIONS);
    println!("Size: {}", format_bytes(total_size));
    println!();

    let segment_size = total_size / CONNECTIONS;

    let progress = Arc::new(Mutex::new(0u64));

    let mut handles = Vec::new();
    let mut part_paths = Vec::new();

    for index in 0..CONNECTIONS {
        let start = index * segment_size;

        let end = if index == CONNECTIONS - 1 {
            total_size - 1
        } else {
            ((index + 1) * segment_size) - 1
        };

        let part_path = final_path.with_extension(
            format!(
                "{}.part{}",
                final_path
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("download"),
                index + 1
            ),
        );

        println!(
            "Connection {} → {} - {}",
            index + 1,
            format_bytes(start),
            format_bytes(end + 1)
        );

        part_paths.push(part_path.clone());

        let client_clone = client.clone();
        let url_clone = url.to_string();
        let progress_clone = Arc::clone(&progress);
        let connection_number = index + 1;

        let handle = tokio::spawn(async move {
            download_segment(
                client_clone,
                url_clone,
                start,
                end,
                part_path,
                progress_clone,
                connection_number,
            )
            .await
        });

        handles.push(handle);
    }

    println!();
    println!(
        "Downloading with {} connections...",
        CONNECTIONS
    );
    println!();

    let start_time = Instant::now();

    loop {
        let finished =
            handles.iter().all(|handle| handle.is_finished());

        let downloaded = *progress.lock().await;

        let elapsed = start_time.elapsed().as_secs_f64();

        let speed = if elapsed > 0.0 {
            downloaded as f64 / elapsed
        } else {
            0.0
        };

        let percent =
            downloaded as f64 / total_size as f64 * 100.0;

        let remaining =
            total_size.saturating_sub(downloaded);

        let eta = if speed > 0.0 {
            (remaining as f64 / speed) as u64
        } else {
            0
        };

        print!(
            "\rProgress: {:6.2}% | {} / {} | {}/s | ETA {}",
            percent,
            format_bytes(downloaded),
            format_bytes(total_size),
            format_bytes(speed as u64),
            format_duration(eta)
        );

        std::io::Write::flush(
            &mut std::io::stdout()
        )?;

        if finished {
            break;
        }

        tokio::time::sleep(
            Duration::from_millis(250)
        )
        .await;
    }

    println!();
    println!();

    for handle in handles {
        match handle.await {
            Ok(Ok(())) => {}

            Ok(Err(error)) => {
                return Err(error.into());
            }

            Err(error) => {
                return Err(format!(
                    "Download task failed: {}",
                    error
                )
                .into());
            }
        }
    }

    combine_parts(
        &part_paths,
        final_path,
        total_size,
    )
    .await?;

    println!();
    println!("✓ Download complete!");
    println!("✓ Saved to: {}", final_path.display());
    println!(
        "✓ Final size: {}",
        format_bytes(total_size)
    );

    Ok(())
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

    println!("Checking server...");

    let head_response = client.head(url).send().await?;

    if !head_response.status().is_success() {
        return Err(format!(
            "Server check failed: HTTP {}",
            head_response.status()
        )
        .into());
    }

    let head_size = head_response
        .content_length()
        .filter(|size| *size > 0);

    let filename = head_response
        .headers()
        .get(CONTENT_DISPOSITION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| {
            value
                .split("filename=")
                .nth(1)
                .map(|name| {
                    name.trim_matches('"')
                        .trim_matches('\'')
                        .to_string()
                })
        })
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| filename_from_url(url));

    let home = env::var("HOME")
        .map(PathBuf::from)
        .map_err(|_| "Could not determine HOME directory")?;

    let download_directory = home.join("Downloads");

    fs::create_dir_all(&download_directory).await?;

    let final_path =
        unique_path(&download_directory, &filename);

    let part_path = final_path.with_extension(
        format!(
            "{}.part",
            final_path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("download")
        ),
    );

    /*
     * Check for an existing single-connection
     * partial download.
     */
    if let Ok(metadata) = fs::metadata(&part_path).await {
        let existing_size = metadata.len();

        if existing_size > 0 {
            println!(
                "Found partial download: {}",
                format_bytes(existing_size)
            );

            println!("Attempting to resume...");

            let resume_response = client
                .get(url)
                .header(
                    RANGE,
                    format!("bytes={}-", existing_size),
                )
                .send()
                .await?;

            if resume_response.status()
                == reqwest::StatusCode::PARTIAL_CONTENT
            {
                println!("✓ Server supports resume.");

                return download_single(
                    resume_response,
                    &final_path,
                    existing_size,
                    &part_path,
                )
                .await;
            }

            println!(
                "Server did not accept the resume request."
            );

            println!("Starting this download again.");

            let _ = fs::remove_file(&part_path).await;
        }
    }

    /*
     * Determine file size.
     */
    let total_size: Option<u64>;

    if let Some(size) = head_size {
        total_size = Some(size);
    } else {
        println!(
            "Server did not provide the file size through HEAD."
        );

        println!("Checking with GET request...");

        let response = client.get(url).send().await?;

        if !response.status().is_success() {
            return Err(format!(
                "Download failed: HTTP {}",
                response.status()
            )
            .into());
        }

        total_size = response
            .content_length()
            .filter(|size| *size > 0);
    }

    println!("File: {}", final_path.display());

    match total_size {
        Some(size) => println!("Size: {}", format_bytes(size)),
        None => println!("Size: unknown"),
    }

    /*
     * Unknown file size = single connection.
     */
    let total_size = match total_size {
        Some(size) if size > 0 => size,

        _ => {
            println!();
            println!("File size unknown.");
            println!("Using single-connection mode.");

            let response = client.get(url).send().await?;

            if !response.status().is_success() {
                return Err(format!(
                    "Download failed: HTTP {}",
                    response.status()
                )
                .into());
            }

            return download_single(
                response,
                &final_path,
                0,
                &part_path,
            )
            .await;
        }
    };

    /*
     * Small files use a single connection.
     */
    if total_size < MULTI_CONNECTION_MIN_SIZE {
        println!();
        println!("File is smaller than 10 MB.");
        println!("Using single-connection mode.");

        let response = client.get(url).send().await?;

        if !response.status().is_success() {
            return Err(format!(
                "Download failed: HTTP {}",
                response.status()
            )
            .into());
        }

        return download_single(
            response,
            &final_path,
            0,
            &part_path,
        )
        .await;
    }

    /*
     * Test HTTP Range support.
     */
    println!("Testing HTTP Range support...");

    let range_test = client
        .get(url)
        .header(RANGE, "bytes=0-0")
        .send()
        .await?;

    if range_test.status()
        == reqwest::StatusCode::PARTIAL_CONTENT
    {
        println!(
            "✓ Server supports HTTP Range requests."
        );

        println!(
            "✓ Starting multi-connection download."
        );

        println!();

        drop(range_test);

        return download_multi(
            &client,
            url,
            &final_path,
            total_size,
        )
        .await;
    }

    println!(
        "Server does not support HTTP Range requests."
    );

    println!("Using single-connection mode.");

    drop(range_test);

    let response = client.get(url).send().await?;

    if !response.status().is_success() {
        return Err(format!(
            "Download failed: HTTP {}",
            response.status()
        )
        .into());
    }

    download_single(
        response,
        &final_path,
        0,
        &part_path,
    )
    .await?;

    Ok(())
}
