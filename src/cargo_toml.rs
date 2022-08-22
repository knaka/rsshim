use std::path::PathBuf;

#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
pub struct CargoToml {
    pub package: Package,
    pub bin: Option<Vec<Bin>>,
}

#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
pub struct Package {
    name: String,
}

#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
pub struct Bin {
    name: String,
    pub path: PathBuf,
}
