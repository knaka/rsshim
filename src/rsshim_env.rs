use std::path::{Path, PathBuf};
use std::{env, fs};
use crate::cargo_toml::CargoToml;

fn update_env_sub<P: AsRef<Path>>(bin_dir: P, prj_dir: P) -> () {
    let mut bin_rel_paths = Vec::<PathBuf>::new();
    let glob_pattern = prj_dir.as_ref().join("src/*/bin/main.rs");
    for glob_result in glob::glob(glob_pattern.to_str().unwrap()).unwrap() {
        let bin_abs_path = glob_result.unwrap();
        let bin_rel_path = bin_abs_path.strip_prefix(prj_dir.as_ref()).unwrap();
        bin_rel_paths.push(PathBuf::from(bin_rel_path));
    }
    let cargo_toml_path = prj_dir.as_ref().join("Cargo.toml");
    let cargo_toml_content = fs::read_to_string(&cargo_toml_path).unwrap();
    let cargo_toml: CargoToml = toml::from_str(&cargo_toml_content).unwrap();
    let cargo_toml_bin_paths: Vec<PathBuf> = if let Some(bin_array) = cargo_toml.bin {
        bin_array.iter().map(|bin| bin.path.clone()).collect()
    } else {
        vec![]
    };
    let cargo_toml_missing_bin_paths: Vec<&PathBuf> = bin_rel_paths.iter().filter(|path_buf|
        !cargo_toml_bin_paths.contains(*path_buf)
    ).collect();
    if !cargo_toml_missing_bin_paths.is_empty() {
        update_cargo_toml(&cargo_toml_path, &cargo_toml_content, &cargo_toml_missing_bin_paths);
    }
    for bin_rel_path in bin_rel_paths {
        let name = crate::utils::extract_bin_name_from_path(&bin_rel_path);
        // ln -sf std::env::current_exe $cargo_bin_dir/$name
        let link_path = bin_dir.as_ref().join(&name);
        if link_path.exists() {
            fs::remove_file(&link_path).expect("Err");
        }
        std::os::unix::fs::symlink(env::current_exe().unwrap(), link_path).expect("Err");
    }
}

pub fn update_env() -> () {
    update_env_sub(crate::utils::get_rust_bin_dir(), crate::utils::get_prj_dir().unwrap());
    return ();
}

fn update_cargo_toml(
    cargo_toml_path: &PathBuf,
    cargo_toml_string: &str,
    cargo_toml_missing_bin_paths: &Vec<&PathBuf>,
) {
    let mut doc = cargo_toml_string.parse::<toml_edit::Document>().expect("invalid doc");
    if !doc.contains_key("bin") {
        doc["bin"] = toml_edit::array();
    }
    let bin_array = doc["bin"].as_array_of_tables_mut().unwrap();
    for path in cargo_toml_missing_bin_paths {
        let mut bin = toml_edit::Table::new();
        let path_str = path.to_str().unwrap();
        let name = crate::utils::extract_bin_name_from_path(&path);
        bin["name"] = toml_edit::value(name);
        bin["path"] = toml_edit::value(path_str);
        bin_array.push(bin);
    }
    fs::write(&cargo_toml_path, doc.to_string()).expect("Unable to write file");
}

#[cfg(test)]
mod tests;

