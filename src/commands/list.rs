use walkdir::WalkDir;
use std::path::PathBuf;
use anyhow::Result;
//列出目录下所有文件
pub(crate) fn list_files(path: &PathBuf) -> Result<()> {
    println!("正在列出目录 {}", path.display());
    for entry in WalkDir::new(path).max_depth(1) {
        let entry = entry?;
        println!("- {}", entry.path().display());
    }
    Ok(())
}