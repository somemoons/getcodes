use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use zzyl_common::{BaseEntity, enums::{UserStatus, DelFlag}};

/// 用户实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysUser {
    /// 用户ID
    pub user_id: Option<Uuid>,
    /// 用户账号
    pub username: String,
    /// 用户昵称
    pub nick_name: Option<String>,
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
    pub salt: Option<String>,
    /// 帐号状态（0正常 1停用）
    pub status: UserStatus,
    /// 删除标志（0代表存在 2代表删除）
    pub del_flag: DelFlag,
    /// 最后登录IP
    pub login_ip: Option<String>,
    /// 最后登录时间
    pub login_date: Option<DateTime<Utc>>,
    /// 部门ID
    pub dept_id: Option<Uuid>,
    /// 基础实体
    #[serde(flatten)]
    pub base: BaseEntity,
}

/// 角色实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysRole {
    /// 角色ID
    pub role_id: Option<Uuid>,
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
    /// 基础实体
    #[serde(flatten)]
    pub base: BaseEntity,
}

/// 菜单实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysMenu {
    /// 菜单ID
    pub menu_id: Option<Uuid>,
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
    /// 基础实体
    #[serde(flatten)]
    pub base: BaseEntity,
}

/// 部门实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysDept {
    /// 部门id
    pub dept_id: Option<Uuid>,
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
    /// 基础实体
    #[serde(flatten)]
    pub base: BaseEntity,
}

/// 岗位实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysPost {
    /// 岗位ID
    pub post_id: Option<Uuid>,
    /// 岗位编码
    pub post_code: String,
    /// 岗位名称
    pub post_name: String,
    /// 岗位排序
    pub post_sort: i32,
    /// 状态（0正常 1停用）
    pub status: String,
    /// 基础实体
    #[serde(flatten)]
    pub base: BaseEntity,
}

/// 字典类型实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysDictType {
    /// 字典主键
    pub dict_id: Option<Uuid>,
    /// 字典名称
    pub dict_name: String,
    /// 字典类型
    pub dict_type: String,
    /// 状态（0正常 1停用）
    pub status: String,
    /// 基础实体
    #[serde(flatten)]
    pub base: BaseEntity,
}

/// 字典数据实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysDictData {
    /// 字典编码
    pub dict_code: Option<Uuid>,
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
    /// 基础实体
    #[serde(flatten)]
    pub base: BaseEntity,
}

/// 系统配置实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysConfig {
    /// 参数主键
    pub config_id: Option<Uuid>,
    /// 参数名称
    pub config_name: String,
    /// 参数键名
    pub config_key: String,
    /// 参数键值
    pub config_value: String,
    /// 系统内置（Y是 N否）
    pub config_type: String,
    /// 基础实体
    #[serde(flatten)]
    pub base: BaseEntity,
}

/// 操作日志实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysOperLog {
    /// 日志主键
    pub oper_id: Option<Uuid>,
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

/// 登录日志实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysLoginInfo {
    /// 访问ID
    pub info_id: Option<Uuid>,
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

/// 在线用户实体
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

/// 用户角色关联实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysUserRole {
    /// 用户ID
    pub user_id: Uuid,
    /// 角色ID
    pub role_id: Uuid,
}

/// 用户岗位关联实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysUserPost {
    /// 用户ID
    pub user_id: Uuid,
    /// 岗位ID
    pub post_id: Uuid,
}

/// 角色菜单关联实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysRoleMenu {
    /// 角色ID
    pub role_id: Uuid,
    /// 菜单ID
    pub menu_id: Uuid,
}

/// 角色部门关联实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysRoleDept {
    /// 角色ID
    pub role_id: Uuid,
    /// 部门ID
    pub dept_id: Uuid,
}

/// 登录请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    /// 用户名
    pub username: String,
    /// 密码
    pub password: String,
    /// 验证码
    pub captcha: String,
    /// 验证码ID
    pub uuid: String,
    /// 记住我
    pub remember_me: Option<bool>,
}

/// 登录响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    /// 访问令牌
    pub access_token: String,
    /// 令牌类型
    pub token_type: String,
    /// 过期时间
    pub expires_in: i64,
    /// 用户信息
    pub user_info: SysUser,
}

/// 用户信息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfoResponse {
    /// 用户信息
    pub user: SysUser,
    /// 角色列表
    pub roles: Vec<SysRole>,
    /// 权限列表
    pub permissions: Vec<String>,
    /// 菜单列表
    pub menus: Vec<SysMenu>,
}

/// 修改密码请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangePasswordRequest {
    /// 旧密码
    pub old_password: String,
    /// 新密码
    pub new_password: String,
}

/// 用户查询条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSearchCondition {
    /// 用户名
    pub username: Option<String>,
    /// 昵称
    pub nick_name: Option<String>,
    /// 手机号
    pub phone: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// 性别
    pub sex: Option<String>,
    /// 状态
    pub status: Option<UserStatus>,
    /// 部门ID
    pub dept_id: Option<Uuid>,
}

/// 角色查询条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleSearchCondition {
    /// 角色名称
    pub role_name: Option<String>,
    /// 角色权限
    pub role_key: Option<String>,
    /// 状态
    pub status: Option<String>,
}

/// 菜单查询条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuSearchCondition {
    /// 菜单名称
    pub menu_name: Option<String>,
    /// 菜单类型
    pub menu_type: Option<String>,
    /// 状态
    pub status: Option<String>,
    /// 父菜单ID
    pub parent_id: Option<Uuid>,
}

/// 部门查询条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeptSearchCondition {
    /// 部门名称
    pub dept_name: Option<String>,
    /// 状态
    pub status: Option<String>,
    /// 父部门ID
    pub parent_id: Option<Uuid>,
}

