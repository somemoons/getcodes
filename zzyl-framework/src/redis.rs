use redis::{Client, ConnectionManager, RedisResult, Commands, AsyncCommands};
use std::time::Duration;
use serde::{Serialize, Deserialize};
use crate::config::RedisConfig;
use crate::error::{Result, ZzylError};

/// Redis管理器
pub struct RedisManager {
    client: Client,
    connection_manager: ConnectionManager,
}

impl RedisManager {
    /// 创建Redis管理器
    pub async fn new(config: &RedisConfig) -> Result<Self> {
        let redis_url = if let Some(password) = &config.password {
            format!("redis://:{}@{}:{}/{}", password, config.host, config.port, config.database)
        } else {
            format!("redis://{}:{}/{}", config.host, config.port, config.database)
        };
        
        let client = Client::open(redis_url)
            .map_err(|e| ZzylError::Redis(e))?;
        
        let connection_manager = ConnectionManager::new(client.clone()).await
            .map_err(|e| ZzylError::Redis(e))?;
        
        Ok(Self {
            client,
            connection_manager,
        })
    }
    
    /// 获取客户端
    pub fn get_client(&self) -> &Client {
        &self.client
    }
    
    /// 获取连接管理器
    pub fn get_connection_manager(&self) -> &ConnectionManager {
        &self.connection_manager
    }
    
    /// 设置字符串值
    pub async fn set(&self, key: &str, value: &str) -> Result<()> {
        let mut conn = self.connection_manager.clone();
        conn.set(key, value).await
            .map_err(|e| ZzylError::Redis(e))?;
        Ok(())
    }
    
    /// 设置字符串值（带过期时间）
    pub async fn set_ex(&self, key: &str, value: &str, seconds: u64) -> Result<()> {
        let mut conn = self.connection_manager.clone();
        conn.set_ex(key, value, seconds).await
            .map_err(|e| ZzylError::Redis(e))?;
        Ok(())
    }
    
    /// 获取字符串值
    pub async fn get(&self, key: &str) -> Result<Option<String>> {
        let mut conn = self.connection_manager.clone();
        let result: RedisResult<Option<String>> = conn.get(key).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(ZzylError::Redis(e)),
        }
    }
    
    /// 删除键
    pub async fn del(&self, key: &str) -> Result<i64> {
        let mut conn = self.connection_manager.clone();
        let result: RedisResult<i64> = conn.del(key).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(ZzylError::Redis(e)),
        }
    }
    
    /// 检查键是否存在
    pub async fn exists(&self, key: &str) -> Result<bool> {
        let mut conn = self.connection_manager.clone();
        let result: RedisResult<bool> = conn.exists(key).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(ZzylError::Redis(e)),
        }
    }
    
    /// 设置过期时间
    pub async fn expire(&self, key: &str, seconds: u64) -> Result<bool> {
        let mut conn = self.connection_manager.clone();
        let result: RedisResult<bool> = conn.expire(key, seconds as i64).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(ZzylError::Redis(e)),
        }
    }
    
    /// 获取剩余过期时间
    pub async fn ttl(&self, key: &str) -> Result<i64> {
        let mut conn = self.connection_manager.clone();
        let result: RedisResult<i64> = conn.ttl(key).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(ZzylError::Redis(e)),
        }
    }
    
    /// 设置JSON对象
    pub async fn set_json<T>(&self, key: &str, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        let json_str = serde_json::to_string(value)
            .map_err(|e| ZzylError::Serialization(e.to_string()))?;
        self.set(key, &json_str).await
    }
    
    /// 设置JSON对象（带过期时间）
    pub async fn set_json_ex<T>(&self, key: &str, value: &T, seconds: u64) -> Result<()>
    where
        T: Serialize,
    {
        let json_str = serde_json::to_string(value)
            .map_err(|e| ZzylError::Serialization(e.to_string()))?;
        self.set_ex(key, &json_str, seconds).await
    }
    
    /// 获取JSON对象
    pub async fn get_json<T>(&self, key: &str) -> Result<Option<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        match self.get(key).await? {
            Some(json_str) => {
                let value = serde_json::from_str(&json_str)
                    .map_err(|e| ZzylError::Serialization(e.to_string()))?;
                Ok(Some(value))
            }
            None => Ok(None),
        }
    }
    
    /// 列表操作 - 左推
    pub async fn lpush(&self, key: &str, value: &str) -> Result<i64> {
        let mut conn = self.connection_manager.clone();
        let result: RedisResult<i64> = conn.lpush(key, value).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(ZzylError::Redis(e)),
        }
    }
    
    /// 列表操作 - 右推
    pub async fn rpush(&self, key: &str, value: &str) -> Result<i64> {
        let mut conn = self.connection_manager.clone();
        let result: RedisResult<i64> = conn.rpush(key, value).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(ZzylError::Redis(e)),
        }
    }
    
    /// 列表操作 - 左弹
    pub async fn lpop(&self, key: &str) -> Result<Option<String>> {
        let mut conn = self.connection_manager.clone();
        let result: RedisResult<Option<String>> = conn.lpop(key, None).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(ZzylError::Redis(e)),
        }
    }
    
    /// 列表操作 - 右弹
    pub async fn rpop(&self, key: &str) -> Result<Option<String>> {
        let mut conn = self.connection_manager.clone();
        let result: RedisResult<Option<String>> = conn.rpop(key, None).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(ZzylError::Redis(e)),
        }
    }
    
    /// 列表操作 - 获取列表长度
    pub async fn llen(&self, key: &str) -> Result<i64> {
        let mut conn = self.connection_manager.clone();
        let result: RedisResult<i64> = conn.llen(key).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(ZzylError::Redis(e)),
        }
    }
    
    /// 列表操作 - 获取列表范围
    pub async fn lrange(&self, key: &str, start: i64, stop: i64) -> Result<Vec<String>> {
        let mut conn = self.connection_manager.clone();
        let result: RedisResult<Vec<String>> = conn.lrange(key, start, stop).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(ZzylError::Redis(e)),
        }
    }
    
    /// 集合操作 - 添加成员
    pub async fn sadd(&self, key: &str, member: &str) -> Result<i64> {
        let mut conn = self.connection_manager.clone();
        let result: RedisResult<i64> = conn.sadd(key, member).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(ZzylError::Redis(e)),
        }
    }
    
    /// 集合操作 - 删除成员
    pub async fn srem(&self, key: &str, member: &str) -> Result<i64> {
        let mut conn = self.connection_manager.clone();
        let result: RedisResult<i64> = conn.srem(key, member).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(ZzylError::Redis(e)),
        }
    }
    
    /// 集合操作 - 获取所有成员
    pub async fn smembers(&self, key: &str) -> Result<Vec<String>> {
        let mut conn = self.connection_manager.clone();
        let result: RedisResult<Vec<String>> = conn.smembers(key).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(ZzylError::Redis(e)),
        }
    }
    
    /// 集合操作 - 检查成员是否存在
    pub async fn sismember(&self, key: &str, member: &str) -> Result<bool> {
        let mut conn = self.connection_manager.clone();
        let result: RedisResult<bool> = conn.sismember(key, member).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(ZzylError::Redis(e)),
        }
    }
    
    /// 哈希操作 - 设置字段值
    pub async fn hset(&self, key: &str, field: &str, value: &str) -> Result<i64> {
        let mut conn = self.connection_manager.clone();
        let result: RedisResult<i64> = conn.hset(key, field, value).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(ZzylError::Redis(e)),
        }
    }
    
    /// 哈希操作 - 获取字段值
    pub async fn hget(&self, key: &str, field: &str) -> Result<Option<String>> {
        let mut conn = self.connection_manager.clone();
        let result: RedisResult<Option<String>> = conn.hget(key, field).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(ZzylError::Redis(e)),
        }
    }
    
    /// 哈希操作 - 获取所有字段值
    pub async fn hgetall(&self, key: &str) -> Result<Vec<(String, String)>> {
        let mut conn = self.connection_manager.clone();
        let result: RedisResult<Vec<(String, String)>> = conn.hgetall(key).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(ZzylError::Redis(e)),
        }
    }
    
    /// 哈希操作 - 删除字段
    pub async fn hdel(&self, key: &str, field: &str) -> Result<i64> {
        let mut conn = self.connection_manager.clone();
        let result: RedisResult<i64> = conn.hdel(key, field).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(ZzylError::Redis(e)),
        }
    }
    
    /// 原子操作 - 递增
    pub async fn incr(&self, key: &str) -> Result<i64> {
        let mut conn = self.connection_manager.clone();
        let result: RedisResult<i64> = conn.incr(key, 1).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(ZzylError::Redis(e)),
        }
    }
    
    /// 原子操作 - 递减
    pub async fn decr(&self, key: &str) -> Result<i64> {
        let mut conn = self.connection_manager.clone();
        let result: RedisResult<i64> = conn.decr(key, 1).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(ZzylError::Redis(e)),
        }
    }
    
    /// 原子操作 - 按指定值递增
    pub async fn incrby(&self, key: &str, increment: i64) -> Result<i64> {
        let mut conn = self.connection_manager.clone();
        let result: RedisResult<i64> = conn.incr(key, increment).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(ZzylError::Redis(e)),
        }
    }
    
    /// 原子操作 - 按指定值递减
    pub async fn decrby(&self, key: &str, decrement: i64) -> Result<i64> {
        let mut conn = self.connection_manager.clone();
        let result: RedisResult<i64> = conn.decr(key, decrement).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(ZzylError::Redis(e)),
        }
    }
    
    /// 获取所有键（匹配模式）
    pub async fn keys(&self, pattern: &str) -> Result<Vec<String>> {
        let mut conn = self.connection_manager.clone();
        let result: RedisResult<Vec<String>> = conn.keys(pattern).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(ZzylError::Redis(e)),
        }
    }
    
    /// 批量删除键
    pub async fn del_batch(&self, keys: &[String]) -> Result<i64> {
        if keys.is_empty() {
            return Ok(0);
        }
        
        let mut conn = self.connection_manager.clone();
        let result: RedisResult<i64> = conn.del(keys).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(ZzylError::Redis(e)),
        }
    }
    
    /// 检查连接是否健康
    pub async fn health_check(&self) -> Result<bool> {
        match self.get("__health_check__").await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}

/// Redis工具类
pub struct RedisUtils;

impl RedisUtils {
    /// 构建缓存键
    pub fn build_cache_key(prefix: &str, key: &str) -> String {
        format!("{}:{}", prefix, key)
    }
    
    /// 构建用户缓存键
    pub fn build_user_cache_key(user_id: &str) -> String {
        Self::build_cache_key("user", user_id)
    }
    
    /// 构建会话缓存键
    pub fn build_session_cache_key(session_id: &str) -> String {
        Self::build_cache_key("session", session_id)
    }
    
    /// 构建验证码缓存键
    pub fn build_captcha_cache_key(captcha_id: &str) -> String {
        Self::build_cache_key("captcha", captcha_id)
    }
    
    /// 构建令牌缓存键
    pub fn build_token_cache_key(token: &str) -> String {
        Self::build_cache_key("token", token)
    }
    
    /// 构建限流缓存键
    pub fn build_rate_limit_cache_key(ip: &str, path: &str) -> String {
        Self::build_cache_key("rate_limit", &format!("{}:{}", ip, path))
    }
    
    /// 构建重复提交缓存键
    pub fn build_repeat_submit_cache_key(user_id: &str, path: &str) -> String {
        Self::build_cache_key("repeat_submit", &format!("{}:{}", user_id, path))
    }
    
    /// 构建登录失败缓存键
    pub fn build_login_fail_cache_key(username: &str) -> String {
        Self::build_cache_key("login_fail", username)
    }
}

/// 全局Redis管理器
static mut REDIS_MANAGER: Option<RedisManager> = None;

/// 初始化Redis
pub async fn init_redis(config: &RedisConfig) -> Result<()> {
    let manager = RedisManager::new(config).await?;
    unsafe {
        REDIS_MANAGER = Some(manager);
    }
    Ok(())
}

/// 获取Redis管理器
pub fn get_redis_manager() -> Option<&'static RedisManager> {
    unsafe {
        REDIS_MANAGER.as_ref()
    }
}

