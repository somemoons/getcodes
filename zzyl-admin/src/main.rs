use axum::{
    routing::{get, post},
    Router,
    response::Html,
    extract::State,
    http::StatusCode,
    Json,
};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::{info, error};
use std::net::SocketAddr;
use zzyl_common::error::{Result, AjaxResult};
use zzyl_framework::{init_config, init_database, init_redis, get_config};

/// 应用状态
#[derive(Clone)]
pub struct AppState {
    pub config: zzyl_framework::config::ZzylConfig,
}

/// 主页处理器
async fn index() -> Html<&'static str> {
    Html(r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>中州养老管理系统</title>
        <meta charset="utf-8">
        <style>
            body { font-family: Arial, sans-serif; margin: 40px; }
            .container { max-width: 800px; margin: 0 auto; text-align: center; }
            .banner { background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); 
                      color: white; padding: 40px; border-radius: 10px; margin-bottom: 30px; }
            .info { background: #f8f9fa; padding: 20px; border-radius: 5px; margin: 20px 0; }
            .api-list { text-align: left; background: white; padding: 20px; border-radius: 5px; }
            .api-item { margin: 10px 0; padding: 10px; background: #e9ecef; border-radius: 3px; }
        </style>
    </head>
    <body>
        <div class="container">
            <div class="banner">
                <h1>🏥 中州养老管理系统</h1>
                <p>ZZYL Nursing Home Management System</p>
                <p>Version: 3.8.8 | Powered by Rust</p>
            </div>
            
            <div class="info">
                <h2>🎉 系统启动成功！</h2>
                <p>欢迎使用中州养老管理系统，这是一个基于Rust技术栈开发的现代化养老院管理系统。</p>
            </div>
            
            <div class="api-list">
                <h3>📋 可用API接口</h3>
                <div class="api-item">
                    <strong>GET /</strong> - 系统主页
                </div>
                <div class="api-item">
                    <strong>GET /health</strong> - 健康检查
                </div>
                <div class="api-item">
                    <strong>GET /api/info</strong> - 系统信息
                </div>
                <div class="api-item">
                    <strong>POST /api/login</strong> - 用户登录
                </div>
                <div class="api-item">
                    <strong>GET /api/elders</strong> - 获取老人列表
                </div>
                <div class="api-item">
                    <strong>GET /api/rooms</strong> - 获取房间列表
                </div>
                <div class="api-item">
                    <strong>GET /api/nursing-plans</strong> - 获取护理计划
                </div>
            </div>
            
            <div class="info">
                <h3>🛠️ 技术栈</h3>
                <p>
                    <strong>后端:</strong> Rust + Axum + SQLx + SeaORM<br>
                    <strong>数据库:</strong> MySQL + Redis<br>
                    <strong>认证:</strong> JWT + Argon2<br>
                    <strong>日志:</strong> Tracing<br>
                    <strong>配置:</strong> Config + Environment Variables
                </p>
            </div>
        </div>
    </body>
    </html>
    "#)
}

/// 健康检查处理器
async fn health() -> Json<AjaxResult<String>> {
    Json(AjaxResult::success("系统运行正常".to_string()))
}

/// 系统信息处理器
async fn system_info(State(state): State<AppState>) -> Json<AjaxResult<SystemInfo>> {
    let info = SystemInfo {
        name: state.config.ruoyi.name.clone(),
        version: state.config.ruoyi.version.clone(),
        copyright_year: state.config.ruoyi.copyright_year.clone(),
        port: state.config.server.port,
        environment: "production".to_string(),
        rust_version: env!("RUSTC_SEMVER").to_string(),
    };
    
    Json(AjaxResult::success(info))
}

/// 系统信息结构体
#[derive(serde::Serialize)]
struct SystemInfo {
    name: String,
    version: String,
    copyright_year: String,
    port: u16,
    environment: String,
    rust_version: String,
}

/// 创建应用路由
fn create_app(state: AppState) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/health", get(health))
        .route("/api/info", get(system_info))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

/// 初始化应用
async fn init_app() -> Result<AppState> {
    // 加载配置
    let config = match zzyl_framework::config::ZzylConfig::from_file("config/application.yml") {
        Ok(config) => config,
        Err(_) => {
            info!("配置文件未找到，使用默认配置");
            zzyl_framework::config::ZzylConfig::default()
        }
    };
    
    // 初始化配置
    init_config(config.clone());
    
    // 初始化数据库
    info!("正在初始化数据库连接...");
    init_database(&config.spring.datasource).await
        .map_err(|e| {
            error!("数据库初始化失败: {}", e);
            e
        })?;
    
    // 初始化Redis
    info!("正在初始化Redis连接...");
    init_redis(&config.spring.redis).await
        .map_err(|e| {
            error!("Redis初始化失败: {}", e);
            e
        })?;
    
    info!("应用初始化完成");
    Ok(AppState { config })
}

/// 主函数
#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    
    info!("🚀 正在启动中州养老管理系统...");
    
    // 初始化应用
    let state = init_app().await?;
    
    // 创建应用
    let app = create_app(state.clone());
    
    // 启动服务器
    let addr = SocketAddr::from(([0, 0, 0, 0], state.config.server.port));
    info!("🌐 服务器启动在: http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await
        .map_err(|e| {
            error!("无法绑定端口 {}: {}", state.config.server.port, e);
            zzyl_common::error::ZzylError::System(format!("端口绑定失败: {}", e))
        })?;
    
    info!("✅ 中州养老管理系统启动成功！");
    info!("📖 访问地址: http://localhost:{}", state.config.server.port);
    
    // 打印启动横幅
    print_banner();
    
    axum::serve(listener, app).await
        .map_err(|e| {
            error!("服务器启动失败: {}", e);
            zzyl_common::error::ZzylError::System(format!("服务器启动失败: {}", e))
        })?;
    
    Ok(())
}

/// 打印启动横幅
fn print_banner() {
    println!(r#"
    ╔══════════════════════════════════════════════════════════════╗
    ║                                                              ║
    ║    🏥 中州养老管理系统 (ZZYL Nursing Home Management)          ║
    ║                                                              ║
    ║    Version: 3.8.8                                           ║
    ║    Powered by Rust + Axum                                   ║
    ║                                                              ║
    ║    (♥◠‿◠)ﾉﾞ  系统启动成功   ლ(´ڡ`ლ)ﾞ                      ║
    ║                                                              ║
    ║    .-------.       ____     __                              ║
    ║    |  _ _   \\      \\   \\   /  /                             ║
    ║    | ( ' )  |       \\  _. /  '                              ║
    ║    |(_ o _) /        _( )_ .'                                ║
    ║    | (_,_).' __  ___(_ o _)'                                 ║
    ║    |  |\\ \\  |  ||   |(_,_)'                                ║
    ║    |  | \\ `'   /|   `-'  /                                  ║
    ║    |  |  \\    /  \\      /                                  ║
    ║    ''-'   `'-'    `-..-'                                     ║
    ║                                                              ║
    ╚══════════════════════════════════════════════════════════════╝
    "#);
}

