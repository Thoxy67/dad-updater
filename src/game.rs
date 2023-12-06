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
                launcher_file: false,
            }
        })
        .collect();

    Ok(files)
}
