// search.rs

use regex::Regex;
use std::fs;
use std::path::Path;

pub fn find<P: AsRef<Path>>(
    root: P,
    regex: &Regex,
    verbose: bool,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut matches = Vec::new();
    walk_tree(root.as_ref(), regex, verbose, &mut matches)?;
    Ok(matches)
}

pub fn walk_tree(
    dir: &Path,
    regex: &Regex,
    verbose: bool,
    matches: &mut Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                walk_tree(&path, regex, verbose, matches)?;
            } else if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                if regex.is_match(filename) {
                    matches.push(path.to_string_lossy().to_string());
                }
            }
            if verbose {
                println!("遍历文件: {}", path.to_string_lossy());
            }
        }
    }
    Ok(())
}
