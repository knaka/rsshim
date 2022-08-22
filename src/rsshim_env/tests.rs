use super::*;
use crate::tests::{module_root_dir, TestEnv};

#[test]
fn it_updates_env() {
    let test_env = TestEnv::new("install_basic");
    let cargo_toml_before = test_env.cargo_toml();
    // bin table、実在 2 + 非実在 1, 非記載 1
    assert_eq!(cargo_toml_before.bin.unwrap().len(), 3);
    std::os::unix::fs::symlink(env::current_exe().unwrap(), test_env.bin_dir.join("foo")).expect("Err");
    // todo: 実在しない bin へのリンクの扱いについては後で考える
    assert_eq!(test_env.bin_dir.read_dir().unwrap().into_iter().count(), 1);
    update_env_sub(&test_env.bin_dir, &test_env.prj_dir);
    let cargo_toml_after = test_env.cargo_toml();
    // bin table、実在 3 + 非実在 1
    assert_eq!(cargo_toml_after.bin.unwrap().len(), 4);
    assert_eq!(test_env.bin_dir.read_dir().unwrap().into_iter().count(), 3);
}

#[test]
fn it_deserialize_cargo_toml() {
    let mut cargo_toml_path = module_root_dir();
    cargo_toml_path.push("test_data/install_basic/prj/Cargo.toml");
    let toml_content = fs::read_to_string(&cargo_toml_path).unwrap();
    let cargo_toml: CargoToml = toml::from_str(&toml_content).unwrap();
    dbg!("#Parsed TOML:\n{:#?}", &cargo_toml);
}
