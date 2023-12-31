use clap::Parser;
use colored::Colorize;
mod config;
mod download;
mod game;
mod launcher;
mod update;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, env = "DAD_PATH")]
    game_path: String,
    #[arg(short, long, env = "BLACKSMITH_PATH")]
    launcher_path: Option<String>,
    #[arg(short, long, env = "DAD_DOWNLOAD_SPEED", default_value_t = 0)]
    speed: u64,
    #[arg(short, long, env = "DAD_THREADS", default_value_t = 8)]
    threads: usize,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let title = r#"
     .___           .___                         .___       __                
   __| _/____     __| _/         __ ________   __| _/____ _/  |_  ___________ 
  / __ |\__  \   / __ |  ______ |  |  \____ \ / __ |\__  \\   __\/ __ \_  __ \
 / /_/ | / __ \_/ /_/ | /_____/ |  |  /  |_> > /_/ | / __ \|  | \  ___/|  | \/
 \____ |(____  /\____ |         |____/|   __/\____ |(____  /__|  \___  >__|   
      \/     \/      \/               |__|        \/     \/          \/      
"#
    .bold();
    println!("{}\n", title);

    let args = Args::parse();

    crate::update::update(args).await?;
    Ok(())
}
