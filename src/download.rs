use futures::prelude::*;
use indicatif::ProgressBar;
use sha256::try_digest;
use std::{error::Error, path::Path, result::Result, sync::Arc};
use tokio::io::AsyncWriteExt;

pub async fn acquire_semaphore_permit(
    semaphore: Arc<tokio::sync::Semaphore>,
) -> tokio::sync::OwnedSemaphorePermit {
    semaphore.acquire_owned().await.unwrap()
}

pub async fn download_file(
    url: Download,
    semaphore_permit: tokio::sync::OwnedSemaphorePermit,
    download_speed_limit: u64,
    progress_bar: ProgressBar,
) {
    let full_path = Path::new(&url.linux_path);
    let prefix = full_path.parent().unwrap();

    if full_path.exists() {
        if existing_files(url.clone()).await.is_ok() {
            progress_bar.finish_with_message("[skipped] already up to date");
            return;
        }
    }

    std::fs::create_dir_all(prefix).unwrap();

    let mut file = tokio::fs::File::create(&full_path).await.unwrap();
    let mut chunk = reqwest::get(&url.url).await.unwrap().bytes_stream();

    while let Some(chunk) = chunk.next().await {
        let chunk = chunk.unwrap();
        let length = chunk.len();

        file.write_all(&chunk).await.unwrap();

        if download_speed_limit != 0 {
            let delay_duration = std::time::Duration::from_micros(
                (length as u64 * 1_000_000) / download_speed_limit,
            );

            tokio::time::sleep(delay_duration).await;
        }
        progress_bar.inc(length as u64);
    }
    if url.real_file_name.is_some() {
        std::fs::copy(full_path, url.real_file_name.unwrap()).unwrap();
    }
    progress_bar.finish_with_message("[ok] file updated");
    drop(semaphore_permit);
}

pub async fn wait_for_tasks_completion(join_handles: Vec<tokio::task::JoinHandle<()>>) {
    for handle in join_handles {
        handle.await.unwrap();
    }
}

pub async fn existing_files(download: Download) -> Result<(), Box<dyn Error>> {
    let p = Path::new(&download.linux_path);

    let val = try_digest(p)?;

    if val == download.sha256 && download.size == p.metadata()?.len() {
        Ok(())
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "File is not up to date",
        )))
    }
}

#[derive(Debug, Clone)]
pub struct Download {
    pub url: String,
    pub file_name: String,
    pub path: String,
    pub sha256: String,
    pub size: u64,
    pub linux_path: String,
    pub real_file_name: Option<String>,
}
