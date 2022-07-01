use std::io;
mod errors;
mod state;
mod worker;
mod payload;


fn main() -> Result<(), io::Error> {
    env_logger::init();
    // let cli = App::new(env!("CARGO_PKG_NAME"))
    //     .version(env!("CARGO_PKG_VERSION"))
    //     .author(env!("CARGO_PKG_AUTHORS"))
    //     .setting(AppSettings::DisableHelpSubcommand)
    //     .setting(AppSettings::SubcommandRequiredElseHelp)
    //     .setting(AppSettings::VersionlessSubcommands)
    //     .subcommand(SubCommand::with_name("serve").about("run RR"))
    //     .get_matches();
    //
    // match cli.subcommand() {
    //     ("serve", Some(matches)) => {
    //         run_process();
    //     }
    //
    //     _ => unreachable!(),
    // }

    let p = http::Plugin::new("127.0.0.1:7878")?;

    Ok(())
}

fn run_process() {}
