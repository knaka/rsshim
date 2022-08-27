use std::path::{Path, PathBuf};
use std::process::{Command, exit, Stdio};
use std::{env, fs, io, thread};
use std::io::{BufRead, Write};
use std::os::unix::prelude::CommandExt;

pub fn exec_cached_bin_sub(bin_cache_dir: &Path, prj_dir: &Path, should_exec: bool, arg0: &Path, args: &[String]) -> () {
    // exec(2) してしまうと、flush() したところで println! や dbg! の出力がどこかへ行ってしまう。なんでだろう
    // dbg!(arg0);
    // println!("{:?}", &args);
    let bin_name = arg0.file_name().expect("None 0efbb78").to_str().expect("None 2574b81");
    let cached_bin = bin_cache_dir.join(&bin_name);
    let bin_source_dir = get_bin_source_dir(&prj_dir, &bin_name);
    if !cached_bin.exists() || crate::utils::newer_source_exists(&cached_bin, &bin_source_dir) {
        // todo: target がテストのたびに生成されてしまうの直す
        let build_target_dir = crate::utils::get_build_target_dir(&prj_dir);
        let mut process = Command::new("cargo")
            .current_dir(&prj_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .arg("--verbose")
            .arg("build")
            .arg("--target-dir")
            .arg(&build_target_dir)
            .arg("--bin")
            .arg(&bin_name)
            .spawn()
            .expect("Err 7c1a53f")
        ;
        let stdout = io::BufReader::new(process.stdout.take().unwrap());
        let handle_stdout = thread::spawn(move || {
            for line in stdout.lines() {
                let line = line.expect("Err 5041f81");
                eprintln!("cargo stdout: {}", line);
            }
        });
        let stderr = io::BufReader::new(process.stderr.take().unwrap());
        let handle_stderr = thread::spawn(move || {
            for line in stderr.lines() {
                let line = line.expect("Err 14ba5aa");
                eprintln!("cargo stderr: {}", line);
            }
        });
        handle_stdout.join().expect("Err cb20084");
        handle_stderr.join().expect("Err dad1cc8");
        let mut built_bin_path = build_target_dir.join("debug");
        built_bin_path.push(&bin_name);
        fs::create_dir_all(&bin_cache_dir).expect("Err a4b39a7");
        fs::copy(&built_bin_path, &cached_bin).expect("Err 8e93053");    }
    let mut cmd = Command::new(&cached_bin);
    cmd.args(args);
    io::stdout().flush().unwrap();
    io::stderr().flush().unwrap();
    if should_exec {
        cmd.exec();
        exit(1);
    } else {
        let output = cmd.output().unwrap();
        let s = std::str::from_utf8(&output.stdout).unwrap();
        println!("d1: {}", &s);
        let s = std::str::from_utf8(&output.stderr).unwrap();
        println!("d2: {}", &s);
    }
}

pub fn exec_cached_bin() {
    let mut args = env::args();
    let arg0 = args.next().unwrap();
    let arg0 = Path::new(&arg0);
    let args_rest: Vec<String> = args.collect();
    let prj_dir = crate::utils::get_prj_dir().unwrap();
    let prj_dir = prj_dir.as_path();
    exec_cached_bin_sub(crate::utils::get_bin_cache_dir().as_path(), &prj_dir, true, arg0, args_rest.as_ref());
    exit(1);
}

fn get_bin_source_dir(prj_dir: &Path, bin_name: &str) -> PathBuf {
    prj_dir.join(format!("src/{}", bin_name))
}

#[cfg(test)]
mod tests;
