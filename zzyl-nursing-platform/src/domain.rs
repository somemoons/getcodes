use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use zzyl_common::{BaseEntity, enums::{ElderStatus, Gender}};

/// 老人实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Elder {
    /// 主键ID
    pub id: Option<Uuid>,
    /// 名称
    pub name: String,
    /// 图片
    pub image: Option<String>,
    /// 身份证号
    pub id_card_no: String,
    /// 性别（0:女 1:男）
    pub sex: Gender,
    /// 状态（0：禁用，1:启用 2:请假 3:退住中 4入住中 5已退住）
    pub status: ElderStatus,
    /// 手机号
    pub phone: Option<String>,
    /// 出生日期
    pub birthday: Option<DateTime<Utc>>,
    /// 家庭住址
    pub address: Option<String>,
    /// 身份证国徽面
    pub id_card_front: Option<String>,
    /// 身份证人像面
    pub id_card_back: Option<String>,
    /// 基础实体
    #[serde(flatten)]
    pub base: BaseEntity,
}

/// 护理计划实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NursingPlan {
    /// 编号
    pub id: Option<Uuid>,
    /// 排序号
    pub sort_no: Option<i32>,
    /// 名称
    pub plan_name: String,
    /// 状态 0禁用 1启用
    pub status: i32,
    /// 基础实体
    #[serde(flatten)]
    pub base: BaseEntity,
}

/// 护理项目实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NursingProject {
    /// 编号
    pub id: Option<Uuid>,
    /// 名称
    pub name: String,
    /// 排序号
    pub order_no: Option<i32>,
    /// 单位
    pub unit: Option<String>,
    /// 价格
    pub price: Option<Decimal>,
    /// 图片
    pub image: Option<String>,
    /// 护理要求
    pub nursing_requirement: Option<String>,
    /// 状态（0：禁用，1：启用）
    pub status: i32,
    /// 基础实体
    #[serde(flatten)]
    pub base: BaseEntity,
}

/// 护理等级实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NursingLevel {
    /// 主键ID
    pub id: Option<Uuid>,
    /// 等级名称
    pub name: String,
    /// 护理计划ID
    pub plan_id: Option<Uuid>,
    /// 护理费用
    pub fee: Option<Decimal>,
    /// 状态（0：禁用，1：启用）
    pub status: i32,
    /// 等级说明
    pub description: Option<String>,
    /// 基础实体
    #[serde(flatten)]
    pub base: BaseEntity,
}

/// 护理项目计划关联实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NursingProjectPlan {
    /// 主键ID
    pub id: Option<Uuid>,
    /// 护理项目ID
    pub project_id: Option<Uuid>,
    /// 护理计划ID
    pub plan_id: Option<Uuid>,
    /// 基础实体
    #[serde(flatten)]
    pub base: BaseEntity,
}

/// 护理员老人关联实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NursingElder {
    /// id
    pub id: Option<Uuid>,
    /// 护理员id
    pub nursing_id: Option<Uuid>,
    /// 老人id
    pub elder_id: Option<Uuid>,
    /// 基础实体
    #[serde(flatten)]
    pub base: BaseEntity,
}

/// 床位实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bed {
    /// 主键ID
    pub id: Option<Uuid>,
    /// 床位编号
    pub bed_no: String,
    /// 房间ID
    pub room_id: Option<Uuid>,
    /// 状态（0：空闲，1：占用）
    pub status: i32,
    /// 基础实体
    #[serde(flatten)]
    pub base: BaseEntity,
}

/// 房间实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
    /// 主键ID
    pub id: Option<Uuid>,
    /// 房间号
    pub room_no: String,
    /// 楼层ID
    pub floor_id: Option<Uuid>,
    /// 房间类型ID
    pub room_type_id: Option<Uuid>,
    /// 状态（0：空闲，1：占用）
    pub status: i32,
    /// 基础实体
    #[serde(flatten)]
    pub base: BaseEntity,
}

/// 楼层实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Floor {
    /// 主键ID
    pub id: Option<Uuid>,
    /// 楼层名称
    pub floor_name: String,
    /// 楼层号
    pub floor_no: i32,
    /// 状态（0：禁用，1：启用）
    pub status: i32,
    /// 基础实体
    #[serde(flatten)]
    pub base: BaseEntity,
}

/// 房间类型实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomType {
    /// 主键ID
    pub id: Option<Uuid>,
    /// 类型名称
    pub type_name: String,
    /// 价格
    pub price: Option<Decimal>,
    /// 状态（0：禁用，1：启用）
    pub status: i32,
    /// 基础实体
    #[serde(flatten)]
    pub base: BaseEntity,
}

/// 入住记录实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckIn {
    /// 主键ID
    pub id: Option<Uuid>,
    /// 老人ID
    pub elder_id: Option<Uuid>,
    /// 床位ID
    pub bed_id: Option<Uuid>,
    /// 入住时间
    pub check_in_time: Option<DateTime<Utc>>,
    /// 退住时间
    pub check_out_time: Option<DateTime<Utc>>,
    /// 状态（0：入住中，1：已退住）
    pub status: i32,
    /// 基础实体
    #[serde(flatten)]
    pub base: BaseEntity,
}

/// 合同实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contract {
    /// 主键ID
    pub id: Option<Uuid>,
    /// 合同编号
    pub contract_no: String,
    /// 老人ID
    pub elder_id: Option<Uuid>,
    /// 合同开始时间
    pub start_time: Option<DateTime<Utc>>,
    /// 合同结束时间
    pub end_time: Option<DateTime<Utc>>,
    /// 合同金额
    pub amount: Option<Decimal>,
    /// 状态（0：有效，1：无效）
    pub status: i32,
    /// 基础实体
    #[serde(flatten)]
    pub base: BaseEntity,
}

/// 家庭成员实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FamilyMember {
    /// 主键ID
    pub id: Option<Uuid>,
    /// 老人ID
    pub elder_id: Option<Uuid>,
    /// 成员姓名
    pub member_name: String,
    /// 关系
    pub relationship: String,
    /// 手机号
    pub phone: Option<String>,
    /// 基础实体
    #[serde(flatten)]
    pub base: BaseEntity,
}

/// 入住配置实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckInConfig {
    /// 主键ID
    pub id: Option<Uuid>,
    /// 配置名称
    pub config_name: String,
    /// 配置值
    pub config_value: String,
    /// 配置类型
    pub config_type: String,
    /// 状态（0：禁用，1：启用）
    pub status: i32,
    /// 基础实体
    #[serde(flatten)]
    pub base: BaseEntity,
}

/// 护理计划DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NursingPlanDto {
    /// 护理计划
    pub plan: NursingPlan,
    /// 护理项目列表
    pub projects: Vec<NursingProject>,
}

/// 老人详细信息DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElderDetailDto {
    /// 老人信息
    pub elder: Elder,
    /// 床位信息
    pub bed: Option<Bed>,
    /// 房间信息
    pub room: Option<Room>,
    /// 楼层信息
    pub floor: Option<Floor>,
    /// 家庭成员列表
    pub family_members: Vec<FamilyMember>,
    /// 合同信息
    pub contract: Option<Contract>,
}

/// 房间详细信息DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomDetailDto {
    /// 房间信息
    pub room: Room,
    /// 楼层信息
    pub floor: Option<Floor>,
    /// 房间类型信息
    pub room_type: Option<RoomType>,
    /// 床位列表
    pub beds: Vec<Bed>,
    /// 入住老人列表
    pub elders: Vec<Elder>,
}

/// 楼层详细信息DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FloorDetailDto {
    /// 楼层信息
    pub floor: Floor,
    /// 房间列表
    pub rooms: Vec<Room>,
    /// 入住统计
    pub occupancy_stats: OccupancyStats,
}

/// 入住统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OccupancyStats {
    /// 总床位数量
    pub total_beds: i32,
    /// 已占用床位数量
    pub occupied_beds: i32,
    /// 空闲床位数量
    pub available_beds: i32,
    /// 入住率
    pub occupancy_rate: f64,
}

/// 护理统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NursingStats {
    /// 总老人数量
    pub total_elders: i32,
    /// 入住老人数量
    pub checked_in_elders: i32,
    /// 请假老人数量
    pub on_leave_elders: i32,
    /// 退住老人数量
    pub checked_out_elders: i32,
    /// 护理员数量
    pub total_nurses: i32,
    /// 护理项目数量
    pub total_projects: i32,
}

/// 费用统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostStats {
    /// 总费用
    pub total_cost: Decimal,
    /// 房间费用
    pub room_cost: Decimal,
    /// 护理费用
    pub nursing_cost: Decimal,
    /// 其他费用
    pub other_cost: Decimal,
    /// 平均费用
    pub average_cost: Decimal,
}

/// 老人搜索条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElderSearchCondition {
    /// 姓名
    pub name: Option<String>,
    /// 身份证号
    pub id_card_no: Option<String>,
    /// 手机号
    pub phone: Option<String>,
    /// 性别
    pub sex: Option<Gender>,
    /// 状态
    pub status: Option<ElderStatus>,
    /// 楼层ID
    pub floor_id: Option<Uuid>,
    /// 房间ID
    pub room_id: Option<Uuid>,
    /// 床位ID
    pub bed_id: Option<Uuid>,
}

/// 房间搜索条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomSearchCondition {
    /// 房间号
    pub room_no: Option<String>,
    /// 楼层ID
    pub floor_id: Option<Uuid>,
    /// 房间类型ID
    pub room_type_id: Option<Uuid>,
    /// 状态
    pub status: Option<i32>,
}

/// 护理计划搜索条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NursingPlanSearchCondition {
    /// 计划名称
    pub plan_name: Option<String>,
    /// 状态
    pub status: Option<i32>,
}

/// 护理项目搜索条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NursingProjectSearchCondition {
    /// 项目名称
    pub name: Option<String>,
    /// 状态
    pub status: Option<i32>,
    /// 护理计划ID
    pub plan_id: Option<Uuid>,
}

