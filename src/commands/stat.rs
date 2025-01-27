use std::{collections::HashMap, path::PathBuf};
use anyhow::Result;
use walkdir::WalkDir;

//统计文件类型
pub(crate) fn stat_files(path: &PathBuf) -> Result<()> {
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