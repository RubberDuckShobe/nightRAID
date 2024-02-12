use clap::Command;
use tracing::debug;

use crate::ws::GameSession;

pub fn execute(session: &mut GameSession, line: &str) -> Result<(), ezsockets::Error> {
    debug!("Evaluating command: {}", line);
    let args = shlex::split(line).ok_or("Invalid quoting")?;
    let matches = cli().try_get_matches_from(args);
    if matches.is_err() {
        session.handle.text("error: invalid command\n\n")?;
        return Ok(());
    }

    match matches?.subcommand() {
        Some(("ping", _matches)) => {
            session.handle.text("pong\n\n")?;
            return Ok(());
        }
        Some(("quit", _matches)) => {
            session.handle.close(None)?;
            return Ok(());
        }
        Some((_, _)) => {
            session.handle.text("error: unknown command\n\n")?;
        }
        None => unreachable!("subcommand required"),
    }

    Ok(())
}

fn cli() -> Command {
    // strip out usage
    const PARSER_TEMPLATE: &str = "\
        {all-args}
    ";
    // strip out name/version
    const APPLET_TEMPLATE: &str = "\
        {about-with-newline}\n\
        {usage-heading}\n    {usage}\n\
        \n\
        {all-args}{after-help}\
    ";

    Command::new("repl")
        .multicall(true)
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand_value_name("APPLET")
        .subcommand_help_heading("APPLETS")
        .help_template(PARSER_TEMPLATE)
        .subcommand(
            Command::new("ping")
                .about("Ping and you shall be ponged")
                .help_template(APPLET_TEMPLATE),
        )
        .subcommand(
            Command::new("quit")
                .alias("exit")
                .about("Quit the REPL")
                .help_template(APPLET_TEMPLATE),
        )
}
