mod download;
mod job;
mod manager;

use download::{
    download_multi,
    download_single,
    MULTI_CONNECTION_MIN_SIZE,
};

use manager::DownloadManager;

use reqwest::header::{CONTENT_DISPOSITION, RANGE};
use reqwest::Client;

use std::env;
use std::path::{Path, PathBuf};

use tokio::fs;

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("NovaDM");
        println!();
        println!("Usage:");
        println!("  novadm <URL>");
        return Ok(());
    }

    let url = &args[1];

    println!("╔══════════════════════════════════╗");
    println!("║             NovaDM               ║");
    println!("╚══════════════════════════════════╝");
    println!();

    println!("URL: {}", url);
    println!();

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
     * Create the download manager.
     */
    let mut manager = DownloadManager::new();

    /*
     * Check for an existing partial download.
     */
    if let Ok(metadata) = fs::metadata(&part_path).await {
        let existing_size = metadata.len();

        if existing_size > 0 {
            println!(
                "Found partial download: {} bytes",
                existing_size
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
                println!();

                let job_id = manager.add_job(
                    url.to_string(),
                    filename.clone(),
                    final_path.clone(),
                    None,
                );

                println!(
                    "Download job created: #{}",
                    job_id
                );

                manager.start_job(job_id);

                let result = download_single(
                    resume_response,
                    &final_path,
                    existing_size,
                    &part_path,
                )
                .await;

                if result.is_ok() {
                    manager.complete_job(job_id);
                } else {
                    manager.fail_job(job_id);
                }

                return result;
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
        Some(size) => {
            println!(
                "Size: {:.2} MB",
                size as f64 / 1_048_576.0
            );
        }

        None => {
            println!("Size: unknown");
        }
    }

    /*
     * Add the download to the manager.
     */
    let job_id = manager.add_job(
        url.to_string(),
        filename.clone(),
        final_path.clone(),
        total_size,
    );

    println!(
        "Download job created: #{}",
        job_id
    );

    /*
     * Unknown file size.
     */
    let total_size = match total_size {
        Some(size) if size > 0 => size,

        _ => {
            println!();
            println!("File size unknown.");
            println!("Using single-connection mode.");

            manager.start_job(job_id);

            let response = client.get(url).send().await?;

            if !response.status().is_success() {
                manager.fail_job(job_id);

                return Err(format!(
                    "Download failed: HTTP {}",
                    response.status()
                )
                .into());
            }

            let result = download_single(
                response,
                &final_path,
                0,
                &part_path,
            )
            .await;

            if result.is_ok() {
                manager.complete_job(job_id);
            } else {
                manager.fail_job(job_id);
            }

            return result;
        }
    };

    /*
     * Small files use one connection.
     */
    if total_size < MULTI_CONNECTION_MIN_SIZE {
        println!();
        println!("File is smaller than 10 MB.");
        println!("Using single-connection mode.");

        manager.start_job(job_id);

        let response = client.get(url).send().await?;

        if !response.status().is_success() {
            manager.fail_job(job_id);

            return Err(format!(
                "Download failed: HTTP {}",
                response.status()
            )
            .into());
        }

        let result = download_single(
            response,
            &final_path,
            0,
            &part_path,
        )
        .await;

        if result.is_ok() {
            manager.complete_job(job_id);
        } else {
            manager.fail_job(job_id);
        }

        return result;
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

        manager.start_job(job_id);

        let result = download_multi(
            &client,
            url,
            &final_path,
            total_size,
        )
        .await;

        if result.is_ok() {
            manager.complete_job(job_id);
        } else {
            manager.fail_job(job_id);
        }

        return result;
    }

    /*
     * Server does not support Range.
     */
    println!(
        "Server does not support HTTP Range requests."
    );

    println!("Using single-connection mode.");

    drop(range_test);

    manager.start_job(job_id);

    let response = client.get(url).send().await?;

    if !response.status().is_success() {
        manager.fail_job(job_id);

        return Err(format!(
            "Download failed: HTTP {}",
            response.status()
        )
        .into());
    }

    let result = download_single(
        response,
        &final_path,
        0,
        &part_path,
    )
    .await;

    if result.is_ok() {
        manager.complete_job(job_id);
    } else {
        manager.fail_job(job_id);
    }

    result
}
