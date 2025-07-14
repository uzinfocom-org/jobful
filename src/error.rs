use owo_colors::OwoColorize;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, JobfulErrors>;

#[derive(Debug, Error)]
pub enum JobfulErrors {
    #[error("reqwest failed at someshit")]
    Reqwest(#[from] reqwest::Error),
    // #[error("data store disconnected")]
    // Disconnect(#[from] io::Error),
    // #[error("the data for key `{0}` is not available")]
    // Redaction(String),
    // #[error("invalid header (expected {expected:?}, found {found:?})")]
    // InvalidHeader { expected: String, found: String },
    #[error("bitch, where's da http client")]
    NoHTTPClient,
    #[error("nigga, no data or client or idk...")]
    MissingDependency,
    #[error("unknown jobless error")]
    Unknown,
}

pub fn beautiful_exit<T>(message: T) -> !
where
    T: AsRef<str>,
{
    eprintln!("{}: {}", "Error:".red(), message.as_ref());

    std::process::exit(1)
}
