use std::io::Write;
use super::*;
use crate::tests::{module_root_dir, TestEnv};
use toml_edit::{Document, value};

#[test]
fn it_gets_bin_cache_dir() {
    let dir = get_bin_cache_dir();
    assert!(dir.ends_with(".cargo/bin/.rsshim/bin_cache"));
}

#[test]
fn it_checks_whther_newer_source_exists() {
    let test_env = TestEnv::new("install_basic");
    let sh = xshell::Shell::new().unwrap();
    let source_dir = test_env.prj_dir.join("src/foo/bin");
    let cmd = sh.cmd("/bin/sh")
        .arg("-c")
        .arg(format!("find {} -type f | xargs touch", source_dir.to_str().unwrap()));
    cmd.run().expect("");
    let metadata = fs::metadata(test_env.prj_dir.join("src/foo/bin/main.rs")).unwrap();
    let source_mtime = FileTime::from_last_modification_time(&metadata);
    let bin_cache = test_env.bin_dir.join(".rsshim/bin_cache/foo");
    fs_extra::dir::create_all(&bin_cache, true).expect("");

    let bin_mtime = FileTime::from_unix_time(source_mtime.unix_seconds() + 1, 0);
    filetime::set_file_mtime(&bin_cache, bin_mtime).expect("");
    assert!(!newer_source_exists(&bin_cache, &source_dir));

    let bin_mtime = FileTime::from_unix_time(source_mtime.unix_seconds() - 1, 0);
    filetime::set_file_mtime(&bin_cache, bin_mtime).expect("");
    assert!(newer_source_exists(&bin_cache, &source_dir));
}

#[test]
fn it_gets_build_target_dir() {
    let dir = get_build_target_dir("/tmp/prj");
    // $ echo -n /tmp/prj | shasum --algorithm 256 | perl -pe 's/^(.......).*/\1/'
    // 749ae27
    assert!(dir.ends_with(".cargo/bin/.rsshim/target-749ae27"));
}

#[test]
fn it_uses_toml_edit() {
    let toml_str = r#"
"hello" = 'toml!' # comment

# another comment

['a'.b]
"#;
    let mut doc = toml_str.parse::<Document>().expect("invalid doc");
    assert_eq!(doc.to_string(), toml_str);
    // let's add a new key/value pair inside a.b: c = {d = "hello"}
    doc["a"]["b"]["c"]["d"] = value("hello");
    doc["a"]["b"]["message"] = value("world");
    // autoformat inline table a.b.c: { d = "hello" }
    doc["a"]["b"]["c"].as_inline_table_mut().map(|t| t.fmt());
    let expected = r#"
"hello" = 'toml!' # comment

# another comment

['a'.b]
c = { d = "hello" }
message = "world"
"#;
    assert_eq!(doc.to_string(), expected);
}

#[test]
fn it_maps() {
    // let p = Path::new("/foo/bar/baz");
    // p.map
    let s: Option<String> = Some("hello".to_string());
    dbg!(&s);
    let s2 = s.map(|s| s.to_uppercase());
    dbg!(s2);

    let p = Path::new("/home/knaka/hoge");
    let s = p.to_str().unwrap().to_uppercase();
    let p2 = Path::new(&s);
    dbg!(&p2);
    dbg!(p.iter().count());
    let y: Vec<&str> = p.iter().map(|s| s.to_str().unwrap()).collect();
    dbg!(&y);
    println!("{}", y[0]);
    for x in p.iter() {
        dbg!(x);
    }
    let v: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<i32> = v.iter().map(|x| x + 1).collect();
    dbg!(&v2);
}

#[test]
fn it_returns_rust_bin_dir() {
    assert_eq!(get_rust_bin_dir().ends_with(".cargo/bin"), true);
}

#[test]
fn it_returns_prj_dir() {
    assert_eq!(get_prj_dir().expect("None").ends_with("src/rs"), true);
}

#[test]
fn it_reads_cargo_toml() {
    let mut cargo_toml_path = module_root_dir();
    cargo_toml_path.push("test_data/install_basic/prj/Cargo.toml");
    let toml_str: String = fs::read_to_string(&cargo_toml_path).unwrap();
    let val: toml::Value = toml::from_str(&toml_str).unwrap();
    dbg!(&val);
    let mut outfile = fs::File::create("/tmp/out.toml").unwrap();
    // write!(file, "{:?}", val.as_str()).unwrap();
    write!(outfile, "{}", &val).unwrap();
    outfile.flush().unwrap();
}
