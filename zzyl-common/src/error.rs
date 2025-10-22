use std::fmt;
use thiserror::Error;

/// 通用错误类型
#[derive(Error, Debug)]
pub enum ZzylError {
    #[error("数据库错误: {0}")]
    Database(String),
    
    #[error("认证错误: {0}")]
    Authentication(String),
    
    #[error("授权错误: {0}")]
    Authorization(String),
    
    #[error("参数错误: {0}")]
    InvalidParameter(String),
    
    #[error("业务错误: {0}")]
    Business(String),
    
    #[error("系统错误: {0}")]
    System(String),
    
    #[error("文件操作错误: {0}")]
    FileOperation(String),
    
    #[error("网络错误: {0}")]
    Network(String),
    
    #[error("配置错误: {0}")]
    Configuration(String),
    
    #[error("序列化错误: {0}")]
    Serialization(String),
    
    #[error("验证码错误: {0}")]
    Captcha(String),
    
    #[error("令牌错误: {0}")]
    Token(String),
    
    #[error("用户错误: {0}")]
    User(String),
    
    #[error("权限错误: {0}")]
    Permission(String),
    
    #[error("数据不存在: {0}")]
    NotFound(String),
    
    #[error("数据已存在: {0}")]
    AlreadyExists(String),
    
    #[error("操作被禁止: {0}")]
    Forbidden(String),
    
    #[error("请求过于频繁: {0}")]
    TooManyRequests(String),
    
    #[error("服务不可用: {0}")]
    ServiceUnavailable(String),
    
    #[error("IO错误: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON错误: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("SQL错误: {0}")]
    Sql(#[from] sqlx::Error),
    
    #[error("Redis错误: {0}")]
    Redis(#[from] redis::RedisError),
    
    #[error("JWT错误: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),
    
    #[error("HTTP错误代: {0}")]
    HttpStatus(u16),
}

/// 通用结果类型
pub type Result<T> = std::result::Result<T, ZzylError>;

impl ZzylError {
    /// 获取HTTP状态码
    pub fn status_code(&self) -> u16 {
        match self {
            ZzylError::InvalidParameter(_) => 400,
            ZzylError::Authentication(_) => 401,
            ZzylError::Authorization(_) | ZzylError::Permission(_) => 403,
            ZzylError::NotFound(_) => 404,
            ZzylError::AlreadyExists(_) => 409,
            ZzylError::TooManyRequests(_) => 429,
            ZzylError::ServiceUnavailable(_) => 503,
            ZzylError::HttpStatus(code) => *code,
            _ => 500,
        }
    }
    
    /// 获取错误代码
    pub fn error_code(&self) -> i32 {
        match self {
            ZzylError::InvalidParameter(_) => 400,
            ZzylError::Authentication(_) => 401,
            ZzylError::Authorization(_) | ZzylError::Permission(_) => 403,
            ZzylError::NotFound(_) => 404,
            ZzylError::AlreadyExists(_) => 409,
            ZzylError::TooManyRequests(_) => 429,
            ZzylError::ServiceUnavailable(_) => 503,
            ZzylError::HttpStatus(code) => *code as i32,
            _ => 500,
        }
    }
    
    /// 获取错误消息
    pub fn error_message(&self) -> String {
        self.to_string()
    }
}

/// HTTP响应结果
#[derive(Debug, Clone, serde::Serialize)]
pub struct AjaxResult<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

impl<T> AjaxResult<T> {
    /// 成功响应
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            msg: "操作成功".to_string(),
            data: Some(data),
        }
    }
    
    /// 成功响应（无数据）
    pub fn success_msg(msg: &str) -> Self {
        Self {
            code: 200,
            msg: msg.to_string(),
            data: None,
        }
    }
    
    /// 失败响应
    pub fn error(msg: &str) -> Self {
        Self {
            code: 500,
            msg: msg.to_string(),
            data: None,
        }
    }
    
    /// 失败响应（带错误代码）
    pub fn error_with_code(code: i32, msg: &str) -> Self {
        Self {
            code,
            msg: msg.to_string(),
            data: None,
        }
    }
}

impl<T> From<ZzylError> for AjaxResult<T> {
    fn from(error: ZzylError) -> Self {
        Self {
            code: error.error_code(),
            msg: error.error_message(),
            data: None,
        }
    }
}

/// 分页响应结果
#[derive(Debug, Clone, serde::Serialize)]
pub struct TableDataInfo<T> {
    pub code: i32,
    pub msg: String,
    pub rows: Vec<T>,
    pub total: i64,
}

impl<T> TableDataInfo<T> {
    /// 分页成功响应
    pub fn success(rows: Vec<T>, total: i64) -> Self {
        Self {
            code: 200,
            msg: "查询成功".to_string(),
            rows,
            total,
        }
    }
    
    /// 分页失败响应
    pub fn error(msg: &str) -> Self {
        Self {
            code: 500,
            msg: msg.to_string(),
            rows: Vec::new(),
            total: 0,
        }
    }
}

