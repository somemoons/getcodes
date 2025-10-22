use sqlx::{MySqlPool, Pool, MySql, Row};
use sea_orm::{Database, DatabaseConnection, ConnectOptions, ConnectionTrait};
use std::time::Duration;
use crate::config::{DatabaseConfig, PoolConfig};
use crate::error::{Result, ZzylError};

/// 数据库连接池
pub struct DatabaseManager {
    pool: MySqlPool,
    sea_orm_conn: DatabaseConnection,
}

impl DatabaseManager {
    /// 创建数据库连接池
    pub async fn new(config: &DatabaseConfig) -> Result<Self> {
        let database_url = format!(
            "mysql://{}:{}@{}:{}/{}",
            config.username,
            config.password,
            config.host,
            config.port,
            config.name
        );
        
        // 创建SQLx连接池
        let pool_options = sqlx::mysql::MySqlPoolOptions::new()
            .min_connections(config.pool.min_idle)
            .max_connections(config.pool.max_size)
            .acquire_timeout(Duration::from_secs(config.pool.connect_timeout))
            .idle_timeout(Duration::from_secs(config.pool.idle_timeout))
            .max_lifetime(Duration::from_secs(config.pool.max_lifetime));
        
        let pool = pool_options.connect(&database_url).await
            .map_err(|e| ZzylError::Database(format!("Failed to create database pool: {}", e)))?;
        
        // 创建SeaORM连接
        let mut opt = ConnectOptions::new(database_url);
        opt.max_connections(config.pool.max_size)
            .min_connections(config.pool.min_idle)
            .connect_timeout(Duration::from_secs(config.pool.connect_timeout))
            .acquire_timeout(Duration::from_secs(config.pool.connect_timeout))
            .idle_timeout(Duration::from_secs(config.pool.idle_timeout))
            .max_lifetime(Duration::from_secs(config.pool.max_lifetime));
        
        let sea_orm_conn = Database::connect(opt).await
            .map_err(|e| ZzylError::Database(format!("Failed to create SeaORM connection: {}", e)))?;
        
        Ok(Self {
            pool,
            sea_orm_conn,
        })
    }
    
    /// 获取SQLx连接池
    pub fn get_pool(&self) -> &MySqlPool {
        &self.pool
    }
    
    /// 获取SeaORM连接
    pub fn get_sea_orm_conn(&self) -> &DatabaseConnection {
        &self.sea_orm_conn
    }
    
    /// 执行查询
    pub async fn query(&self, sql: &str) -> Result<Vec<sqlx::mysql::MySqlRow>> {
        let rows = sqlx::query(sql)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| ZzylError::Database(format!("Query failed: {}", e)))?;
        
        Ok(rows)
    }
    
    /// 执行查询并返回单个结果
    pub async fn query_one(&self, sql: &str) -> Result<sqlx::mysql::MySqlRow> {
        let row = sqlx::query(sql)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| ZzylError::Database(format!("Query one failed: {}", e)))?;
        
        Ok(row)
    }
    
    /// 执行更新操作
    pub async fn execute(&self, sql: &str) -> Result<sqlx::mysql::MySqlQueryResult> {
        let result = sqlx::query(sql)
            .execute(&self.pool)
            .await
            .map_err(|e| ZzylError::Database(format!("Execute failed: {}", e)))?;
        
        Ok(result)
    }
    
    /// 执行事务
    pub async fn transaction<F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(&mut sqlx::Transaction<'_, MySql>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<R>> + Send + '_>>,
    {
        let mut tx = self.pool.begin().await
            .map_err(|e| ZzylError::Database(format!("Begin transaction failed: {}", e)))?;
        
        let result = f(&mut tx).await?;
        
        tx.commit().await
            .map_err(|e| ZzylError::Database(format!("Commit transaction failed: {}", e)))?;
        
        Ok(result)
    }
    
    /// 检查连接是否健康
    pub async fn health_check(&self) -> Result<bool> {
        match sqlx::query("SELECT 1")
            .fetch_one(&self.pool)
            .await
        {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
    
    /// 获取连接池状态
    pub fn get_pool_status(&self) -> PoolStatus {
        PoolStatus {
            size: self.pool.size(),
            idle: self.pool.num_idle(),
            max_size: self.pool.options().get_max_connections(),
            min_size: self.pool.options().get_min_connections(),
        }
    }
    
    /// 关闭连接池
    pub async fn close(self) -> Result<()> {
        self.pool.close().await;
        Ok(())
    }
}

/// 连接池状态
#[derive(Debug, Clone)]
pub struct PoolStatus {
    /// 当前连接数
    pub size: u32,
    /// 空闲连接数
    pub idle: usize,
    /// 最大连接数
    pub max_size: u32,
    /// 最小连接数
    pub min_size: u32,
}

/// 数据库工具类
pub struct DatabaseUtils;

impl DatabaseUtils {
    /// 构建分页查询SQL
    pub fn build_pagination_sql(base_sql: &str, page_num: i32, page_size: i32, order_by: Option<&str>) -> String {
        let mut sql = base_sql.to_string();
        
        // 添加排序
        if let Some(order) = order_by {
            sql.push_str(&format!(" ORDER BY {}", order));
        }
        
        // 添加分页
        let offset = (page_num - 1) * page_size;
        sql.push_str(&format!(" LIMIT {} OFFSET {}", page_size, offset));
        
        sql
    }
    
    /// 构建计数查询SQL
    pub fn build_count_sql(base_sql: &str) -> String {
        format!("SELECT COUNT(*) as total FROM ({}) as count_table", base_sql)
    }
    
    /// 构建WHERE条件SQL
    pub fn build_where_conditions(conditions: &[String]) -> String {
        if conditions.is_empty() {
            String::new()
        } else {
            format!(" WHERE {}", conditions.join(" AND "))
        }
    }
    
    /// 构建IN条件SQL
    pub fn build_in_condition<T>(column: &str, values: &[T]) -> String
    where
        T: std::fmt::Display,
    {
        if values.is_empty() {
            return String::new();
        }
        
        let values_str = values.iter()
            .map(|v| format!("'{}'", v))
            .collect::<Vec<_>>()
            .join(",");
        
        format!("{} IN ({})", column, values_str)
    }
    
    /// 构建LIKE条件SQL
    pub fn build_like_condition(column: &str, value: &str, pattern: LikePattern) -> String {
        match pattern {
            LikePattern::Contains => format!("{} LIKE '%{}%'", column, value),
            LikePattern::StartsWith => format!("{} LIKE '{}%'", column, value),
            LikePattern::EndsWith => format!("{} LIKE '%{}'", column, value),
        }
    }
    
    /// 构建范围条件SQL
    pub fn build_range_condition(column: &str, start: Option<&str>, end: Option<&str>) -> String {
        let mut conditions = Vec::new();
        
        if let Some(start_val) = start {
            conditions.push(format!("{} >= '{}'", column, start_val));
        }
        
        if let Some(end_val) = end {
            conditions.push(format!("{} <= '{}'", column, end_val));
        }
        
        conditions.join(" AND ")
    }
    
    /// 转义SQL字符串
    pub fn escape_sql_string(s: &str) -> String {
        s.replace("'", "''")
            .replace("\\", "\\\\")
            .replace("\0", "\\0")
            .replace("\n", "\\n")
            .replace("\r", "\\r")
            .replace("\x1a", "\\Z")
    }
    
    /// 验证SQL安全性（简单的SQL注入检查）
    pub fn is_sql_safe(sql: &str) -> bool {
        let dangerous_patterns = [
            "DROP", "DELETE", "UPDATE", "INSERT", "ALTER", "CREATE", "TRUNCATE",
            "EXEC", "EXECUTE", "UNION", "SCRIPT", "javascript:", "vbscript:",
            "onload", "onerror", "onclick", "onmouseover",
        ];
        
        let sql_upper = sql.to_uppercase();
        !dangerous_patterns.iter().any(|pattern| sql_upper.contains(pattern))
    }
}

/// LIKE模式
#[derive(Debug, Clone, Copy)]
pub enum LikePattern {
    /// 包含
    Contains,
    /// 以...开始
    StartsWith,
    /// 以...结束
    EndsWith,
}

/// 全局数据库管理器
static mut DATABASE_MANAGER: Option<DatabaseManager> = None;

/// 初始化数据库
pub async fn init_database(config: &DatabaseConfig) -> Result<()> {
    let manager = DatabaseManager::new(config).await?;
    unsafe {
        DATABASE_MANAGER = Some(manager);
    }
    Ok(())
}

/// 获取数据库管理器
pub fn get_database_manager() -> Option<&'static DatabaseManager> {
    unsafe {
        DATABASE_MANAGER.as_ref()
    }
}

/// 获取数据库连接池
pub fn get_database_pool() -> Option<&'static MySqlPool> {
    get_database_manager().map(|manager| manager.get_pool())
}

/// 获取SeaORM连接
pub fn get_sea_orm_connection() -> Option<&'static DatabaseConnection> {
    get_database_manager().map(|manager| manager.get_sea_orm_conn())
}

