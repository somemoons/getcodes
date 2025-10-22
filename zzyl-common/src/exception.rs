use crate::error::{ZzylError, Result};

/// 业务异常
pub struct BusinessException {
    pub message: String,
    pub error_code: i32,
}

impl BusinessException {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
            error_code: 500,
        }
    }
    
    pub fn with_code(message: &str, error_code: i32) -> Self {
        Self {
            message: message.to_string(),
            error_code,
        }
    }
}

impl std::fmt::Display for BusinessException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BusinessException: {}", self.message)
    }
}

impl std::fmt::Debug for BusinessException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BusinessException {{ message: {}, error_code: {} }}", self.message, self.error_code)
    }
}

impl std::error::Error for BusinessException {}

impl From<BusinessException> for ZzylError {
    fn from(exception: BusinessException) -> Self {
        ZzylError::Business(exception.message)
    }
}

/// 参数异常
pub struct ParameterException {
    pub message: String,
}

impl ParameterException {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl std::fmt::Display for ParameterException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParameterException: {}", self.message)
    }
}

impl std::fmt::Debug for ParameterException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParameterException {{ message: {} }}", self.message)
    }
}

impl std::error::Error for ParameterException {}

impl From<ParameterException> for ZzylError {
    fn from(exception: ParameterException) -> Self {
        ZzylError::InvalidParameter(exception.message)
    }
}

/// 认证异常
pub struct AuthenticationException {
    pub message: String,
}

impl AuthenticationException {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl std::fmt::Display for AuthenticationException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AuthenticationException: {}", self.message)
    }
}

impl std::fmt::Debug for AuthenticationException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AuthenticationException {{ message: {} }}", self.message)
    }
}

impl std::error::Error for AuthenticationException {}

impl From<AuthenticationException> for ZzylError {
    fn from(exception: AuthenticationException) -> Self {
        ZzylError::Authentication(exception.message)
    }
}

/// 授权异常
pub struct AuthorizationException {
    pub message: String,
}

impl AuthorizationException {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl std::fmt::Display for AuthorizationException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AuthorizationException: {}", self.message)
    }
}

impl std::fmt::Debug for AuthorizationException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AuthorizationException {{ message: {} }}", self.message)
    }
}

impl std::error::Error for AuthorizationException {}

impl From<AuthorizationException> for ZzylError {
    fn from(exception: AuthorizationException) -> Self {
        ZzylError::Authorization(exception.message)
    }
}

/// 数据不存在异常
pub struct DataNotFoundException {
    pub message: String,
}

impl DataNotFoundException {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl std::fmt::Display for DataNotFoundException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DataNotFoundException: {}", self.message)
    }
}

impl std::fmt::Debug for DataNotFoundException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DataNotFoundException {{ message: {} }}", self.message)
    }
}

impl std::error::Error for DataNotFoundException {}

impl From<DataNotFoundException> for ZzylError {
    fn from(exception: DataNotFoundException) -> Self {
        ZzylError::NotFound(exception.message)
    }
}

/// 数据已存在异常
pub struct DataAlreadyExistsException {
    pub message: String,
}

impl DataAlreadyExistsException {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl std::fmt::Display for DataAlreadyExistsException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DataAlreadyExistsException: {}", self.message)
    }
}

impl std::fmt::Debug for DataAlreadyExistsException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DataAlreadyExistsException {{ message: {} }}", self.message)
    }
}

impl std::error::Error for DataAlreadyExistsException {}

impl From<DataAlreadyExistsException> for ZzylError {
    fn from(exception: DataAlreadyExistsException) -> Self {
        ZzylError::AlreadyExists(exception.message)
    }
}

/// 系统异常
pub struct SystemException {
    pub message: String,
}

impl SystemException {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl std::fmt::Display for SystemException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SystemException: {}", self.message)
    }
}

impl std::fmt::Debug for SystemException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SystemException {{ message: {} }}", self.message)
    }
}

impl std::error::Error for SystemException {}

impl From<SystemException> for ZzylError {
    fn from(exception: SystemException) -> Self {
        ZzylError::System(exception.message)
    }
}

/// 文件操作异常
pub struct FileOperationException {
    pub message: String,
}

impl FileOperationException {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl std::fmt::Display for FileOperationException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FileOperationException: {}", self.message)
    }
}

impl std::fmt::Debug for FileOperationException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FileOperationException {{ message: {} }}", self.message)
    }
}

impl std::error::Error for FileOperationException {}

impl From<FileOperationException> for ZzylError {
    fn from(exception: FileOperationException) -> Self {
        ZzylError::FileOperation(exception.message)
    }
}

/// 验证码异常
pub struct CaptchaException {
    pub message: String,
}

impl CaptchaException {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl std::fmt::Display for CaptchaException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CaptchaException: {}", self.message)
    }
}

impl std::fmt::Debug for CaptchaException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CaptchaException {{ message: {} }}", self.message)
    }
}

impl std::error::Error for CaptchaException {}

impl From<CaptchaException> for ZzylError {
    fn from(exception: CaptchaException) -> Self {
        ZzylError::Captcha(exception.message)
    }
}

/// 令牌异常
pub struct TokenException {
    pub message: String,
}

impl TokenException {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl std::fmt::Display for TokenException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TokenException: {}", self.message)
    }
}

impl std::fmt::Debug for TokenException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TokenException {{ message: {} }}", self.message)
    }
}

impl std::error::Error for TokenException {}

impl From<TokenException> for ZzylError {
    fn from(exception: TokenException) -> Self {
        ZzylError::Token(exception.message)
    }
}

/// 用户异常
pub struct UserException {
    pub message: String,
}

impl UserException {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl std::fmt::Display for UserException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UserException: {}", self.message)
    }
}

impl std::fmt::Debug for UserException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UserException {{ message: {} }}", self.message)
    }
}

impl std::error::Error for UserException {}

impl From<UserException> for ZzylError {
    fn from(exception: UserException) -> Self {
        ZzylError::User(exception.message)
    }
}

/// 权限异常
pub struct PermissionException {
    pub message: String,
}

impl PermissionException {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl std::fmt::Display for PermissionException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PermissionException: {}", self.message)
    }
}

impl std::fmt::Debug for PermissionException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PermissionException {{ message: {} }}", self.message)
    }
}

impl std::error::Error for PermissionException {}

impl From<PermissionException> for ZzylError {
    fn from(exception: PermissionException) -> Self {
        ZzylError::Permission(exception.message)
    }
}

/// 操作被禁止异常
pub struct ForbiddenException {
    pub message: String,
}

impl ForbiddenException {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl std::fmt::Display for ForbiddenException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ForbiddenException: {}", self.message)
    }
}

impl std::fmt::Debug for ForbiddenException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ForbiddenException {{ message: {} }}", self.message)
    }
}

impl std::error::Error for ForbiddenException {}

impl From<ForbiddenException> for ZzylError {
    fn from(exception: ForbiddenException) -> Self {
        ZzylError::Forbidden(exception.message)
    }
}

/// 请求过于频繁异常
pub struct TooManyRequestsException {
    pub message: String,
}

impl TooManyRequestsException {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl std::fmt::Display for TooManyRequestsException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TooManyRequestsException: {}", self.message)
    }
}

impl std::fmt::Debug for TooManyRequestsException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TooManyRequestsException {{ message: {} }}", self.message)
    }
}

impl std::error::Error for TooManyRequestsException {}

impl From<TooManyRequestsException> for ZzylError {
    fn from(exception: TooManyRequestsException) -> Self {
        ZzylError::TooManyRequests(exception.message)
    }
}

/// 服务不可用异常
pub struct ServiceUnavailableException {
    pub message: String,
}

impl ServiceUnavailableException {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl std::fmt::Display for ServiceUnavailableException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ServiceUnavailableException: {}", self.message)
    }
}

impl std::fmt::Debug for ServiceUnavailableException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ServiceUnavailableException {{ message: {} }}", self.message)
    }
}

impl std::error::Error for ServiceUnavailableException {}

impl From<ServiceUnavailableException> for ZzylError {
    fn from(exception: ServiceUnavailableException) -> Self {
        ZzylError::ServiceUnavailable(exception.message)
    }
}

