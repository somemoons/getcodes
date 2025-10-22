use chrono::{DateTime, Local, Utc, NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

/// 日期工具类
pub struct DateUtils;

impl DateUtils {
    /// 获取当前时间
    pub fn now() -> DateTime<Utc> {
        Utc::now()
    }
    
    /// 获取本地当前时间
    pub fn now_local() -> DateTime<Local> {
        Local::now()
    }
    
    /// 格式化日期时间
    pub fn format_datetime(dt: &DateTime<Utc>, format: &str) -> String {
        dt.format(format).to_string()
    }
    
    /// 默认日期时间格式
    pub fn format_datetime_default(dt: &DateTime<Utc>) -> String {
        Self::format_datetime(dt, "%Y-%m-%d %H:%M:%S")
    }
    
    /// 格式化日期
    pub fn format_date(dt: &DateTime<Utc>) -> String {
        Self::format_datetime(dt, "%Y-%m-%d")
    }
    
    /// 格式化时间
    pub fn format_time(dt: &DateTime<Utc>) -> String {
        Self::format_datetime(dt, "%H:%M:%S")
    }
    
    /// 解析日期时间字符串
    pub fn parse_datetime(date_str: &str, format: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
        let naive = NaiveDateTime::parse_from_str(date_str, format)?;
        Ok(DateTime::from_naive_utc_and_offset(naive, Utc))
    }
    
    /// 解析日期字符串（默认格式）
    pub fn parse_datetime_default(date_str: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
        Self::parse_datetime(date_str, "%Y-%m-%d %H:%M:%S")
    }
    
    /// 解析日期字符串
    pub fn parse_date(date_str: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
        let naive_date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")?;
        let naive_datetime = naive_date.and_hms_opt(0, 0, 0).unwrap();
        Ok(DateTime::from_naive_utc_and_offset(naive_datetime, Utc))
    }
    
    /// 获取时间戳（秒）
    pub fn timestamp(dt: &DateTime<Utc>) -> i64 {
        dt.timestamp()
    }
    
    /// 从时间戳创建日期时间
    pub fn from_timestamp(timestamp: i64) -> DateTime<Utc> {
        DateTime::from_timestamp(timestamp, 0).unwrap_or_default()
    }
    
    /// 获取时间戳（毫秒）
    pub fn timestamp_millis(dt: &DateTime<Utc>) -> i64 {
        dt.timestamp_millis()
    }
    
    /// 从毫秒时间戳创建日期时间
    pub fn from_timestamp_millis(timestamp: i64) -> DateTime<Utc> {
        DateTime::from_timestamp_millis(timestamp).unwrap_or_default()
    }
    
    /// 计算两个日期之间的天数差
    pub fn days_between(start: &DateTime<Utc>, end: &DateTime<Utc>) -> i64 {
        (end.date_naive() - start.date_naive()).num_days()
    }
    
    /// 计算年龄
    pub fn calculate_age(birth_date: &DateTime<Utc>) -> i32 {
        let now = Utc::now();
        let age = now.year() - birth_date.year();
        
        // 如果今年的生日还没过，年龄减1
        if now.month() < birth_date.month() 
            || (now.month() == birth_date.month() && now.day() < birth_date.day()) {
            age - 1
        } else {
            age
        }
    }
    
    /// 获取本周开始时间
    pub fn week_start() -> DateTime<Utc> {
        let now = Utc::now();
        let days_from_monday = now.weekday().num_days_from_monday();
        now.date_naive()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .signed_duration_since(chrono::Duration::days(days_from_monday as i64))
            .to_utc()
    }
    
    /// 获取本月开始时间
    pub fn month_start() -> DateTime<Utc> {
        let now = Utc::now();
        now.date_naive()
            .with_day(1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc()
    }
    
    /// 获取本年开始时间
    pub fn year_start() -> DateTime<Utc> {
        let now = Utc::now();
        now.date_naive()
            .with_month(1)
            .unwrap()
            .with_day(1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc()
    }
}

