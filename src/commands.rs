use std::path::PathBuf;
use clap::{Parser, Subcommand};

pub mod list;
pub mod stat;
pub mod rename;
pub mod search;

//定义命令行参数结构
#[derive(Parser)]
#[command(name = "rfm")]
#[command(about = "Rust 文件管理器", version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

//子命令枚举
#[derive(Subcommand)]
pub(crate) enum Commands {
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
    },
    ///按内容和按文件名检索文件
    Search {
        #[arg(short, long, default_value = ".")]
        path: PathBuf,
        #[arg(short, long)]
        keyword: String,
        #[arg(long)]
        content: bool,
        #[arg(long, long)]
        recursive: bool,
        #[arg(long)]
        case_sensitive: bool,
    }
}