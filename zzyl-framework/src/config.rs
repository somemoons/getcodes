use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 应用配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// 应用名称
    pub name: String,
    /// 应用版本
    pub version: String,
    /// 版权年份
    pub copyright_year: String,
    /// 文件路径
    pub profile: String,
    /// 获取ip地址开关
    pub address_enabled: bool,
    /// 验证码类型
    pub captcha_type: String,
}

/// 服务器配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// 服务器端口
    pub port: u16,
    /// 应用上下文路径
    pub context_path: String,
    /// URI编码
    pub uri_encoding: String,
    /// 连接数满后的排队数
    pub accept_count: i32,
    /// 最大线程数
    pub max_threads: i32,
    /// 最小空闲线程数
    pub min_spare_threads: i32,
}

/// 数据库配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// 数据库类型
    pub db_type: String,
    /// 数据库地址
    pub host: String,
    /// 数据库端口
    pub port: u16,
    /// 数据库名称
    pub name: String,
    /// 用户名
    pub username: String,
    /// 密码
    pub password: String,
    /// 连接池配置
    pub pool: PoolConfig,
}

/// 连接池配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolConfig {
    /// 最小连接数
    pub min_idle: u32,
    /// 最大连接数
    pub max_size: u32,
    /// 连接超时时间（秒）
    pub connect_timeout: u64,
    /// 空闲超时时间（秒）
    pub idle_timeout: u64,
    /// 最大生存时间（秒）
    pub max_lifetime: u64,
}

/// Redis配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    /// Redis地址
    pub host: String,
    /// Redis端口
    pub port: u16,
    /// Redis密码
    pub password: Option<String>,
    /// 数据库索引
    pub database: i64,
    /// 连接池配置
    pub pool: PoolConfig,
    /// 连接超时时间（秒）
    pub timeout: u64,
}

/// 日志配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// 日志级别
    pub level: String,
    /// 日志文件路径
    pub file_path: Option<String>,
    /// 日志文件大小限制（MB）
    pub max_file_size: Option<u64>,
    /// 保留的日志文件数量
    pub max_files: Option<u32>,
}

/// 用户配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserConfig {
    /// 密码配置
    pub password: PasswordConfig,
}

/// 密码配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordConfig {
    /// 密码最大错误次数
    pub max_retry_count: i32,
    /// 密码锁定时间（分钟）
    pub lock_time: i32,
}

/// Spring配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpringConfig {
    /// 数据源配置
    pub datasource: DatabaseConfig,
    /// Redis配置
    pub redis: RedisConfig,
    /// 缓存配置
    pub cache: CacheConfig,
    /// 任务配置
    pub task: TaskConfig,
}

/// 缓存配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// 缓存类型
    pub cache_type: String,
    /// 缓存前缀
    pub key_prefix: String,
    /// 缓存过期时间（秒）
    pub timeout: u64,
    /// 最大缓存数量
    pub max_size: Option<usize>,
}

/// 任务配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskConfig {
    /// 线程池配置
    pub pool: ThreadPoolConfig,
}

/// 线程池配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadPoolConfig {
    /// 核心线程数
    pub core_size: i32,
    /// 最大线程数
    pub max_size: i32,
    /// 队列容量
    pub queue_capacity: i32,
    /// 线程空闲时间（秒）
    pub keep_alive_seconds: i32,
}

/// 完整配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZzylConfig {
    /// 应用配置
    pub ruoyi: AppConfig,
    /// 服务器配置
    pub server: ServerConfig,
    /// 日志配置
    pub logging: LoggingConfig,
    /// 用户配置
    pub user: UserConfig,
    /// Spring配置
    pub spring: SpringConfig,
}

impl ZzylConfig {
    /// 从文件加载配置
    pub fn from_file(path: &str) -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name(path).required(false))
            .add_source(Environment::with_prefix("ZZYL").separator("_"))
            .build()?;
        
        config.try_deserialize()
    }
    
    /// 从环境变量加载配置
    pub fn from_env() -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(Environment::with_prefix("ZZYL").separator("_"))
            .build()?;
        
        config.try_deserialize()
    }
    
    /// 获取数据库连接URL
    pub fn get_database_url(&self) -> String {
        format!(
            "{}://{}:{}@{}:{}/{}",
            self.spring.datasource.db_type,
            self.spring.datasource.username,
            self.spring.datasource.password,
            self.spring.datasource.host,
            self.spring.datasource.port,
            self.spring.datasource.name
        )
    }
    
    /// 获取Redis连接URL
    pub fn get_redis_url(&self) -> String {
        if let Some(password) = &self.spring.redis.password {
            format!(
                "redis://:{}@{}:{}/{}",
                password,
                self.spring.redis.host,
                self.spring.redis.port,
                self.spring.redis.database
            )
        } else {
            format!(
                "redis://{}:{}/{}",
                self.spring.redis.host,
                self.spring.redis.port,
                self.spring.redis.database
            )
        }
    }
    
    /// 获取JWT密钥
    pub fn get_jwt_secret(&self) -> String {
        std::env::var("JWT_SECRET").unwrap_or_else(|_| "zzyl_default_secret".to_string())
    }
    
    /// 获取文件上传路径
    pub fn get_upload_path(&self) -> String {
        self.ruoyi.profile.clone()
    }
    
    /// 是否启用地址获取
    pub fn is_address_enabled(&self) -> bool {
        self.ruoyi.address_enabled
    }
    
    /// 获取验证码类型
    pub fn get_captcha_type(&self) -> &str {
        &self.ruoyi.captcha_type
    }
}

impl Default for ZzylConfig {
    fn default() -> Self {
        Self {
            ruoyi: AppConfig {
                name: "RuoYi".to_string(),
                version: "3.8.8".to_string(),
                copyright_year: "2024".to_string(),
                profile: "D:/ruoyi/uploadPath".to_string(),
                address_enabled: false,
                captcha_type: "math".to_string(),
            },
            server: ServerConfig {
                port: 8080,
                context_path: "/".to_string(),
                uri_encoding: "UTF-8".to_string(),
                accept_count: 1000,
                max_threads: 800,
                min_spare_threads: 100,
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                file_path: None,
                max_file_size: None,
                max_files: None,
            },
            user: UserConfig {
                password: PasswordConfig {
                    max_retry_count: 5,
                    lock_time: 10,
                },
            },
            spring: SpringConfig {
                datasource: DatabaseConfig {
                    db_type: "mysql".to_string(),
                    host: "localhost".to_string(),
                    port: 3306,
                    name: "zzyl".to_string(),
                    username: "root".to_string(),
                    password: "password".to_string(),
                    pool: PoolConfig {
                        min_idle: 5,
                        max_size: 20,
                        connect_timeout: 30,
                        idle_timeout: 600,
                        max_lifetime: 1800,
                    },
                },
                redis: RedisConfig {
                    host: "localhost".to_string(),
                    port: 6379,
                    password: None,
                    database: 0,
                    pool: PoolConfig {
                        min_idle: 5,
                        max_size: 20,
                        connect_timeout: 30,
                        idle_timeout: 600,
                        max_lifetime: 1800,
                    },
                    timeout: 30,
                },
                cache: CacheConfig {
                    cache_type: "redis".to_string(),
                    key_prefix: "zzyl:".to_string(),
                    timeout: 3600,
                    max_size: Some(10000),
                },
                task: TaskConfig {
                    pool: ThreadPoolConfig {
                        core_size: 10,
                        max_size: 50,
                        queue_capacity: 200,
                        keep_alive_seconds: 60,
                    },
                },
            },
        }
    }
}

/// 配置管理器
pub struct ConfigManager {
    config: ZzylConfig,
}

impl ConfigManager {
    /// 创建配置管理器
    pub fn new(config: ZzylConfig) -> Self {
        Self { config }
    }
    
    /// 获取配置
    pub fn get_config(&self) -> &ZzylConfig {
        &self.config
    }
    
    /// 更新配置
    pub fn update_config(&mut self, config: ZzylConfig) {
        self.config = config;
    }
    
    /// 获取应用配置
    pub fn get_app_config(&self) -> &AppConfig {
        &self.config.ruoyi
    }
    
    /// 获取服务器配置
    pub fn get_server_config(&self) -> &ServerConfig {
        &self.config.server
    }
    
    /// 获取数据库配置
    pub fn get_database_config(&self) -> &DatabaseConfig {
        &self.config.spring.datasource
    }
    
    /// 获取Redis配置
    pub fn get_redis_config(&self) -> &RedisConfig {
        &self.config.spring.redis
    }
    
    /// 获取日志配置
    pub fn get_logging_config(&self) -> &LoggingConfig {
        &self.config.logging
    }
    
    /// 获取用户配置
    pub fn get_user_config(&self) -> &UserConfig {
        &self.config.user
    }
}

/// 全局配置实例
static mut CONFIG_MANAGER: Option<ConfigManager> = None;

/// 初始化配置
pub fn init_config(config: ZzylConfig) {
    unsafe {
        CONFIG_MANAGER = Some(ConfigManager::new(config));
    }
}

/// 获取全局配置
pub fn get_config() -> Option<&'static ZzylConfig> {
    unsafe {
        CONFIG_MANAGER.as_ref().map(|manager| manager.get_config())
    }
}

/// 获取配置管理器
pub fn get_config_manager() -> Option<&'static ConfigManager> {
    unsafe {
        CONFIG_MANAGER.as_ref()
    }
}
