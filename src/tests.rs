use std::fs;
use tempdir::TempDir;
use std::path::{Path, PathBuf};

pub struct TestEnv {
    pub temp_dir: TempDir,
    pub bin_dir: PathBuf,
    pub prj_dir: PathBuf,
}

pub fn module_root_dir() -> PathBuf {
    let this_file = Path::new(file!());
    let test_data_dir = PathBuf::from(this_file.parent().unwrap());
    test_data_dir
}

impl TestEnv {
    pub fn new(test_data_name: &str) -> TestEnv {
        let temp_dir = TempDir::new("test").unwrap();
        let mut test_data_dir = module_root_dir();
        test_data_dir.push("test_data");
        test_data_dir.push(test_data_name);
        let copy_opts = fs_extra::dir::CopyOptions::new();
        fs_extra::dir::copy(
            test_data_dir.join("bin"), &temp_dir,
            &copy_opts,
        ).expect("Err");
        fs_extra::dir::copy(
            test_data_dir.join("prj"), &temp_dir,
            &copy_opts,
        ).expect("Err");
        TestEnv {
            bin_dir: temp_dir.path().join("bin"),
            prj_dir: temp_dir.path().join("prj"),
            temp_dir,
        }
    }

    pub fn cargo_toml(self: &Self) -> crate::cargo_toml::CargoToml {
        toml::from_str(&fs::read_to_string(self.prj_dir.join("Cargo.toml")).unwrap()).unwrap()
    }
}
