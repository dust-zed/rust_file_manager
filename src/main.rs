use std::{collections::HashMap, path::PathBuf};

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
    },
    ///按类型统计文件
    Stat {
        #[arg(short, long, default_value= ".")]
        path: PathBuf,
    },
    ///批量重命名文件(添加前缀)
    Rename {
        #[arg(short, long, default_value = ".")]
        path: PathBuf,
        #[arg(short, long)]
        prefix: String,
    } 
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::List { path } => list_files(&path)?,
        Commands::Stat { path } => stat_files(&path)?,
        Commands::Rename { path, prefix } => rename_files(&path, &prefix)?,
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

//统计文件类型
fn stat_files(path: &PathBuf) -> Result<()> {
    let mut counts  = HashMap::new();

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let ext = entry
                .path()
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("unknown")
                .to_string();
            *counts.entry(ext).or_insert(0) += 1;    
        }
    }

    for (ext, count) in counts {
        println!("{} : {}个", ext, count);
    }
    Ok(())
}

//重命名文件
fn rename_files(path: &PathBuf, prefix: &str) -> Result<()> {
    let files: Vec<_> = WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .collect();
    for file in files {
        let old_path = file.path();
        let file_name = old_path.file_name().unwrap().to_str().unwrap();
        let new_name = format!("{}{}", prefix, file_name);
        let new_path = old_path.with_file_name(new_name);

        if let Err(e) = std::fs::rename(old_path, new_path) {
            eprintln!("重命名失败: {}", e);
        }
    }
    Ok(())
}