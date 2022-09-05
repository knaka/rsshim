use std::fs;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use home::home_dir;

#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
struct RsshimToml {
    pub project: Option<Vec<ProjectToml>>,
}

#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
struct ProjectToml {
    pub directory: PathBuf,
}

#[derive(Debug)]
pub struct ProjectCluster {
    pub projects: Vec<Project>,
}

#[derive(Debug)]
pub struct Project {
    pub directory: PathBuf,
}

impl ProjectCluster {
    pub fn get_project_dir(&self) -> PathBuf {
        self.projects[0].directory.clone()
    }
}

pub fn unmarshal_config_file<P: AsRef<Path>>(user_config_dir: P) -> ProjectCluster {
    create_dir_all(&user_config_dir).expect("Err 56e23d9");
    let config_file_path = user_config_dir.as_ref().join("rsshim.toml");
    if !config_file_path.exists() {
        let mut config_file = File::create(&config_file_path).expect("Err 766e9f3");
        config_file.write_all(include_bytes!("rsshim-default.toml")).expect("Err 97eb2f7");
    }
    let rsshim_content = fs::read_to_string(&config_file_path).expect("Err afff349");
    let rsshim_toml: RsshimToml = toml::from_str(&rsshim_content).expect("Err dd549fa");
    let mut config = ProjectCluster {
        projects: vec!()
    };
    for project in rsshim_toml.project.expect("None 644fd67") {
        println!("{}", project.directory.to_str().expect("Err 15f5727"));
        let mut directory = project.directory.clone();
        if directory.starts_with("$HOME") {
            directory = home_dir().expect("None c06bba9").join(
                directory.strip_prefix("$HOME").expect("Err 6b172ff")
            );
        }
        config.projects.push(
            Project {
                directory,
            }
        );
    }
    config
}

#[cfg(test)]
mod tests;
