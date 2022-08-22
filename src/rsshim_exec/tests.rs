use super::*;
use crate::tests::TestEnv;

#[test]
fn it_executes_command() {
    let env = TestEnv::new("install_basic");
    let opts = [String::from("bar"), String::from("baz")];
    exec_cached_bin_sub(
        &env.bin_dir.as_path(),
        &env.prj_dir.as_path(),
        false,
        Path::new("/home/foo/.cargo/bin/foo"),
        &opts,
    );
}
