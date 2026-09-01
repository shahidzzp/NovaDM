use futures_util::StreamExt;
use reqwest::Client;
use std::env;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: novadm <URL>");
        return Ok(());
    }

    let url = &args[1];

    println!("NovaDM");
    println!("Downloading: {url}");

    let client = Client::new();

    let response = client.get(url).send().await?;

    if !response.status().is_success() {
        return Err(format!("Server returned {}", response.status()).into());
    }

    let total_size = response.content_length();

    let filename = url
        .split('/')
        .next_back()
        .filter(|name| !name.is_empty())
        .unwrap_or("download");

    let output = PathBuf::from(filename);
    let mut file = File::create(&output).await?;

    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk).await?;

        downloaded += chunk.len() as u64;

        match total_size {
            Some(total) => {
                let percent = downloaded as f64 / total as f64 * 100.0;

                print!(
                    "\rProgress: {:.1}% ({}/{} MB)",
                    percent,
                    downloaded / 1_048_576,
                    total / 1_048_576
                );
            }
            None => {
                print!(
                    "\rDownloaded: {} MB",
                    downloaded / 1_048_576
                );
            }
        }
    }

    file.flush().await?;

    println!("\nDownload complete: {}", output.display());

    Ok(())
}
