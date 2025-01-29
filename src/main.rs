use std::os::unix::net::SocketAddr;

use anyhow::{Ok, Result};
use axum::{
    routing::{get, post},
    Router,
};
use clap::Parser;
use commands::{
    list,
    rename::{self, rename_files},
    search, stat, Cli, Commands,
};

mod commands;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        //执行cli命令
        handle_cli_command(command)?;
    } else {
        start_http_server().await?;
    }
    Ok(())
}

//处理cli命令
fn handle_cli_command(command: Commands) -> Result<()> {
    match command {
        Commands::List { path } => list::list_files(&path),
        Commands::Stat { path } => stat::stat_files(&path),
        Commands::Rename { path, prefix } => rename::rename_files(&path, &prefix),
        Commands::Search {
            path,
            keyword,
            content,
            recursive,
            case_sensitive,
        } => search::search(&path, &keyword, content, recursive, case_sensitive),
    }
}

//启动http服务
async fn start_http_server() -> Result<()> {
    let app = Router::new()
        .route("/list/:path", get(handle_list))
        .route("/stat/:path", get(handle_stat))
        .route("/rename/:path/:prefix", post(handle_rename))
        .route("/search/:path", post(handle_search));

    let addr = SocketAddr::from(([0.0.0.0], 3000));
    axum::serve(
        axum::Server::bind(&addr),
        app.into_make_service()).await?;
    
    Ok(())
}
