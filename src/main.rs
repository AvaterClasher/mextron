use anyhow::{Context, Result};
use builder::{cache, utils, Worker};
use clap::{Parser, Subcommand};
use dev::server::Clients;
use directories::ProjectDirs;
use owo_colors::OwoColorize;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::{env, println};
use tokio::net::TcpListener;
use tokio::sync::Mutex;

mod builder;
mod create;
mod dev;
mod shared;

#[derive(Debug, Parser)]
#[command(name = "mextron")]
#[command(bin_name = "mextron")]
#[command(about = "A blazing fast static site generator", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Create new project
    #[command()]
    New {
        #[clap(required = true, help = "Project directory")]
        project_dir: PathBuf,

        #[clap(
            required = false,
            help = "Theme",
            default_value = "pico",
            short = 't',
            long = "theme"
        )]
        theme: String,
    },
    /// Start dev mode
    #[command()]
    Dev {
        #[clap(required = true, help = "Input directory")]
        input_dir: PathBuf,

        /// Watch for changes
        #[clap(short = 'w', long = "watch")]
        watch: bool,
    },
    /// Build the site
    #[command()]
    Build {
        #[clap(required = true, help = "Input directory")]
        input_dir: PathBuf,
    },
    /// Clean the site
    #[command()]
    Clean {},
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    let project_dirs = ProjectDirs::from("io/mextron", "github", "avaterclasher")
        .context("Failed to get project directories")?;
    let cache_dir = project_dirs.cache_dir().to_string_lossy().to_string();
    let cache = cache::Cache::new(cache_dir)?;

    match args.command {
        Commands::New { project_dir, theme } => {
            println!("{}...", "\n- Creating new project".bold());

            match create::project(&project_dir, &theme).await {
                Err(e) => {
                    println!("- {}", e.to_string().red().bold());
                }
                _ => {
                    // Create settings file
                    create::settings_file(&project_dir)?;

                    println!(
                        "- Project created in {}",
                        project_dir.display().blue().bold()
                    );
                }
            }
        }
        Commands::Dev { input_dir, watch } => {
            if utils::path_to_string(&input_dir)? == utils::path_to_string(&env::current_dir()?)? {
                println!(
                    "{}",
                    "\nSorry, you cannot use current directory as input directory as output is written to it!"
                        .red()
                        .bold()
                );

                return Ok(());
            }

            let worker = Worker::dev(&input_dir, Some(cache), true)?;
            let output_dir = worker.get_output_dir().to_string();
            let port = worker.get_settings().dev.port;
            let ws_port = worker.get_settings().dev.ws_port;

            // Trigger a build
            if let Err(e) = worker.build() {
                println!("- Build failed -> {}", e.to_string().red().bold());
            }

            // Start dev server
            tokio::task::spawn(dev::server::start(output_dir, port));

            if watch {
                let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

                tokio::spawn(dev::server::handle_file_changes(
                    input_dir,
                    worker,
                    clients.clone(),
                ));

                let addr = format!("0.0.0.0:{}", ws_port);
                let listener = TcpListener::bind(&addr).await?;

                while let Ok((stream, _)) = listener.accept().await {
                    tokio::spawn(dev::server::accept_connection(stream, clients.clone()));
                }
            }
        }
        Commands::Build { input_dir } => {
            let worker = Worker::prod(&input_dir)?;

            if let Err(e) = worker.build() {
                println!("- Build failed -> {}", e.to_string().red().bold());
            }
        }
        Commands::Clean {} => {
            cache.clean().context("Failed to clean cache")?;
        }
    }

    Ok(())
}
