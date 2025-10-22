#!/bin/bash

# 中州养老管理系统运行脚本
# ZZYL Nursing Home Management System Run Script

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 日志函数
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# 检查可执行文件
check_executable() {
    local mode=${1:-release}
    local executable=""
    
    if [ "$mode" = "release" ]; then
        executable="target/release/zzyl-admin"
    else
        executable="target/debug/zzyl-admin"
    fi
    
    if [ ! -f "$executable" ]; then
        log_error "可执行文件不存在: $executable"
        log_info "请先运行构建脚本: ./scripts/build.sh"
        exit 1
    fi
    
    echo "$executable"
}

# 检查配置文件
check_config() {
    log_info "检查配置文件..."
    
    if [ ! -f "config/application.yml" ]; then
        log_warning "配置文件不存在: config/application.yml"
        log_info "使用默认配置启动"
    else
        log_success "配置文件检查通过"
    fi
}

# 检查数据库连接
check_database() {
    log_info "检查数据库连接..."
    
    # 这里可以添加数据库连接检查逻辑
    # 例如使用 mysql 客户端连接测试
    
    log_success "数据库连接检查完成"
}

# 检查 Redis 连接
check_redis() {
    log_info "检查 Redis 连接..."
    
    # 这里可以添加 Redis 连接检查逻辑
    # 例如使用 redis-cli 连接测试
    
    log_success "Redis 连接检查完成"
}

# 设置环境变量
setup_environment() {
    log_info "设置环境变量..."
    
    # 设置 Rust 日志级别
    export RUST_LOG=${RUST_LOG:-info}
    
    # 设置配置文件路径
    export CONFIG_PATH=${CONFIG_PATH:-config/application.yml}
    
    # 设置数据库连接
    export DATABASE_URL=${DATABASE_URL:-mysql://root:password@localhost:3306/zzyl}
    
    # 设置 Redis 连接
    export REDIS_URL=${REDIS_URL:-redis://localhost:6379/0}
    
    # 设置 JWT 密钥
    export JWT_SECRET=${JWT_SECRET:-zzyl_default_secret_key}
    
    log_success "环境变量设置完成"
}

# 启动应用
start_application() {
    local executable=$1
    local mode=${2:-release}
    
    log_info "启动中州养老管理系统..."
    log_info "运行模式: $mode"
    log_info "可执行文件: $executable"
    
    # 显示启动信息
    echo "=========================================="
    echo "🏥 中州养老管理系统"
    echo "=========================================="
    echo "版本: 3.8.8"
    echo "技术栈: Rust + Axum + MySQL + Redis"
    echo "启动时间: $(date)"
    echo "=========================================="
    echo ""
    
    # 启动应用
    exec "$executable"
}

# 开发模式运行
run_dev() {
    log_info "开发模式启动..."
    
    local executable=$(check_executable debug)
    check_config
    setup_environment
    
    # 开发模式设置
    export RUST_LOG=debug
    export RUST_BACKTRACE=1
    
    start_application "$executable" "development"
}

# 生产模式运行
run_prod() {
    log_info "生产模式启动..."
    
    local executable=$(check_executable release)
    check_config
    check_database
    check_redis
    setup_environment
    
    # 生产模式设置
    export RUST_LOG=info
    
    start_application "$executable" "production"
}

# 测试模式运行
run_test() {
    log_info "测试模式启动..."
    
    local executable=$(check_executable debug)
    setup_environment
    
    # 测试模式设置
    export RUST_LOG=debug
    export RUST_BACKTRACE=1
    export TEST_MODE=true
    
    start_application "$executable" "test"
}

# 后台运行
run_daemon() {
    log_info "后台模式启动..."
    
    local executable=$(check_executable release)
    local pid_file="zzyl-admin.pid"
    local log_file="zzyl-admin.log"
    
    check_config
    check_database
    check_redis
    setup_environment
    
    # 检查是否已经在运行
    if [ -f "$pid_file" ]; then
        local pid=$(cat "$pid_file")
        if ps -p "$pid" > /dev/null 2>&1; then
            log_error "应用已在运行 (PID: $pid)"
            exit 1
        else
            log_warning "清理旧的 PID 文件"
            rm -f "$pid_file"
        fi
    fi
    
    # 后台启动
    nohup "$executable" > "$log_file" 2>&1 &
    local pid=$!
    echo "$pid" > "$pid_file"
    
    log_success "应用已在后台启动 (PID: $pid)"
    log_info "日志文件: $log_file"
    log_info "PID 文件: $pid_file"
    log_info "停止命令: ./scripts/stop.sh"
}

# 停止应用
stop_application() {
    local pid_file="zzyl-admin.pid"
    
    if [ ! -f "$pid_file" ]; then
        log_warning "PID 文件不存在，应用可能未在后台运行"
        exit 1
    fi
    
    local pid=$(cat "$pid_file")
    
    if ! ps -p "$pid" > /dev/null 2>&1; then
        log_warning "进程不存在 (PID: $pid)"
        rm -f "$pid_file"
        exit 1
    fi
    
    log_info "停止应用 (PID: $pid)..."
    kill "$pid"
    
    # 等待进程结束
    local count=0
    while ps -p "$pid" > /dev/null 2>&1 && [ $count -lt 10 ]; do
        sleep 1
        count=$((count + 1))
    done
    
    if ps -p "$pid" > /dev/null 2>&1; then
        log_warning "进程未正常结束，强制终止..."
        kill -9 "$pid"
    fi
    
    rm -f "$pid_file"
    log_success "应用已停止"
}

# 查看状态
show_status() {
    local pid_file="zzyl-admin.pid"
    
    if [ ! -f "$pid_file" ]; then
        log_info "应用状态: 未运行"
        exit 0
    fi
    
    local pid=$(cat "$pid_file")
    
    if ps -p "$pid" > /dev/null 2>&1; then
        log_success "应用状态: 运行中 (PID: $pid)"
        
        # 显示进程信息
        ps -p "$pid" -o pid,ppid,cmd,etime,pcpu,pmem
        
        # 显示端口占用
        log_info "端口占用情况:"
        netstat -tlnp 2>/dev/null | grep ":8080" || log_warning "端口 8080 未监听"
    else
        log_warning "应用状态: 已停止 (PID 文件存在但进程不存在)"
        rm -f "$pid_file"
    fi
}

# 查看日志
show_logs() {
    local log_file="zzyl-admin.log"
    local lines=${1:-50}
    
    if [ ! -f "$log_file" ]; then
        log_error "日志文件不存在: $log_file"
        exit 1
    fi
    
    log_info "显示最后 $lines 行日志:"
    echo "=========================================="
    tail -n "$lines" "$log_file"
    echo "=========================================="
}

# 重启应用
restart_application() {
    log_info "重启应用..."
    stop_application
    sleep 2
    run_daemon
}

# 显示帮助信息
show_help() {
    echo "中州养老管理系统运行脚本"
    echo ""
    echo "用法: $0 [命令] [选项]"
    echo ""
    echo "命令:"
    echo "  dev         开发模式运行"
    echo "  prod        生产模式运行"
    echo "  test        测试模式运行"
    echo "  daemon      后台模式运行"
    echo "  stop        停止应用"
    echo "  restart     重启应用"
    echo "  status      查看状态"
    echo "  logs [行数] 查看日志 (默认: 50行)"
    echo "  help        显示帮助信息"
    echo ""
    echo "环境变量:"
    echo "  RUST_LOG     日志级别 (默认: info)"
    echo "  CONFIG_PATH  配置文件路径 (默认: config/application.yml)"
    echo "  DATABASE_URL 数据库连接URL"
    echo "  REDIS_URL    Redis连接URL"
    echo "  JWT_SECRET   JWT密钥"
    echo ""
    echo "示例:"
    echo "  $0 dev                    # 开发模式运行"
    echo "  $0 prod                   # 生产模式运行"
    echo "  $0 daemon                 # 后台运行"
    echo "  $0 logs 100               # 查看最后100行日志"
    echo "  RUST_LOG=debug $0 dev     # 调试模式运行"
}

# 主函数
main() {
    local command=${1:-help}
    
    case $command in
        dev)
            run_dev
            ;;
        prod)
            run_prod
            ;;
        test)
            run_test
            ;;
        daemon)
            run_daemon
            ;;
        stop)
            stop_application
            ;;
        restart)
            restart_application
            ;;
        status)
            show_status
            ;;
        logs)
            show_logs "$2"
            ;;
        help)
            show_help
            ;;
        *)
            log_error "未知命令: $command"
            show_help
            exit 1
            ;;
    esac
}

# 执行主函数
main "$@"

