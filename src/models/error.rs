use std::fmt;

pub struct Error {
    code: usize,
    message: String,
}

impl Error {
    pub fn from(msg: &str) -> Self {
        Self {
            code: 500,
            message: msg.to_string(),
        }
    }
}

impl Default for Error {
    fn default() -> Self {
        Self {
            code: 500,
            message: "fatal!!".to_string(),
        }
    }
}

// Different error messages according to AppError.code
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err_msg = match self.code {
            404 => "Sorry, Can not find the Page!",
            _ => "Sorry, something is wrong! Please Try Again!",
        };

        write!(f, "{}", err_msg)
    }
}

// A unique format for dubugging output
impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error {{ code: {}, message: {} }}",
            self.code, self.message
        )
    }
}
