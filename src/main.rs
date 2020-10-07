mod worker;

use clap::{App, AppSettings, SubCommand};
use std::error::Error;
use std::io::{Read, Write};
use std::process::{Command, Stdio};

static PANGRAM: &'static str = "the quick brown fox jumped the lazy dog\n";

fn main() {
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

fn run_process() {
    let process = match Command::new("/usr/bin/php")
        .arg("/home/valery/Projects/opensource/roadrunner-rr/psr-worker.php")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        // .current_dir("/home/valery/Projects/opensource/")
        .spawn()
    {
        Err(why) => panic!("couldn't spawn php: {}", why.to_string()),
        Ok(process) => process,
    };

    match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
        Err(why) => panic!("couldn't write to stdin: {}", why.description()),
        Ok(_) => println!("sent!"),
    }

    let mut s = String::new();

    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read from stdout: {}", why.description()),
        Ok(_) => println!("responded: {}", s),
    }
}
