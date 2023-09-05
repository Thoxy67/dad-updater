use clap::Parser;
use colored::Colorize;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::sync::Arc;

mod download;
mod game;
mod launcher;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, env = "DAD_PATH")]
    game_path: String,
    #[arg(short, long, env = "BLACKSMITH_PATH")]
    launcher_path: Option<String>,
    #[arg(short, long, env = "DAD_DOWNLOAD_SPEED", default_value_t = 0)]
    speed: u64,
    #[arg(short, long, env = "DAD_THREADS", default_value_t = 10)]
    threads: usize,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let semaphore = Arc::new(tokio::sync::Semaphore::new(args.threads));

    match args.launcher_path {
        Some(p) => {
            let join_handles = Vec::new();

            let multi_progress = MultiProgress::new();
            for u in crate::launcher::get_launcher_url(p).await? {
                let mut join_handles = Vec::new();

                let multi_progress = MultiProgress::new();
                let semaphore_permit =
                    crate::download::acquire_semaphore_permit(semaphore.clone()).await;

                // Create a new ProgressBar for each download task
                let progress_bar = multi_progress.add(ProgressBar::new(u.size));
                let sty = ProgressStyle::with_template(
                    format!(
                        "{} {}",
                        u.file_name, "{spinner:.green} [{elapsed_precise}] │{wide_bar:.blue/magenta}│ {bytes}/{total_bytes} ({bytes_per_sec}, {eta}) {msg}",
                    )
                    .as_str(),
                )
                .unwrap()
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
            let _ = multi_progress.clear();
            drop(multi_progress);

            println!("\n\n{}", "Blacksmith Launcher up to date".green());
        }
        None => (),
    };
    let mut join_handles = Vec::new();

    let multi_progress = MultiProgress::new();

    for u in crate::game::get_game_urls(args.game_path).await? {
        let semaphore_permit = crate::download::acquire_semaphore_permit(semaphore.clone()).await;

        // Create a new ProgressBar for each download task
        let progress_bar = multi_progress.add(ProgressBar::new(u.size));
        let sty = ProgressStyle::with_template(
            format!(
                "{} {}",
                u.file_name, "{spinner:.green} [{elapsed_precise}] │{wide_bar:.blue/magenta}│ {bytes}/{total_bytes} ({bytes_per_sec}, {eta}) {msg}",
            )
            .as_str(),
        )
        .unwrap()
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
    let _ = multi_progress.clear();
    drop(multi_progress);

    println!("\n\n{}", "Dark and Darker up to date".green());

    Ok(())
}
