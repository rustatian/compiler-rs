use std::fmt::{Display, Formatter};

pub(crate) struct Error {
    pub code: usize,
    pub message: String,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let err_msg = match self.code {
            1 => "Unexpected",
            _ => "Undefined",
        };
        write!(f, "{}", err_msg)
    }
}
