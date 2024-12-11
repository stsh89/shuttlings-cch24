use eyre::Report;

#[derive(Debug, thiserror::Error)]
#[error("{report}")]
pub struct Error {
    kind: ErrorKind,
    report: Report,
}

#[derive(Debug)]
enum ErrorKind {
    CorruptedDataFormat,
    MissingKeyword,
    RateLimited,
}

impl Error {
    pub fn is_rate_limited(&self) -> bool {
        matches!(self.kind, ErrorKind::RateLimited)
    }

    pub fn is_corrupted_data_format(&self) -> bool {
        matches!(self.kind, ErrorKind::CorruptedDataFormat)
    }

    pub fn is_missing_keyword(&self) -> bool {
        matches!(self.kind, ErrorKind::MissingKeyword)
    }

    pub fn corrupted_data_format(report: Report) -> Self {
        Self {
            kind: ErrorKind::CorruptedDataFormat,
            report,
        }
    }

    pub fn missing_keyword() -> Self {
        Self {
            kind: ErrorKind::MissingKeyword,
            report: Report::msg(""),
        }
    }

    pub fn rate_limited() -> Self {
        Self {
            kind: ErrorKind::RateLimited,
            report: Report::msg(""),
        }
    }

    pub fn report(self) -> Report {
        self.report
    }
}
