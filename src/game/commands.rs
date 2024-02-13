use clap::{Command, CommandFactory, Parser, Subcommand};
use tracing::debug;

use crate::{
    errors::{IntoTextError, TextError},
    ws::GameSession,
}; // Import the IntoTextError trait

pub fn execute(session: &mut GameSession, line: &str) -> Result<(), TextError> {
    debug!("Evaluating command: {}", line);
    let args = shlex::split(line)
        .ok_or(anyhow::anyhow!("User messed up quotes"))
        .text_error("erroneous quotes")?;
    debug!("Parsed args: {:?}", args);
    let cli = Cli::try_parse_from(args).text_error("unknown command")?;

    match cli.command {
        Commands::Ping => {
            session.handle.text("pong\n\n")?;
        }
        Commands::Exit => {
            session.handle.text("Goodbye.")?;
            session.handle.close(None)?;
        }
        Commands::Login { token } => {
            debug!("Username: {:?}", token);
            session.handle.text("login\n\n")?;
        }
        Commands::Register => {
            session.handle.text("register\n\n")?;
        }
    }

    Ok(())
}

#[derive(Debug, Parser)]
#[command(multicall = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Ping,
    Exit,
    Login {
        /// Optional. You will be prompted if you don't provide it.
        #[arg(index = 0)]
        token: String,
    },
    Register,
}
