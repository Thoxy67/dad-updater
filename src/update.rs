use crate::config::*;
use colored::Colorize;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::{path::Path, sync::Arc};

pub async fn update(args: crate::Args) -> Result<(), Box<dyn std::error::Error>> {
    let semaphore = Arc::new(tokio::sync::Semaphore::new(args.threads));
    let mut join_handles = Vec::new();
    let multi_progress = MultiProgress::new();

    let mut urls = crate::game::get_game_urls(args.game_path).await?;
    if let Some(p) = args.launcher_path.clone() {
        urls.append(&mut crate::launcher::get_launcher_urls(p).await?);
    }
    for u in urls {
        let semaphore_permit = crate::download::acquire_semaphore_permit(semaphore.clone()).await;

        let style = {
            if u.launcher_file {
                format!(
                        "{} : {}",
                         "{spinner:.green} [{elapsed_precise}] │{bar:40.yellow/red}│ {bytes}/{total_bytes} ({bytes_per_sec}, {eta}) {msg}", u.file_name.bold(),
                    )
            } else {
                format!(
                        "{} : {}",
                        "{spinner:.green} [{elapsed_precise}] │{bar:40.blue/yellow}│ {bytes}/{total_bytes} ({bytes_per_sec}, {eta}) {msg}",u.file_name.bold(), 
                    )
            }
        };

        let progress_bar = multi_progress.add(ProgressBar::new(u.size));
        let sty = ProgressStyle::default_bar()
            .template(&style)?
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
