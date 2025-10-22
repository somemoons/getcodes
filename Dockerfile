# 多阶段构建 Dockerfile for 中州养老管理系统
# Multi-stage Dockerfile for ZZYL Nursing Home Management System

# 阶段1: 构建阶段
FROM rust:1.75-slim as builder

# 设置工作目录
WORKDIR /app

# 安装系统依赖
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    default-libmysqlclient-dev \
    && rm -rf /var/lib/apt/lists/*

# 复制 Cargo 文件
COPY Cargo.toml Cargo.lock ./

# 复制源代码
COPY . .

# 构建应用
RUN cargo build --release

# 阶段2: 运行阶段
FROM debian:bookworm-slim

# 设置维护者信息
LABEL maintainer="ZZYL Team"
LABEL description="中州养老管理系统 - ZZYL Nursing Home Management System"
LABEL version="3.8.8"

# 安装运行时依赖
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    default-libmysqlclient21 \
    && rm -rf /var/lib/apt/lists/*

# 创建非 root 用户
RUN groupadd -r zzyl && useradd -r -g zzyl zzyl

# 设置工作目录
WORKDIR /app

# 从构建阶段复制二进制文件
COPY --from=builder /app/target/release/zzyl-admin /app/zzyl-admin

# 复制配置文件
COPY --from=builder /app/config /app/config

# 创建必要的目录
RUN mkdir -p /app/logs /app/uploads && \
    chown -R zzyl:zzyl /app

# 切换到非 root 用户
USER zzyl

# 暴露端口
EXPOSE 8080

# 设置环境变量
ENV RUST_LOG=info
ENV RUST_BACKTRACE=1

# 健康检查
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# 启动应用
CMD ["./zzyl-admin"]

