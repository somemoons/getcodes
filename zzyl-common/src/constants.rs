/// 通用常量定义
pub mod constants {
    /// 成功标记
    pub const SUCCESS: i32 = 200;
    
    /// 失败标记
    pub const FAIL: i32 = 500;
    
    /// 登录成功
    pub const LOGIN_SUCCESS: &str = "登录成功";
    
    /// 退出成功
    pub const LOGOUT: &str = "退出成功";
    
    /// 注册成功
    pub const REGISTER: &str = "注册成功";
    
    /// 操作成功
    pub const OPERATION_SUCCESS: &str = "操作成功";
    
    /// 操作失败
    pub const OPERATION_FAIL: &str = "操作失败";
    
    /// 数据权限
    pub const DATA_SCOPE_ALL: &str = "1";
    pub const DATA_SCOPE_CUSTOM: &str = "2";
    pub const DATA_SCOPE_DEPT: &str = "3";
    pub const DATA_SCOPE_DEPT_AND_CHILD: &str = "4";
    pub const DATA_SCOPE_SELF: &str = "5";
    
    /// 菜单类型
    pub const TYPE_DIR: &str = "M"; // 目录
    pub const TYPE_MENU: &str = "C"; // 菜单
    pub const TYPE_BUTTON: &str = "F"; // 按钮
    
    /// Layout组件标识
    pub const LAYOUT: &str = "Layout";
    pub const PARENT_VIEW: &str = "ParentView";
    pub const INNER_LINK: &str = "InnerLink";
    
    /// 用户名长度限制
    pub const USERNAME_MIN_LENGTH: usize = 2;
    pub const USERNAME_MAX_LENGTH: usize = 20;
    
    /// 密码长度限制
    pub const PASSWORD_MIN_LENGTH: usize = 5;
    pub const PASSWORD_MAX_LENGTH: usize = 20;
    
    /// 默认页码
    pub const PAGE_NUM: i32 = 1;
    
    /// 默认页面大小
    pub const PAGE_SIZE: i32 = 10;
    
    /// 最大页面大小
    pub const MAX_PAGE_SIZE: i32 = 500;
    
    /// 验证码
    pub const CAPTCHA_CODE_KEY: &str = "captcha_codes:";
    pub const CAPTCHA_EXPIRATION: i64 = 2; // 验证码有效期(分钟)
    
    /// 令牌
    pub const TOKEN: &str = "token";
    pub const LOGIN_USER_KEY: &str = "login_user_key";
    
    /// 参数管理 cache key
    pub const SYS_CONFIG_KEY: &str = "sys_config:";
    
    /// 字典管理 cache key
    pub const SYS_DICT_KEY: &str = "sys_dict:";
    
    /// 资源映射路径 前缀
    pub const RESOURCE_PREFIX: &str = "/profile";
    
    /// RMI 远程方法调用
    pub const LOOKUP_RMI: &str = "rmi://";
    
    /// LDAP 远程方法调用
    pub const LOOKUP_LDAP: &str = "ldap://";
    
    /// LDAPS 远程方法调用
    pub const LOOKUP_LDAPS: &str = "ldaps://";
    
    /// 定时任务白名单配置
    pub const JOB_WHITELIST_STR: &str = "ruoyi,zzyl";
    
    /// 定时任务违规的字符
    pub const JOB_ERROR_STR: &str = "java.lang.management.ManagementFactory";
    
    /// 是否唯一
    pub const UNIQUE: bool = true;
    pub const NOT_UNIQUE: bool = false;
    
    /// 是否菜单外链
    pub const YES_FRAME: i32 = 0;
    pub const NO_FRAME: i32 = 1;
    
    /// 菜单类型（目录）
    pub const TYPE_DIRECTORY: &str = "M";
    
    /// 菜单类型（菜单）
    pub const TYPE_MENU_ITEM: &str = "C";
    
    /// 菜单类型（按钮）
    pub const TYPE_BUTTON_ITEM: &str = "F";
    
    /// Layout组件标识
    pub const LAYOUT_COMPONENT: &str = "Layout";
    pub const PARENT_VIEW_COMPONENT: &str = "ParentView";
    pub const INNER_LINK_COMPONENT: &str = "InnerLink";
    
    /// 用户正常状态
    pub const NORMAL: &str = "0";
    
    /// 用户封禁状态
    pub const EXCEPTION: &str = "1";
    
    /// 角色正常状态
    pub const ROLE_NORMAL: &str = "0";
    
    /// 角色封禁状态
    pub const ROLE_EXCEPTION: &str = "1";
    
    /// 部门正常状态
    pub const DEPT_NORMAL: &str = "0";
    
    /// 部门停用状态
    pub const DEPT_DISABLE: &str = "1";
    
    /// 字典正常状态
    pub const DICT_NORMAL: &str = "0";
    
    /// 是否为系统默认（是）
    pub const YES: &str = "Y";
    
    /// 是否菜单外链（是）
    pub const YES_FRAME_STR: &str = "0";
    
    /// 是否菜单外链（否）
    pub const NO_FRAME_STR: &str = "1";
    
    /// 菜单显示状态
    pub const MENU_SHOW: &str = "0";
    
    /// 菜单隐藏状态
    pub const MENU_HIDE: &str = "1";
    
    /// 菜单缓存标识
    pub const CACHE_MENU_KEY: &str = "menu_key";
    
    /// 用户缓存
    pub const CACHE_USER_KEY: &str = "user_key";
    
    /// 用户缓存标识
    pub const CACHE_USER_DETAILS_KEY: &str = "user_details_key";
    
    /// 用户会话
    pub const CACHE_USER_SESSION_KEY: &str = "user_session_key";
    
    /// 在线用户
    pub const ONLINE_USER_KEY: &str = "online_user_key";
    
    /// 登录地点
    pub const LOGININFO_CACHE_KEY: &str = "logininfo_cache_key";
    
    /// 系统用户授权缓存
    pub const SYS_AUTH_CACHE: &str = "sys-authCache";
    
    /// 参数缓存
    pub const SYS_CONFIG_CACHE: &str = "sys-config";
    
    /// 字典缓存
    pub const SYS_DICT_CACHE: &str = "sys-dict";
    
    /// 防重提交
    pub const REPEAT_SUBMIT_KEY: &str = "repeat_submit:";
    
    /// 限流 redis key
    pub const RATE_LIMIT_KEY: &str = "rate_limit:";
    
    /// 登录失败次数
    pub const PWD_ERR_CNT_KEY: &str = "pwd_err_cnt:";
    
    /// 登录失败锁定时间
    pub const PWD_ERR_LOCK_TIME: i64 = 10; // 10分钟
    
    /// 登录失败最大次数
    pub const PWD_ERR_MAX_CNT: i32 = 5;
    
    /// 令牌前缀
    pub const TOKEN_PREFIX: &str = "Bearer ";
    
    /// 令牌前缀
    pub const LOGIN_TOKEN_KEY: &str = "login_tokens:";
    
    /// 用户ID字段
    pub const DETAILS_USER_ID: &str = "user_id";
    
    /// 用户名字段
    pub const DETAILS_USERNAME: &str = "username";
    
    /// 授权信息字段
    pub const AUTHORIZATION_HEADER: &str = "authorization";
    
    /// 验证码有效期（分钟）
    pub const CAPTCHA_EXPIRATION_MINUTES: i64 = 2;
    
    /// 验证码 redis key
    pub const CAPTCHA_CODE_KEY_PREFIX: &str = "captcha_codes:";
    
    /// 登录用户 redis key
    pub const LOGIN_USER_KEY_PREFIX: &str = "login_user_key:";
    
    /// 防重提交 redis key
    pub const REPEAT_SUBMIT_KEY_PREFIX: &str = "repeat_submit:";
    
    /// 限流 redis key
    pub const RATE_LIMIT_KEY_PREFIX: &str = "rate_limit:";
    
    /// 登录失败次数 redis key
    pub const PWD_ERR_CNT_KEY_PREFIX: &str = "pwd_err_cnt:";
    
    /// 登录令牌 redis key
    pub const LOGIN_TOKEN_KEY_PREFIX: &str = "login_tokens:";
}

