use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 基础实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseEntity {
    /// 创建者
    pub create_by: Option<String>,
    /// 创建时间
    pub create_time: Option<DateTime<Utc>>,
    /// 更新者
    pub update_by: Option<String>,
    /// 更新时间
    pub update_time: Option<DateTime<Utc>>,
    /// 备注
    pub remark: Option<String>,
}

impl Default for BaseEntity {
    fn default() -> Self {
        Self {
            create_by: None,
            create_time: Some(Utc::now()),
            update_by: None,
            update_time: Some(Utc::now()),
            remark: None,
        }
    }
}

/// 分页参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageDomain {
    /// 当前页码
    pub page_num: Option<i32>,
    /// 每页显示数量
    pub page_size: Option<i32>,
    /// 排序列
    pub order_by_column: Option<String>,
    /// 排序的方向 "desc" 或者 "asc"
    pub is_asc: Option<String>,
}

impl Default for PageDomain {
    fn default() -> Self {
        Self {
            page_num: Some(1),
            page_size: Some(10),
            order_by_column: None,
            is_asc: Some("asc".to_string()),
        }
    }
}

/// 分页结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageResult<T> {
    /// 当前页
    pub page_num: i32,
    /// 每页数量
    pub page_size: i32,
    /// 总记录数
    pub total: i64,
    /// 列表数据
    pub list: Vec<T>,
}

impl<T> PageResult<T> {
    /// 创建分页结果
    pub fn new(page_num: i32, page_size: i32, total: i64, list: Vec<T>) -> Self {
        Self {
            page_num,
            page_size,
            total,
            list,
        }
    }
    
    /// 获取总页数
    pub fn total_pages(&self) -> i32 {
        ((self.total as f64) / (self.page_size as f64)).ceil() as i32
    }
    
    /// 是否有下一页
    pub fn has_next(&self) -> bool {
        self.page_num < self.total_pages()
    }
    
    /// 是否有上一页
    pub fn has_prev(&self) -> bool {
        self.page_num > 1
    }
}

/// 登录用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginUser {
    /// 用户ID
    pub user_id: Uuid,
    /// 用户账号
    pub username: String,
    /// 用户昵称
    pub nick_name: String,
    /// 用户邮箱
    pub email: Option<String>,
    /// 手机号码
    pub phone: Option<String>,
    /// 用户性别
    pub sex: Option<String>,
    /// 用户头像
    pub avatar: Option<String>,
    /// 密码
    pub password: String,
    /// 盐加密
    pub salt: String,
    /// 帐号状态（0正常 1停用）
    pub status: String,
    /// 删除标志（0代表存在 2代表删除）
    pub del_flag: String,
    /// 最后登录IP
    pub login_ip: Option<String>,
    /// 最后登录时间
    pub login_date: Option<DateTime<Utc>>,
    /// 部门对象
    pub dept: Option<Dept>,
    /// 角色对象
    pub roles: Vec<Role>,
    /// 权限列表
    pub permissions: Vec<String>,
    /// 角色集合
    pub role_ids: Vec<Uuid>,
    /// 角色组
    pub role_keys: Vec<String>,
    /// 数据权限 当前角色ID
    pub role_id: Option<Uuid>,
}

/// 部门信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dept {
    /// 部门id
    pub dept_id: Uuid,
    /// 父部门id
    pub parent_id: Option<Uuid>,
    /// 祖级列表
    pub ancestors: Option<String>,
    /// 部门名称
    pub dept_name: String,
    /// 显示顺序
    pub order_num: i32,
    /// 负责人
    pub leader: Option<String>,
    /// 联系电话
    pub phone: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// 部门状态（0正常 1停用）
    pub status: String,
    /// 删除标志（0代表存在 2代表删除）
    pub del_flag: String,
    /// 父部门名称
    pub parent_name: Option<String>,
    /// 子部门
    pub children: Vec<Dept>,
}

/// 角色信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    /// 角色ID
    pub role_id: Uuid,
    /// 角色名称
    pub role_name: String,
    /// 角色权限
    pub role_key: String,
    /// 角色排序
    pub role_sort: i32,
    /// 数据范围（1：全部数据权限 2：自定数据权限 3：本部门数据权限 4：本部门及以下数据权限）
    pub data_scope: String,
    /// 菜单树选择项是否关联显示
    pub menu_check_strictly: bool,
    /// 部门树选择项是否关联显示
    pub dept_check_strictly: bool,
    /// 角色状态（0正常 1停用）
    pub status: String,
    /// 删除标志（0代表存在 2代表删除）
    pub del_flag: String,
    /// 备注
    pub remark: Option<String>,
    /// 用户是否存在此角色标识 默认不存在
    pub flag: Option<bool>,
    /// 菜单权限
    pub permissions: Vec<String>,
    /// 角色菜单权限
    pub role_menu_ids: Vec<Uuid>,
    /// 角色部门权限（数据权限）
    pub role_dept_ids: Vec<Uuid>,
}

/// 菜单信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Menu {
    /// 菜单ID
    pub menu_id: Uuid,
    /// 菜单名称
    pub menu_name: String,
    /// 父菜单ID
    pub parent_id: Option<Uuid>,
    /// 显示顺序
    pub order_num: i32,
    /// 路由地址
    pub path: Option<String>,
    /// 组件路径
    pub component: Option<String>,
    /// 路由参数
    pub query: Option<String>,
    /// 是否为外链（0是 1否）
    pub is_frame: i32,
    /// 是否缓存（0缓存 1不缓存）
    pub is_cache: i32,
    /// 菜单类型（M目录 C菜单 F按钮）
    pub menu_type: String,
    /// 菜单状态（0显示 1隐藏）
    pub visible: String,
    /// 菜单状态（0正常 1停用）
    pub status: String,
    /// 权限标识
    pub perms: Option<String>,
    /// 菜单图标
    pub icon: Option<String>,
    /// 子菜单
    pub children: Vec<Menu>,
}

/// 系统配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysConfig {
    /// 参数主键
    pub config_id: Uuid,
    /// 参数名称
    pub config_name: String,
    /// 参数键名
    pub config_key: String,
    /// 参数键值
    pub config_value: String,
    /// 系统内置（Y是 N否）
    pub config_type: String,
}

/// 字典数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictData {
    /// 字典编码
    pub dict_code: Uuid,
    /// 字典排序
    pub dict_sort: i32,
    /// 字典标签
    pub dict_label: String,
    /// 字典键值
    pub dict_value: String,
    /// 字典类型
    pub dict_type: String,
    /// 样式属性（其他样式扩展）
    pub css_class: Option<String>,
    /// 表格字典样式
    pub list_class: Option<String>,
    /// 是否默认（Y是 N否）
    pub is_default: String,
    /// 状态（0正常 1停用）
    pub status: String,
}

/// 字典类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictType {
    /// 字典主键
    pub dict_id: Uuid,
    /// 字典名称
    pub dict_name: String,
    /// 字典类型
    pub dict_type: String,
    /// 状态（0正常 1停用）
    pub status: String,
    /// 字典数据
    pub dict_data: Vec<DictData>,
}

/// 操作日志
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysOperLog {
    /// 日志主键
    pub oper_id: Uuid,
    /// 业务模块
    pub title: Option<String>,
    /// 方法名称
    pub method: Option<String>,
    /// 请求方式
    pub request_method: Option<String>,
    /// 操作类别（0其它 1新增 2修改 3删除）
    pub business_type: i32,
    /// 操作人员
    pub oper_name: Option<String>,
    /// 部门名称
    pub dept_name: Option<String>,
    /// 请求URL
    pub oper_url: Option<String>,
    /// 主机地址
    pub oper_ip: Option<String>,
    /// 操作地点
    pub oper_location: Option<String>,
    /// 请求参数
    pub oper_param: Option<String>,
    /// 返回参数
    pub json_result: Option<String>,
    /// 操作状态（0正常 1异常）
    pub status: i32,
    /// 错误消息
    pub error_msg: Option<String>,
    /// 操作时间
    pub oper_time: Option<DateTime<Utc>>,
}

/// 登录日志
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysLoginInfo {
    /// 访问ID
    pub info_id: Uuid,
    /// 用户账号
    pub username: Option<String>,
    /// 登录IP地址
    pub ipaddr: Option<String>,
    /// 登录地点
    pub login_location: Option<String>,
    /// 浏览器类型
    pub browser: Option<String>,
    /// 操作系统
    pub os: Option<String>,
    /// 登录状态（0成功 1失败）
    pub status: Option<String>,
    /// 提示消息
    pub msg: Option<String>,
    /// 访问时间
    pub login_time: Option<DateTime<Utc>>,
}

/// 在线用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysUserOnline {
    /// 会话编号
    pub token_id: String,
    /// 部门名称
    pub dept_name: Option<String>,
    /// 用户名称
    pub user_name: String,
    /// 登录IP地址
    pub ipaddr: String,
    /// 登录地址
    pub login_location: Option<String>,
    /// 浏览器类型
    pub browser: Option<String>,
    /// 操作系统
    pub os: Option<String>,
    /// 登录时间
    pub login_time: DateTime<Utc>,
}

