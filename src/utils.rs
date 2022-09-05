use std::path::{Path, PathBuf};
use std::fs;
use filetime::FileTime;

pub fn get_prj_dir() -> Option<PathBuf> {
    let mut prj_dir = home::home_dir()?;
    prj_dir.push("src/rs");
    return Some(prj_dir);
}

pub fn get_rust_bin_dir() -> PathBuf {
    return home::home_dir().unwrap().join(".cargo/bin");
}

pub fn get_bin_cache_dir() -> PathBuf {
    get_rust_bin_dir().join(".rsshim/bin_cache")
}

pub fn get_build_target_dir<P: AsRef<Path>>(prj_dir: P) -> PathBuf {
    let prj_dir_str = prj_dir.as_ref().to_str().unwrap();
    let hash = sha256::digest(prj_dir_str);
    let hash_short = &hash[..7];
    get_rust_bin_dir().join(format!(".rsshim/target-{}", &hash_short))
}

fn newer_source_exists_sub(bin_mtime: &FileTime, bin_source_dir: &Path) -> bool {
    // dbg!(bin_source_dir);
    for entry in bin_source_dir.read_dir().unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            if newer_source_exists_sub(bin_mtime, &path) {
                return true;
            }
        }
        if path.is_file() {
            let metadata = fs::metadata(&path).unwrap();
            let file_mtime = FileTime::from_last_modification_time(&metadata);
            if file_mtime.gt(bin_mtime) {
                return true;
            }
        }
    }
    false
}

pub fn newer_source_exists<P: AsRef<Path>, Q: AsRef<Path>>(cached_bin: P, bin_source_dir: Q) -> bool {
    let metadata = fs::metadata(&cached_bin).unwrap();
    let bin_mtime = FileTime::from_last_modification_time(&metadata);
    newer_source_exists_sub(&bin_mtime, bin_source_dir.as_ref())
}

#[cfg(test)]
mod tests;
