use std::path::PathBuf;
use anyhow::Result;
use walkdir::WalkDir;
use rayon::prelude::*;

//重命名文件
pub(crate) fn rename_files(path: &PathBuf, prefix: &str) -> Result<()> {
    let files: Vec<_> = WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .collect();
    files.par_iter().for_each(|entry| {
        let old_path = entry.path();
        let file_name = old_path.file_name().unwrap().to_str().unwrap();
        let new_name = format!("{}{}", prefix, file_name);
        let new_path = old_path.with_file_name(new_name);

        if let Err(e) = std::fs::rename(old_path, new_path) {
            eprintln!("重命名失败: {}", e);
        }
    });
    Ok(())
}