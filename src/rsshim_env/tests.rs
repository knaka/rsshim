use super::*;
use crate::tests::{module_root_dir, TestEnv};

#[test]
fn it_deserialize_cargo_toml() {
    let mut cargo_toml_path = module_root_dir();
    cargo_toml_path.push("test_data/install_basic/prj/Cargo.toml");
    let toml_content = fs::read_to_string(&cargo_toml_path).unwrap();
    let cargo_toml: CargoToml = toml::from_str(&toml_content).unwrap();
    dbg!("#Parsed TOML:\n{:#?}", &cargo_toml);
}
