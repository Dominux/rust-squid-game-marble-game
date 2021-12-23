use std::{fmt, error};

#[derive(Debug)]
pub struct ValidationError {
    pub detail: String,
}

impl ValidationError {
    pub fn new(detail: &str) -> ValidationError {
        ValidationError {
            detail: String::from(detail),
        }
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.detail)
    }
}

impl error::Error for ValidationError {
    fn description(&self) -> &str {
        &self.detail.as_str()
    }
}


#[derive(Debug)]
pub struct WrongStateError {
    pub detail: String,
}

impl WrongStateError {
    pub fn new(detail: &str) -> ValidationError {
        ValidationError {
            detail: String::from(detail),
        }
    }
}

impl fmt::Display for WrongStateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.detail)
    }
}

impl error::Error for WrongStateError {
    fn description(&self) -> &str {
        &self.detail.as_str()
    }
}

