use eyre::Report;

pub struct Error {
    kind: ErrorKind,
    report: Report,
}

enum ErrorKind {
    CorruptedDataFormat,
    MissingKeyword,
}

impl Error {
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

    pub fn report(self) -> Report {
        self.report
    }
}
