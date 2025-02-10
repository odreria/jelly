use crate::errors::beetle_error::BeetleError;

trait Command {
    fn execute(&self) -> Result<(), BeetleError>;
}

struct DownloadPomCommand {
    dep: Depedency,
}

impl Command for DownloadPomCommand {
    fn execute(&self) -> Result<(), BeetleError> {

    }
}

