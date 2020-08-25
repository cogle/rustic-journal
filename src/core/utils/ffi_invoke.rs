#[derive(Debug)]
pub struct CError {
    pub message: String,
    pub error_code: c_int,
}

