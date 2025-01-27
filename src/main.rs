use std::path::PathBuf;

use anyhow::{Ok, Result};
use clap::{Parser, Subcommand};
use walkdir::WalkDir;


//定义命令行参数结构
#[derive(Parser)]
#[command(name = "rfm")]
#[command(about = "Rust 文件管理器", version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

//子命令枚举
#[derive(Subcommand)]
enum Commands {
    ///列出目录内容
    List {
        #[arg(short, long, default_value = ".")]
        path: PathBuf,
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::List { path } => list_files(&path)?,
        _ => unimplemented!()
    }
    Ok(())
}

//列出目录下所有文件
fn list_files(path: &PathBuf) -> Result<()> {
    println!("正在列出目录 {}", path.display());
    for entry in WalkDir::new(path).max_depth(1) {
        let entry = entry?;
        println!("- {}", entry.path().display());
    }
    Ok(())
}