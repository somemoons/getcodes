use serde::{Deserialize, Serialize};

/// Excel注解
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Excel {
    /// 导出到Excel中的名字
    pub name: Option<String>,
    /// 日期格式，如：yyyy-MM-dd
    pub date_format: Option<String>,
    /// 如果是字典类型，请设置字典的type值，如：sys_user_sex
    pub dict_type: Option<String>,
    /// 读取内容转表达式，如：0=男,1=女,2=未知
    pub read_converter_exp: Option<String>,
    /// 分隔符，读取字符串组内容
    pub separator: Option<String>,
    /// 导出类型（0数字 1字符串）
    pub cell_type: Option<i32>,
    /// 导出时在excel中每个列的高度，单位为字符
    pub height: Option<i32>,
    /// 导出时在excel中每个列的宽度，单位为字符
    pub width: Option<i32>,
    /// 文字后缀，如：% 90变成90%
    pub suffix: Option<String>,
    /// 当值为空时，字段的默认值
    pub default_value: Option<String>,
    /// 提示信息
    pub prompt: Option<String>,
    /// 设置只能选择不能输入的列内容
    pub combo: Option<Vec<String>>,
    /// 是否导出数据，应对需求：有时我们需要导出一份模板，这是标题需要但内容需要用户手工填写
    pub is_export: Option<bool>,
    /// 另一个类中的属性名称，支持多级获取，以小数点隔开
    pub target_attr: Option<String>,
    /// 是否自动统计数据，在最后追加一行统计数据
    pub is_statistics: Option<bool>,
}

impl Default for Excel {
    fn default() -> Self {
        Self {
            name: None,
            date_format: None,
            dict_type: None,
            read_converter_exp: None,
            separator: None,
            cell_type: None,
            height: None,
            width: None,
            suffix: None,
            default_value: None,
            prompt: None,
            combo: None,
            is_export: Some(true),
            target_attr: None,
            is_statistics: None,
        }
    }
}

/// Excel注解宏
#[macro_export]
macro_rules! excel {
    ($name:expr) => {
        Excel {
            name: Some($name.to_string()),
            ..Default::default()
        }
    };
    ($name:expr, $date_format:expr) => {
        Excel {
            name: Some($name.to_string()),
            date_format: Some($date_format.to_string()),
            ..Default::default()
        }
    };
    ($name:expr, $date_format:expr, $dict_type:expr) => {
        Excel {
            name: Some($name.to_string()),
            date_format: Some($date_format.to_string()),
            dict_type: Some($dict_type.to_string()),
            ..Default::default()
        }
    };
}

/// 数据权限注解
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataScope {
    /// 用户表的别名
    pub user_alias: Option<String>,
    /// 权限字符
    pub dept_alias: Option<String>,
}

impl Default for DataScope {
    fn default() -> Self {
        Self {
            user_alias: None,
            dept_alias: None,
        }
    }
}

/// 数据权限注解宏
#[macro_export]
macro_rules! data_scope {
    () => {
        DataScope::default()
    };
    ($user_alias:expr) => {
        DataScope {
            user_alias: Some($user_alias.to_string()),
            dept_alias: None,
        }
    };
    ($user_alias:expr, $dept_alias:expr) => {
        DataScope {
            user_alias: Some($user_alias.to_string()),
            dept_alias: Some($dept_alias.to_string()),
        }
    };
}

/// 重复提交注解
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepeatSubmit {
    /// 间隔时间，单位毫秒
    pub interval: Option<i64>,
    /// 提示消息
    pub message: Option<String>,
}

impl Default for RepeatSubmit {
    fn default() -> Self {
        Self {
            interval: Some(5000), // 默认5秒
            message: Some("不允许重复提交，请稍候再试".to_string()),
        }
    }
}

/// 重复提交注解宏
#[macro_export]
macro_rules! repeat_submit {
    () => {
        RepeatSubmit::default()
    };
    ($interval:expr) => {
        RepeatSubmit {
            interval: Some($interval),
            message: Some("不允许重复提交，请稍候再试".to_string()),
        }
    };
    ($interval:expr, $message:expr) => {
        RepeatSubmit {
            interval: Some($interval),
            message: Some($message.to_string()),
        }
    };
}

/// 日志注解
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Log {
    /// 模块
    pub title: Option<String>,
    /// 功能
    pub business_type: Option<i32>,
    /// 操作人类别
    pub operator_type: Option<i32>,
    /// 是否保存请求的参数
    pub is_save_request_data: Option<bool>,
    /// 是否保存响应的参数
    pub is_save_response_data: Option<bool>,
}

impl Default for Log {
    fn default() -> Self {
        Self {
            title: None,
            business_type: None,
            operator_type: None,
            is_save_request_data: Some(true),
            is_save_response_data: Some(true),
        }
    }
}

/// 日志注解宏
#[macro_export]
macro_rules! log {
    ($title:expr) => {
        Log {
            title: Some($title.to_string()),
            ..Default::default()
        }
    };
    ($title:expr, $business_type:expr) => {
        Log {
            title: Some($title.to_string()),
            business_type: Some($business_type),
            ..Default::default()
        }
    };
    ($title:expr, $business_type:expr, $operator_type:expr) => {
        Log {
            title: Some($title.to_string()),
            business_type: Some($business_type),
            operator_type: Some($operator_type),
            ..Default::default()
        }
    };
}

/// 验证注解
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Valid {
    /// 验证组
    pub groups: Option<Vec<String>>,
}

impl Default for Valid {
    fn default() -> Self {
        Self {
            groups: None,
        }
    }
}

/// 验证注解宏
#[macro_export]
macro_rules! valid {
    () => {
        Valid::default()
    };
    ($($group:expr),*) => {
        Valid {
            groups: Some(vec![$($group.to_string()),*]),
        }
    };
}

/// 预授权注解
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreAuthorize {
    /// 权限字符串
    pub has_perm: Option<String>,
    /// 角色权限
    pub has_role: Option<String>,
    /// 逻辑 AND/OR
    pub logical: Option<String>,
}

impl Default for PreAuthorize {
    fn default() -> Self {
        Self {
            has_perm: None,
            has_role: None,
            logical: Some("AND".to_string()),
        }
    }
}

/// 预授权注解宏
#[macro_export]
macro_rules! pre_authorize {
    ($has_perm:expr) => {
        PreAuthorize {
            has_perm: Some($has_perm.to_string()),
            ..Default::default()
        }
    };
    ($has_perm:expr, $has_role:expr) => {
        PreAuthorize {
            has_perm: Some($has_perm.to_string()),
            has_role: Some($has_role.to_string()),
            ..Default::default()
        }
    };
    ($has_perm:expr, $has_role:expr, $logical:expr) => {
        PreAuthorize {
            has_perm: Some($has_perm.to_string()),
            has_role: Some($has_role.to_string()),
            logical: Some($logical.to_string()),
        }
    };
}

/// 匿名访问注解
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Anonymous;

impl Default for Anonymous {
    fn default() -> Self {
        Self
    }
}

/// 匿名访问注解宏
#[macro_export]
macro_rules! anonymous {
    () => {
        Anonymous::default()
    };
}

