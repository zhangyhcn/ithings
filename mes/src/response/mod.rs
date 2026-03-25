use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
    pub code: u16,
    pub message: String,
    pub data: T,
}

impl<T> Response<T> {
    pub fn success(data: T) -> Self {
        Response {
            code: 200,
            message: "success".to_string(),
            data,
        }
    }

    pub fn error(code: u16, message: String) -> Response<()> {
        Response {
            code,
            message,
            data: (),
        }
    }
}
