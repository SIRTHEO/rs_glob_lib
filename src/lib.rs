pub mod glob {
    use std::{fs, path::{Path, PathBuf}};

    pub fn glob(pattern: &str, exclude_patterns: &[&str]) -> Result<Vec<std::path::PathBuf>, std::io::Error> {
        let path = Path::new(pattern);
        let paths: Vec<_> = collect_file_paths(path, exclude_patterns);

        if paths.is_empty() {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "No files found matching the glob pattern",
            ))
        } else {
            Ok(paths)
        }
    }

    fn collect_file_paths(folder_path: &Path, exclude_patterns: &[&str]) -> Vec<PathBuf> {
        let mut file_paths = Vec::new();
    
        if let Ok(entries) = fs::read_dir(folder_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();

                    if exclude_patterns.into_iter().any(|pattern| path.to_string_lossy().contains(pattern)) {
                        continue;
                    }

                    let normalized_path = normalize_path(&path);

                    if normalized_path.is_file() {
                        file_paths.push(normalized_path);
                    } else if normalized_path.is_dir() {
                        // Chiamata ricorsiva per le sottocartelle
                        file_paths.extend(collect_file_paths(&normalized_path, exclude_patterns));
                    }
                }
            }
        }
    
        file_paths
    }

    fn normalize_path(path: &Path) -> PathBuf {
        let path_str = path.to_string_lossy().replace("\\", "/");
        PathBuf::from(path_str)
    }
    
}
