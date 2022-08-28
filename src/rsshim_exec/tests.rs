use home::home_dir;
use super::*;
use crate::tests::TestEnv;

#[test]
fn it_executes_command() {
    let env = TestEnv::new("install_basic");
    let opts = [String::from("bar"), String::from("baz")];
    let home_dir = home_dir().expect("None 45f0e51");
    env::set_var(
        "PATH",
        format!(
            "/bin:/usr/bin:/usr/local/bin:{}/.cargo/bin",
            home_dir.to_str().expect("None 5e2c3d5")));
    exec_cached_bin_sub(
        &env.bin_dir.as_path(),
        &env.prj_dir.as_path(),
        false,
        Path::new("/dummy/foo"),
        &opts,
        Some(env.temp_dir.path().join("target"))
    );
}
