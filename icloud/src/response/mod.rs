use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

impl<T> Response<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            message: "success".to_string(),
            data: Some(data),
        }
    }

    pub fn error(code: i32, message: String) -> Self {
        Self {
            code,
            message,
            data: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PageResponse<T> {
    pub list: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
}

impl<T> PageResponse<T> {
    pub fn new(list: Vec<T>, total: i64, page: i64, page_size: i64) -> Self {
        Self {
            list,
            total,
            page,
            page_size,
        }
    }
}
