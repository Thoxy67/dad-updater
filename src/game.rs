use crate::config::*;
use colored::Colorize;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::{path::Path, sync::Arc};

pub async fn get_game_urls(
    path: String,
) -> Result<Vec<crate::download::Download>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::USER_AGENT,
        reqwest::header::HeaderValue::from_static(USER_AGENT),
    );
    headers.insert(
        reqwest::header::CONNECTION,
        reqwest::header::HeaderValue::from_static(CONNECTION),
    );
    headers.insert(
        reqwest::header::CACHE_CONTROL,
        reqwest::header::HeaderValue::from_static(CACHE_CONTROL),
    );

    headers.insert(
        reqwest::header::HOST,
        reqwest::header::HeaderValue::from_static(HOST),
    );
    pub const BASE_URL: &str = "http://cdn.darkanddarker.com/Dark%20and%20Darker";

    let res = client
        .get(format!("{}/PatchFileList.txt", BASE_URL))
        .headers(headers)
        .send()
        .await?
        .bytes()
        .await?;
    let body_str = format!("path,sha256,size\n{}", String::from_utf8_lossy(&res));

    let mut r = csv::Reader::from_reader(body_str.as_bytes());

    let files: Vec<crate::download::Download> = r
        .records()
        .map(|record| {
            let record = record.unwrap();
            let file_path = record[0].to_string();
            let file_name = file_path.split('\\').last().unwrap().to_string();
            let uri = file_path.replace('\\', "/");
            let sha256 = record[1].to_string().to_lowercase();
            let size = record[2].parse::<u64>().unwrap();
            let full_path = Path::new(&path).join(&uri[1..]);

            crate::download::Download {
                path: file_path.clone(),
                linux_path: full_path.as_os_str().to_str().unwrap().to_string(),
                url: format!(
                    "http://cdn.darkanddarker.com/Dark%20and%20Darker/Patch{}",
                    uri
                ),
                file_name,
                sha256,
                size,
                real_file_name: None,
            }
        })
        .collect();

    Ok(files)
}

pub async fn update_game(args: crate::Args) -> Result<(), Box<dyn std::error::Error>> {
    let semaphore = Arc::new(tokio::sync::Semaphore::new(args.threads));

    let mut join_handles = Vec::new();
    let multi_progress = MultiProgress::new();

    for u in crate::game::get_game_urls(args.game_path).await? {
        let semaphore_permit = crate::download::acquire_semaphore_permit(semaphore.clone()).await;

        let progress_bar = multi_progress.add(ProgressBar::new(u.size));
        let sty = ProgressStyle::default_bar()
            .template(
                    format!(
                        "{} : {}",
                        "{spinner:.green} [{elapsed_precise}] │{bar:40.blue/yellow}│ {bytes}/{total_bytes} ({bytes_per_sec}, {eta}) {msg}",u.file_name.bold(), 
                    )
                    .as_str(),
                )?
            .progress_chars("▓▒░");

        progress_bar.set_style(sty);

        let download_task = tokio::spawn(crate::download::download_file(
            u,
            semaphore_permit,
            args.speed,
            progress_bar.clone(),
        ));
        join_handles.push(download_task);
    }

    crate::download::wait_for_tasks_completion(join_handles).await;
    println!("\n\n{}", "Dark and Darker up to date\n\n".green());

    Ok(())
}
