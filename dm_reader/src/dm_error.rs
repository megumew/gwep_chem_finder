#[derive(Debug)]
pub struct DMError {
    line: usize,
    message: String,
}

impl DMError {
    pub fn error(line: usize, message: String) -> DMError {
        DMError { line, message }
    }

    pub fn report(&self) {
        eprintln!("[Line {}] Error: {}", self.line, self.message);
    }
}
