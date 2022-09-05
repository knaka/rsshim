use std::path::Path;
use std::{env, fs};

fn update_shims_sub<P: AsRef<Path>>(bin_dir: P, prj_dir: P) -> () {
    let mut bin_names = Vec::<String>::new();
    let glob_pattern = prj_dir.as_ref().join("src/bin/*");
    for glob_result in glob::glob(glob_pattern.to_str()
        .expect("None b5e174c"))
        .expect("Err ff0dc48")
    {
        let bin_abs_path = glob_result.expect("None 0f8028e");
        bin_names.push(
            bin_abs_path
                .file_stem()
                .expect("None 6581b94")
                .to_os_string()
                .into_string()
                .expect("Err a2003e2")
        );
    }
    // dbg!(bin_names);
    for bin_name in bin_names {
        let link_path = bin_dir.as_ref().join(&bin_name);
        if link_path.exists() {
            fs::remove_file(&link_path).expect("Err 9376fcc");
        }
        std::os::unix::fs::symlink(env::current_exe().unwrap(), link_path).expect("Err 8cb8776");
    }
}

pub fn update_shims() -> () {
    update_shims_sub(
        crate::utils::get_rust_bin_dir(),
        crate::utils::get_prj_dir()
            .expect("None ddfd19c"));
    return ();
}

#[cfg(test)]
mod tests;

