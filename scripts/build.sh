#!/bin/bash

# 中州养老管理系统构建脚本
# ZZYL Nursing Home Management System Build Script

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

# 检查 Rust 环境
check_rust() {
    log_info "检查 Rust 环境..."
    
    if ! command -v cargo &> /dev/null; then
        log_error "Cargo 未安装，请先安装 Rust"
        exit 1
    fi
    
    local rust_version=$(cargo --version | cut -d' ' -f2)
    log_success "Rust 版本: $rust_version"
}

# 检查依赖
check_dependencies() {
    log_info "检查项目依赖..."
    
    # 检查 MySQL
    if ! command -v mysql &> /dev/null; then
        log_warning "MySQL 客户端未安装，请确保数据库服务可用"
    fi
    
    # 检查 Redis
    if ! command -v redis-cli &> /dev/null; then
        log_warning "Redis 客户端未安装，请确保 Redis 服务可用"
    fi
    
    log_success "依赖检查完成"
}

# 清理构建缓存
clean_build() {
    log_info "清理构建缓存..."
    cargo clean
    log_success "构建缓存清理完成"
}

# 代码格式化
format_code() {
    log_info "格式化代码..."
    cargo fmt --all
    log_success "代码格式化完成"
}

# 代码检查
check_code() {
    log_info "检查代码质量..."
    cargo clippy --all-targets --all-features -- -D warnings
    log_success "代码检查通过"
}

# 运行测试
run_tests() {
    log_info "运行测试..."
    cargo test --all
    log_success "测试通过"
}

# 构建项目
build_project() {
    log_info "构建项目..."
    
    # 检查构建模式
    local build_mode=${1:-release}
    
    if [ "$build_mode" = "release" ]; then
        log_info "构建发布版本..."
        cargo build --release
    else
        log_info "构建调试版本..."
        cargo build
    fi
    
    log_success "项目构建完成"
}

# 构建文档
build_docs() {
    log_info "构建文档..."
    cargo doc --all --no-deps
    log_success "文档构建完成"
    log_info "文档位置: target/doc/index.html"
}

# 安全检查
security_check() {
    log_info "进行安全检查..."
    
    if command -v cargo-audit &> /dev/null; then
        cargo audit
        log_success "安全检查完成"
    else
        log_warning "cargo-audit 未安装，跳过安全检查"
        log_info "安装命令: cargo install cargo-audit"
    fi
}

# 显示构建信息
show_build_info() {
    log_info "构建信息:"
    echo "  - 项目名称: 中州养老管理系统"
    echo "  - 项目版本: 3.8.8"
    echo "  - 构建时间: $(date)"
    echo "  - Rust 版本: $(cargo --version | cut -d' ' -f2)"
    echo "  - 目标平台: $(rustc -vV | grep host | cut -d' ' -f2)"
    
    if [ -f "target/release/zzyl-admin" ]; then
        echo "  - 可执行文件: target/release/zzyl-admin"
        echo "  - 文件大小: $(du -h target/release/zzyl-admin | cut -f1)"
    elif [ -f "target/debug/zzyl-admin" ]; then
        echo "  - 可执行文件: target/debug/zzyl-admin"
        echo "  - 文件大小: $(du -h target/debug/zzyl-admin | cut -f1)"
    fi
}

# 显示帮助信息
show_help() {
    echo "中州养老管理系统构建脚本"
    echo ""
    echo "用法: $0 [选项]"
    echo ""
    echo "选项:"
    echo "  clean       清理构建缓存"
    echo "  format      格式化代码"
    echo "  check       检查代码质量"
    echo "  test        运行测试"
    echo "  build       构建项目 (默认: release)"
    echo "  build-debug 构建调试版本"
    echo "  docs        构建文档"
    echo "  security    安全检查"
    echo "  all         执行所有步骤 (默认)"
    echo "  help        显示帮助信息"
    echo ""
    echo "示例:"
    echo "  $0                    # 执行完整构建流程"
    echo "  $0 clean              # 清理构建缓存"
    echo "  $0 build-debug        # 构建调试版本"
    echo "  $0 test               # 运行测试"
}

# 主函数
main() {
    local action=${1:-all}
    
    echo "=========================================="
    echo "🏥 中州养老管理系统构建脚本"
    echo "=========================================="
    echo ""
    
    case $action in
        clean)
            check_rust
            clean_build
            ;;
        format)
            check_rust
            format_code
            ;;
        check)
            check_rust
            check_code
            ;;
        test)
            check_rust
            run_tests
            ;;
        build)
            check_rust
            check_dependencies
            format_code
            check_code
            run_tests
            build_project release
            show_build_info
            ;;
        build-debug)
            check_rust
            check_dependencies
            format_code
            check_code
            run_tests
            build_project debug
            show_build_info
            ;;
        docs)
            check_rust
            build_docs
            ;;
        security)
            check_rust
            security_check
            ;;
        all)
            check_rust
            check_dependencies
            format_code
            check_code
            run_tests
            build_project release
            build_docs
            security_check
            show_build_info
            ;;
        help)
            show_help
            ;;
        *)
            log_error "未知选项: $action"
            show_help
            exit 1
            ;;
    esac
    
    echo ""
    log_success "构建脚本执行完成！"
}

# 执行主函数
main "$@"

