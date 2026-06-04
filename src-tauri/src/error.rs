use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum AppError {
    Fetch(String),
    Config(String),
    Io(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fetch(s) | Self::Config(s) | Self::Io(s) => write!(f, "{s}"),
        }
    }
}

impl From<crate::fetcher::FetchError> for AppError {
    fn from(e: crate::fetcher::FetchError) -> Self {
        Self::Fetch(e.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        Self::Config(e.to_string())
    }
}
