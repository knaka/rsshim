use clap::Parser;
use option_parser::{Options, SubCommand};
use std::env;
use std::path::Path;
use std::process::exit;

mod cargo_toml;
mod option_parser;
mod rsshim_env;
mod rsshim_exec;
mod utils;

fn called_as_rsshim_command() -> bool {
    let mut args = env::args();
    let arg0 = args.next().unwrap();
    let file_name = Path::new(&arg0).file_stem();
    if let Some(file_name) = file_name {
        file_name == "rsshim"
    } else {
        false
    }
}

fn main() {
    if called_as_rsshim_command() {
        let options = Options::parse();
        match options.command {
            SubCommand::Install(_) => rsshim_env::update_shims(),
            SubCommand::Reinstall(_) => rsshim_env::update_shims(),
        }
        return;
    }
    rsshim_exec::exec_cached_bin();
    exit(1);
}

#[cfg(test)]
mod tests;
