use colored::Colorize;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use std::{path::Path, sync::Arc};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub versioncode: i64,
    pub version: String,
    pub baseuri: String,
    pub force_update: bool,
    pub files: Vec<File>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub name: String,
    pub real_file_name: Option<String>,
    pub hash: String,
    pub size: u64,
    pub dir: Option<String>,
}

pub async fn get_launcher_url(
    path: String,
) -> core::result::Result<Vec<crate::download::Download>, Box<dyn std::error::Error>> {
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
        .get("http://cdn.darkanddarker.com/launcher/launcherinfo.json")
        .headers(headers)
        .send()
        .await?
        .json::<Root>()
        .await?;

    let mut files = Vec::new();
    for file in res.files {
        let dir = match file.dir {
            Some(dir) => dir,
            None => "".to_string(),
        };
        let full_path = Path::new(&path).join(&dir).join(&file.name);
        let url = format!(
            "http://cdn.darkanddarker.com/launcher/{}/{}",
            dir.clone(),
            file.name,
        );
        let sha256 = file.hash.to_lowercase();
        let size = file.size;
        if file.real_file_name != None {
            let full_path2 = Path::new(&path)
                .join(&dir)
                .join(&file.real_file_name.clone().unwrap());
            files.push(crate::download::Download {
                path: full_path2.as_os_str().to_str().unwrap().to_string(),
                linux_path: full_path2.as_os_str().to_str().unwrap().to_string(),
                url: url.clone(),
                file_name: file.real_file_name.unwrap(),
                sha256: sha256.clone(),
                size: size.clone(),
            });
        }
        files.push(crate::download::Download {
            path: full_path.as_os_str().to_str().unwrap().to_string(),
            linux_path: full_path.as_os_str().to_str().unwrap().to_string(),
            url,
            file_name: file.name,
            sha256,
            size,
        });
    }

    Ok(files)
}

pub async fn update_launcher(
    p: String,
    args: crate::Args,
) -> Result<(), Box<dyn std::error::Error>> {
    let semaphore = Arc::new(tokio::sync::Semaphore::new(args.threads));

    let mut join_handles = Vec::new();
    let multi_progress = MultiProgress::new();

    for u in crate::launcher::get_launcher_url(p).await? {
        let semaphore_permit = crate::download::acquire_semaphore_permit(semaphore.clone()).await;

        let progress_bar = multi_progress.add(ProgressBar::new(u.size));
        let sty = ProgressStyle::default_bar()
                .template(
                    format!(
                        "{} : {}",
                         "{spinner:.green} [{elapsed_precise}] │{bar:40.yellow/red}│ {bytes}/{total_bytes} ({bytes_per_sec}, {eta}) {msg}", u.file_name.bold(),
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
    println!("\n\n{}", "Blacksmith Launcher up to date\n\n".green());
    Ok(())
}
