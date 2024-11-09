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
}
