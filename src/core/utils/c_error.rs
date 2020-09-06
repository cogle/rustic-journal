#[derive(Debug)]
pub struct CError {
    pub message: String,
    pub error_code: i32,
}

pub fn check_c_error_code(error_code: i32) ->Result<(), CError> {
    if error_code == 0 {
        return Ok(());
    }

    let test = CError {
        message: "".to_string(),
        error_code: 1,
    };

    return Err(test);
}