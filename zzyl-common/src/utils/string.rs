use regex::Regex;

/// 字符串工具类
pub struct StringUtils;

impl StringUtils {
    /// 判断字符串是否为空
    pub fn is_empty(s: &str) -> bool {
        s.is_empty()
    }
    
    /// 判断字符串是否不为空
    pub fn is_not_empty(s: &str) -> bool {
        !s.is_empty()
    }
    
    /// 判断字符串是否为空白
    pub fn is_blank(s: &str) -> bool {
        s.trim().is_empty()
    }
    
    /// 判断字符串是否不为空白
    pub fn is_not_blank(s: &str) -> bool {
        !s.trim().is_empty()
    }
    
    /// 如果字符串为空则返回默认值
    pub fn default_if_empty(s: &str, default: &str) -> String {
        if Self::is_empty(s) {
            default.to_string()
        } else {
            s.to_string()
        }
    }
    
    /// 如果字符串为空白则返回默认值
    pub fn default_if_blank(s: &str, default: &str) -> String {
        if Self::is_blank(s) {
            default.to_string()
        } else {
            s.to_string()
        }
    }
    
    /// 驼峰转下划线
    pub fn camel_to_snake(s: &str) -> String {
        let re = Regex::new(r"([a-z])([A-Z])").unwrap();
        re.replace_all(s, "$1_$2").to_lowercase()
    }
    
    /// 下划线转驼峰
    pub fn snake_to_camel(s: &str) -> String {
        let parts: Vec<&str> = s.split('_').collect();
        let mut result = String::new();
        
        for (i, part) in parts.iter().enumerate() {
            if i == 0 {
                result.push_str(&part.to_lowercase());
            } else {
                let mut chars = part.chars();
                if let Some(first) = chars.next() {
                    result.push(first.to_uppercase().next().unwrap());
                    result.push_str(&chars.as_str().to_lowercase());
                }
            }
        }
        
        result
    }
    
    /// 首字母大写
    pub fn capitalize(s: &str) -> String {
        if s.is_empty() {
            return s.to_string();
        }
        
        let mut chars = s.chars();
        let first = chars.next().unwrap().to_uppercase().next().unwrap();
        let rest = chars.as_str();
        
        format!("{}{}", first, rest)
    }
    
    /// 首字母小写
    pub fn uncapitalize(s: &str) -> String {
        if s.is_empty() {
            return s.to_string();
        }
        
        let mut chars = s.chars();
        let first = chars.next().unwrap().to_lowercase().next().unwrap();
        let rest = chars.as_str();
        
        format!("{}{}", first, rest)
    }
    
    /// 重复字符串
    pub fn repeat(s: &str, count: usize) -> String {
        s.repeat(count)
    }
    
    /// 左填充
    pub fn left_pad(s: &str, length: usize, pad_char: char) -> String {
        if s.len() >= length {
            return s.to_string();
        }
        
        let pad_length = length - s.len();
        format!("{}{}", Self::repeat(&pad_char.to_string(), pad_length), s)
    }
    
    /// 右填充
    pub fn right_pad(s: &str, length: usize, pad_char: char) -> String {
        if s.len() >= length {
            return s.to_string();
        }
        
        let pad_length = length - s.len();
        format!("{}{}", s, Self::repeat(&pad_char.to_string(), pad_length))
    }
    
    /// 截取字符串
    pub fn substring(s: &str, start: usize, end: Option<usize>) -> String {
        let end = end.unwrap_or(s.len());
        if start >= s.len() || start >= end {
            return String::new();
        }
        
        let end = end.min(s.len());
        s[start..end].to_string()
    }
    
    /// 删除前后空白
    pub fn trim(s: &str) -> String {
        s.trim().to_string()
    }
    
    /// 删除前后指定字符
    pub fn trim_char(s: &str, c: char) -> String {
        s.trim_matches(c).to_string()
    }
    
    /// 替换字符串
    pub fn replace(s: &str, old: &str, new: &str) -> String {
        s.replace(old, new)
    }
    
    /// 替换所有匹配的字符串
    pub fn replace_all(s: &str, pattern: &str, replacement: &str) -> String {
        let re = Regex::new(pattern).unwrap();
        re.replace_all(s, replacement).to_string()
    }
    
    /// 分割字符串
    pub fn split(s: &str, delimiter: &str) -> Vec<String> {
        s.split(delimiter).map(|s| s.to_string()).collect()
    }
    
    /// 连接字符串
    pub fn join(strings: &[String], delimiter: &str) -> String {
        strings.join(delimiter)
    }
    
    /// 验证邮箱格式
    pub fn is_email(s: &str) -> bool {
        let re = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        re.is_match(s)
    }
    
    /// 验证手机号格式
    pub fn is_phone(s: &str) -> bool {
        let re = Regex::new(r"^1[3-9]\d{9}$").unwrap();
        re.is_match(s)
    }
    
    /// 验证身份证号格式
    pub fn is_id_card(s: &str) -> bool {
        let re = Regex::new(r"^\d{17}[\dXx]$").unwrap();
        re.is_match(s)
    }
    
    /// 验证用户名格式（字母数字下划线，2-20位）
    pub fn is_username(s: &str) -> bool {
        let re = Regex::new(r"^[a-zA-Z0-9_]{2,20}$").unwrap();
        re.is_match(s)
    }
    
    /// 验证密码格式（字母数字特殊字符，5-20位）
    pub fn is_password(s: &str) -> bool {
        let re = Regex::new(r"^[a-zA-Z0-9!@#$%^&*()_+\-=\[\]{};':\"\\|,.<>\/?]{5,20}$").unwrap();
        re.is_match(s)
    }
    
    /// 隐藏手机号中间4位
    pub fn mask_phone(phone: &str) -> String {
        if phone.len() == 11 {
            format!("{}****{}", &phone[..3], &phone[7..])
        } else {
            phone.to_string()
        }
    }
    
    /// 隐藏邮箱用户名
    pub fn mask_email(email: &str) -> String {
        if let Some(at_pos) = email.find('@') {
            let username = &email[..at_pos];
            let domain = &email[at_pos..];
            
            if username.len() <= 2 {
                format!("*{}", domain)
            } else {
                let visible = &username[..1];
                format!("{}*{}", visible, domain)
            }
        } else {
            email.to_string()
        }
    }
    
    /// 隐藏身份证号中间部分
    pub fn mask_id_card(id_card: &str) -> String {
        if id_card.len() == 18 {
            format!("{}****{}", &id_card[..6], &id_card[14..])
        } else {
            id_card.to_string()
        }
    }
}

