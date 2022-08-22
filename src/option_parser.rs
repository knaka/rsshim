use clap::{Parser, Args, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Options {
    #[clap(short, long)]
    /// Verbosity
    pub verbose: bool,
    #[clap(subcommand)]
    pub command: SubCommand,
}

#[derive(Args)]
pub struct InstallOptions {
    /// Doc comment
    field: bool,
}

#[derive(Subcommand)]
pub enum SubCommand {
    /// Install
    Install(InstallOptions),
    /// Reinstall
    Reinstall(InstallOptions),
}
