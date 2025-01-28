/*!
* # 按名称搜索（不区分大小写）
* cargo run -- search --keyword "rust" --recursive

* # 按内容搜索（区分大小写）
* cargo run -- search --keyword "TODO" --content --case-sensitive
*/

use std::{fs::File, io::{BufRead, BufReader}, path::PathBuf};

use anyhow::{Ok, Result};
use walkdir::WalkDir;

pub fn search(
    path: &PathBuf,
    keyword: &str,
    content: bool,
    recursive: bool,
    case_sensitive: bool,
) -> Result<()> {
    let result = if content {
        search_by_content(path, keyword, recursive, case_sensitive)?
    } else {
        search_by_name(path, keyword, recursive, case_sensitive)?
    };
    println!("找到{}个匹配的文件:", result.len());
    for file in result {
        println!("- {}", file.display());
    }
    Ok(())
}

fn search_by_name(
    path: &PathBuf,
    keyword: &str,
    recursive: bool,
    case_sensitive: bool
) -> Result<Vec<PathBuf>> {
    let mut result = Vec::new();
    let walker = WalkDir::new(path).max_depth(if recursive { usize::MAX} else { 1 } );
    for entry in walker.into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let filename = entry.file_name().to_string_lossy();
            let target = if case_sensitive {
                filename.to_string()
            } else {
                filename.to_lowercase()
            };
            let keyword_check = if case_sensitive {
                keyword
            } else {
                &keyword.to_lowercase()
            };
            if target.contains(keyword_check) {
                result.push(entry.into_path());
            }
        }
    }
    Ok(result)
}

fn search_by_content(
    path: &PathBuf,
    keyword: &str,
    recursive: bool,
    case_sensitive: bool,
) -> Result<Vec<PathBuf>> {
    let mut results = Vec::new();
    let walker = WalkDir::new(path).max_depth(if recursive { usize::MAX } else { 1 });
    for entry in walker.into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let file = File::open(entry.path())?;
            let buf_reader = BufReader::new(file);
            for line in buf_reader.lines() {
                let line = match line {
                    Err(_) => break, //跳过无法处理的文件，包括权限不足，非utf_8编码文件
                    Result::Ok(line) => line
                };
                let line_check = if case_sensitive {
                    line.as_str()
                } else {
                    &line.to_lowercase()
                };
                let keyword_check = if case_sensitive {
                    keyword
                } else {
                    &keyword.to_lowercase()
                };
                if line_check.contains(keyword_check) {
                    results.push(entry.path().to_path_buf());
                    break;
                }
            }

        }
    }
    Ok(results)
}