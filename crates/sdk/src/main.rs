#![warn(rust_2018_idioms)]
#![allow(dead_code)]
#![allow(unused_variables)]
mod errors;
mod state;
mod worker;
mod payload;

use clap::{App, AppSettings, SubCommand};

fn main() {
    env_logger::init();
    let cli = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .subcommand(SubCommand::with_name("serve").about("run RR"))
        .get_matches();

    match cli.subcommand() {
        ("serve", Some(matches)) => {
            run_process();
        }

        _ => unreachable!(),
    }
}

fn run_process() {}
