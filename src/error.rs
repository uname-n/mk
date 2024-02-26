use termion::color;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RError {
    #[error(
        "{}mk {}command not found in config{}",
        color::Fg(color::LightBlack),
        color::Fg(color::Red),
        color::Fg(color::Reset)
    )]
    TaskNotFound,
    #[error("{}mk:: {}failed to start task: {:?}{}", color::Fg(color::LightBlack), color::Fg(color::Red), .0, color::Fg(color::Reset))]
    TaskStartError(Vec<String>),
}
