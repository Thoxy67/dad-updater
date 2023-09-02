use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::{path::Path, sync::Arc};
mod download;
use clap::Parser;
use colored::Colorize;
use download::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, env = "DAD_PATH")]
    path: String,
    #[arg(short, long, env = "DAD_DOWNLOAD_SPEED", default_value_t = 0)]
    speed: u64,
    #[arg(short, long, env = "DAD_THREADS", default_value_t = 5)]
    threads: usize,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let file_urls = get_urls(args.path).await?;

    let semaphore = Arc::new(tokio::sync::Semaphore::new(args.threads));
    let mut join_handles = Vec::new();

    let multi_progress = MultiProgress::new();

    for u in file_urls {
        let semaphore_permit = acquire_semaphore_permit(semaphore.clone()).await;

        // Create a new ProgressBar for each download task
        let progress_bar = multi_progress.add(ProgressBar::new(u.size));
        let sty = ProgressStyle::with_template(
            format!(
                "{} {}",
                u.file_name, "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta}) {msg}",
            )
            .as_str(),
        )
        .unwrap()
        .progress_chars("##-");

        progress_bar.set_style(sty);

        let download_task = tokio::spawn(download_file(
            u,
            semaphore_permit,
            args.speed,
            progress_bar.clone(),
        ));

        join_handles.push(download_task);
    }

    wait_for_tasks_completion(join_handles).await;
    let _ = multi_progress.clear();
    drop(multi_progress);

    println!("\n\n{}", "Dark and Darker up to date".green());

    Ok(())
}

async fn get_urls(path: String) -> Result<Vec<Download>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::USER_AGENT,
        reqwest::header::HeaderValue::from_static(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/114.0",
        ),
    );
    headers.insert(
        reqwest::header::CONNECTION,
        reqwest::header::HeaderValue::from_static("keep-alive"),
    );
    headers.insert(
        reqwest::header::CACHE_CONTROL,
        reqwest::header::HeaderValue::from_static("no-cache"),
    );

    headers.insert(
        reqwest::header::HOST,
        reqwest::header::HeaderValue::from_static("cdn.darkanddarker.com"),
    );

    let res = client
        .get("http://cdn.darkanddarker.com/Dark%20and%20Darker/PatchFileList.txt")
        .headers(headers)
        .send()
        .await?
        .bytes()
        .await?;
    let body_str = format!("path,sha256,size\n{}", String::from_utf8_lossy(&res));

    let mut files = Vec::new();

    let mut r = csv::Reader::from_reader(body_str.as_bytes());
    for record in r.records() {
        let record = record?;
        let file_path = record[0].to_string();
        let file_name = file_path.split('\\').last().unwrap().to_string();
        let uri = file_path.replace("\\", "/");
        let sha256 = record[1].to_string();
        let size = record[2].parse::<u64>().unwrap();
        // Remove the first character
        let full_path = Path::new(&path).join(&uri.clone().get(1..).unwrap().to_string());
        files.push(Download {
            path: file_path,
            linux_path: full_path.as_os_str().to_str().unwrap().to_string(),
            url: format!(
                "http://cdn.darkanddarker.com/Dark%20and%20Darker/Patch{}",
                uri
            ),
            file_name,
            sha256,
            size,
        });
    }
    Ok(files)
}
