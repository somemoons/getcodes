use serde::{Deserialize, Serialize};

/// 用户状态枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserStatus {
    /// 正常
    Normal = 0,
    /// 停用
    Disabled = 1,
}

impl From<i32> for UserStatus {
    fn from(value: i32) -> Self {
        match value {
            0 => UserStatus::Normal,
            1 => UserStatus::Disabled,
            _ => UserStatus::Disabled,
        }
    }
}

impl From<UserStatus> for i32 {
    fn from(status: UserStatus) -> Self {
        status as i32
    }
}

/// 删除标志枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DelFlag {
    /// 存在
    Exists = 0,
    /// 删除
    Deleted = 2,
}

impl From<i32> for DelFlag {
    fn from(value: i32) -> Self {
        match value {
            0 => DelFlag::Exists,
            2 => DelFlag::Deleted,
            _ => DelFlag::Exists,
        }
    }
}

impl From<DelFlag> for i32 {
    fn from(flag: DelFlag) -> Self {
        flag as i32
    }
}

/// 菜单类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MenuType {
    /// 目录
    Directory = 0,
    /// 菜单
    Menu = 1,
    /// 按钮
    Button = 2,
}

impl From<i32> for MenuType {
    fn from(value: i32) -> Self {
        match value {
            0 => MenuType::Directory,
            1 => MenuType::Menu,
            2 => MenuType::Button,
            _ => MenuType::Directory,
        }
    }
}

impl From<MenuType> for i32 {
    fn from(menu_type: MenuType) -> Self {
        menu_type as i32
    }
}

/// 业务类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BusinessType {
    /// 其它
    Other = 0,
    /// 新增
    Insert = 1,
    /// 修改
    Update = 2,
    /// 删除
    Delete = 3,
    /// 授权
    Grant = 4,
    /// 导出
    Export = 5,
    /// 导入
    Import = 6,
    /// 强退
    Force = 7,
    /// 生成代码
    GenCode = 8,
    /// 清空数据
    Clean = 9,
}

impl From<i32> for BusinessType {
    fn from(value: i32) -> Self {
        match value {
            0 => BusinessType::Other,
            1 => BusinessType::Insert,
            2 => BusinessType::Update,
            3 => BusinessType::Delete,
            4 => BusinessType::Grant,
            5 => BusinessType::Export,
            6 => BusinessType::Import,
            7 => BusinessType::Force,
            8 => BusinessType::GenCode,
            9 => BusinessType::Clean,
            _ => BusinessType::Other,
        }
    }
}

impl From<BusinessType> for i32 {
    fn from(business_type: BusinessType) -> Self {
        business_type as i32
    }
}

/// 操作状态枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OperStatus {
    /// 正常
    Success = 0,
    /// 异常
    Fail = 1,
}

impl From<i32> for OperStatus {
    fn from(value: i32) -> Self {
        match value {
            0 => OperStatus::Success,
            1 => OperStatus::Fail,
            _ => OperStatus::Fail,
        }
    }
}

impl From<OperStatus> for i32 {
    fn from(status: OperStatus) -> Self {
        status as i32
    }
}

/// 登录状态枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoginStatus {
    /// 成功
    Success = 0,
    /// 失败
    Fail = 1,
}

impl From<i32> for LoginStatus {
    fn from(value: i32) -> Self {
        match value {
            0 => LoginStatus::Success,
            1 => LoginStatus::Fail,
            _ => LoginStatus::Fail,
        }
    }
}

impl From<LoginStatus> for i32 {
    fn from(status: LoginStatus) -> Self {
        status as i32
    }
}

/// 数据范围枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataScope {
    /// 全部数据权限
    All = 1,
    /// 自定数据权限
    Custom = 2,
    /// 本部门数据权限
    Dept = 3,
    /// 本部门及以下数据权限
    DeptAndChild = 4,
    /// 仅本人数据权限
    Self = 5,
}

impl From<i32> for DataScope {
    fn from(value: i32) -> Self {
        match value {
            1 => DataScope::All,
            2 => DataScope::Custom,
            3 => DataScope::Dept,
            4 => DataScope::DeptAndChild,
            5 => DataScope::Self,
            _ => DataScope::Self,
        }
    }
}

impl From<DataScope> for i32 {
    fn from(scope: DataScope) -> Self {
        scope as i32
    }
}

/// 老人状态枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ElderStatus {
    /// 禁用
    Disabled = 0,
    /// 启用
    Enabled = 1,
    /// 请假
    Leave = 2,
    /// 退住中
    Leaving = 3,
    /// 入住中
    CheckedIn = 4,
    /// 已退住
    CheckedOut = 5,
}

impl From<i32> for ElderStatus {
    fn from(value: i32) -> Self {
        match value {
            0 => ElderStatus::Disabled,
            1 => ElderStatus::Enabled,
            2 => ElderStatus::Leave,
            3 => ElderStatus::Leaving,
            4 => ElderStatus::CheckedIn,
            5 => ElderStatus::CheckedOut,
            _ => ElderStatus::Disabled,
        }
    }
}

impl From<ElderStatus> for i32 {
    fn from(status: ElderStatus) -> Self {
        status as i32
    }
}

/// 性别枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Gender {
    /// 女
    Female = 0,
    /// 男
    Male = 1,
}

impl From<i32> for Gender {
    fn from(value: i32) -> Self {
        match value {
            0 => Gender::Female,
            1 => Gender::Male,
            _ => Gender::Female,
        }
    }
}

impl From<Gender> for i32 {
    fn from(gender: Gender) -> Self {
        gender as i32
    }
}

