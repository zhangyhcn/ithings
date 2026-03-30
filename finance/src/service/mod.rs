pub mod account;
pub mod voucher;

pub use account::AccountService;
pub use voucher::VoucherService;
pub use voucher::CreateVoucherRequest;
pub use voucher::CreateVoucherItemRequest;

// 通用错误类型
#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sea_orm::DbErr),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Business logic error: {0}")]
    BusinessError(String),
}

pub type ServiceResult<T> = Result<T, ServiceError>;
