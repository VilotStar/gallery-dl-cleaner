use clap::Parser;

pub mod cli;
pub mod handlers;

use cli::Cli;
use handlers::{dotparty::DotPartyHandler, HandlerError, HandlerImpl};
use thiserror::Error;

// Example file name: e621_3650669_48a64dcda61894b2a05494204efcd113.jpg
// Folder structure:
// - site :3~
//   - id or whatever nyaaa~
//     - files wur enoywuou

#[derive(Debug, Error)]
pub enum MainError {
    #[error("Handler Error {0:#?}")]
    HandlerError(HandlerError),
}

fn main() -> Result<(), MainError> {
    let cli = Cli::parse();

    match cli.subcommand {
        cli::HandlerCommand::E621 => todo!(),
        cli::HandlerCommand::DotParty => {
            let handler = DotPartyHandler::new(&cli.path, cli.update.clone());

            if cli.update.as_ref().is_some() {
                handler
                    .update()
                    .map_err(|err| MainError::HandlerError(err))?;
            }

            handler
                .handle()
                .map_err(|err| MainError::HandlerError(err))?;
        }
    }

    Ok(())
}
