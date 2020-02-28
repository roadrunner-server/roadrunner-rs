use clap::{App, AppSettings, SubCommand};

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
            println!("wooooow");
        }

        _ => unreachable!(),
    }
}
