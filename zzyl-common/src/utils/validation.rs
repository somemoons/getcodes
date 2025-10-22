use validator::{Validate, ValidationError};

/// 验证工具类
pub struct ValidationUtils;

impl ValidationUtils {
    /// 验证用户名
    pub fn validate_username(username: &str) -> Result<(), ValidationError> {
        if username.len() < 2 || username.len() > 20 {
            return Err(ValidationError::new("username_length"));
        }
        
        if !username.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err(ValidationError::new("username_format"));
        }
        
        Ok(())
    }
    
    /// 验证密码
    pub fn validate_password(password: &str) -> Result<(), ValidationError> {
        if password.len() < 5 || password.len() > 20 {
            return Err(ValidationError::new("password_length"));
        }
        
        Ok(())
    }
    
    /// 验证邮箱
    pub fn validate_email(email: &str) -> Result<(), ValidationError> {
        let re = regex::Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        if !re.is_match(email) {
            return Err(ValidationError::new("email_format"));
        }
        Ok(())
    }
    
    /// 验证手机号
    pub fn validate_phone(phone: &str) -> Result<(), ValidationError> {
        let re = regex::Regex::new(r"^1[3-9]\d{9}$").unwrap();
        if !re.is_match(phone) {
            return Err(ValidationError::new("phone_format"));
        }
        Ok(())
    }
    
    /// 验证身份证号
    pub fn validate_id_card(id_card: &str) -> Result<(), ValidationError> {
        let re = regex::Regex::new(r"^\d{17}[\dXx]$").unwrap();
        if !re.is_match(id_card) {
            return Err(ValidationError::new("id_card_format"));
        }
        Ok(())
    }
    
    /// 验证URL
    pub fn validate_url(url: &str) -> Result<(), ValidationError> {
        if url::Url::parse(url).is_err() {
            return Err(ValidationError::new("url_format"));
        }
        Ok(())
    }
    
    /// 验证IP地址
    pub fn validate_ip(ip: &str) -> Result<(), ValidationError> {
        if ip.parse::<std::net::IpAddr>().is_err() {
            return Err(ValidationError::new("ip_format"));
        }
        Ok(())
    }
    
    /// 验证端口号
    pub fn validate_port(port: u16) -> Result<(), ValidationError> {
        if port == 0 {
            return Err(ValidationError::new("port_range"));
        }
        Ok(())
    }
    
    /// 验证年龄
    pub fn validate_age(age: i32) -> Result<(), ValidationError> {
        if age < 0 || age > 150 {
            return Err(ValidationError::new("age_range"));
        }
        Ok(())
    }
    
    /// 验证价格
    pub fn validate_price(price: f64) -> Result<(), ValidationError> {
        if price < 0.0 {
            return Err(ValidationError::new("price_range"));
        }
        Ok(())
    }
    
    /// 验证排序号
    pub fn validate_order_num(order_num: i32) -> Result<(), ValidationError> {
        if order_num < 0 {
            return Err(ValidationError::new("order_num_range"));
        }
        Ok(())
    }
    
    /// 验证状态值
    pub fn validate_status(status: i32) -> Result<(), ValidationError> {
        if status != 0 && status != 1 {
            return Err(ValidationError::new("status_value"));
        }
        Ok(())
    }
    
    /// 验证删除标志
    pub fn validate_del_flag(del_flag: i32) -> Result<(), ValidationError> {
        if del_flag != 0 && del_flag != 2 {
            return Err(ValidationError::new("del_flag_value"));
        }
        Ok(())
    }
    
    /// 验证菜单类型
    pub fn validate_menu_type(menu_type: &str) -> Result<(), ValidationError> {
        if !["M", "C", "F"].contains(&menu_type) {
            return Err(ValidationError::new("menu_type_value"));
        }
        Ok(())
    }
    
    /// 验证数据范围
    pub fn validate_data_scope(data_scope: i32) -> Result<(), ValidationError> {
        if !(1..=5).contains(&data_scope) {
            return Err(ValidationError::new("data_scope_value"));
        }
        Ok(())
    }
    
    /// 验证业务类型
    pub fn validate_business_type(business_type: i32) -> Result<(), ValidationError> {
        if !(0..=9).contains(&business_type) {
            return Err(ValidationError::new("business_type_value"));
        }
        Ok(())
    }
    
    /// 验证操作状态
    pub fn validate_oper_status(oper_status: i32) -> Result<(), ValidationError> {
        if oper_status != 0 && oper_status != 1 {
            return Err(ValidationError::new("oper_status_value"));
        }
        Ok(())
    }
    
    /// 验证登录状态
    pub fn validate_login_status(login_status: i32) -> Result<(), ValidationError> {
        if login_status != 0 && login_status != 1 {
            return Err(ValidationError::new("login_status_value"));
        }
        Ok(())
    }
    
    /// 验证老人状态
    pub fn validate_elder_status(elder_status: i32) -> Result<(), ValidationError> {
        if !(0..=5).contains(&elder_status) {
            return Err(ValidationError::new("elder_status_value"));
        }
        Ok(())
    }
    
    /// 验证性别
    pub fn validate_gender(gender: i32) -> Result<(), ValidationError> {
        if gender != 0 && gender != 1 {
            return Err(ValidationError::new("gender_value"));
        }
        Ok(())
    }
    
    /// 验证分页参数
    pub fn validate_page_params(page_num: i32, page_size: i32) -> Result<(), ValidationError> {
        if page_num < 1 {
            return Err(ValidationError::new("page_num_range"));
        }
        
        if page_size < 1 || page_size > 500 {
            return Err(ValidationError::new("page_size_range"));
        }
        
        Ok(())
    }
}

