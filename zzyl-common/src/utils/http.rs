use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// HTTP工具类
pub struct HttpUtils {
    client: Client,
}

impl HttpUtils {
    /// 创建HTTP工具实例
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap_or_else(|_| Client::new());
        
        Self { client }
    }
    
    /// 创建带超时的HTTP工具实例
    pub fn with_timeout(timeout: Duration) -> Self {
        let client = Client::builder()
            .timeout(timeout)
            .build()
            .unwrap_or_else(|_| Client::new());
        
        Self { client }
    }
    
    /// GET请求
    pub async fn get(&self, url: &str) -> Result<String, reqwest::Error> {
        let response = self.client.get(url).send().await?;
        response.text().await
    }
    
    /// GET请求（带查询参数）
    pub async fn get_with_params(&self, url: &str, params: &HashMap<String, String>) -> Result<String, reqwest::Error> {
        let mut request = self.client.get(url);
        
        for (key, value) in params {
            request = request.query(&[(key, value)]);
        }
        
        let response = request.send().await?;
        response.text().await
    }
    
    /// POST请求（JSON）
    pub async fn post_json<T>(&self, url: &str, data: &T) -> Result<String, reqwest::Error>
    where
        T: Serialize,
    {
        let response = self.client
            .post(url)
            .json(data)
            .send()
            .await?;
        response.text().await
    }
    
    /// POST请求（表单数据）
    pub async fn post_form(&self, url: &str, data: &HashMap<String, String>) -> Result<String, reqwest::Error> {
        let response = self.client
            .post(url)
            .form(data)
            .send()
            .await?;
        response.text().await
    }
    
    /// PUT请求（JSON）
    pub async fn put_json<T>(&self, url: &str, data: &T) -> Result<String, reqwest::Error>
    where
        T: Serialize,
    {
        let response = self.client
            .put(url)
            .json(data)
            .send()
            .await?;
        response.text().await
    }
    
    /// DELETE请求
    pub async fn delete(&self, url: &str) -> Result<String, reqwest::Error> {
        let response = self.client.delete(url).send().await?;
        response.text().await
    }
    
    /// 下载文件
    pub async fn download_file(&self, url: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let response = self.client.get(url).send().await?;
        let bytes = response.bytes().await?;
        std::fs::write(file_path, bytes)?;
        Ok(())
    }
    
    /// 上传文件
    pub async fn upload_file(&self, url: &str, file_path: &str, field_name: &str) -> Result<String, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(file_path)?;
        let file_part = reqwest::multipart::Part::stream(file)
            .file_name(std::path::Path::new(file_path).file_name().unwrap().to_str().unwrap());
        
        let form = reqwest::multipart::Form::new().part(field_name, file_part);
        
        let response = self.client
            .post(url)
            .multipart(form)
            .send()
            .await?;
        
        Ok(response.text().await?)
    }
    
    /// 设置请求头
    pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
        let mut request_builder = self.client.request(reqwest::Method::GET, "");
        
        for (key, value) in headers {
            request_builder = request_builder.header(&key, &value);
        }
        
        self.client = request_builder.build().unwrap_or_else(|_| Client::new());
        self
    }
    
    /// 设置认证头
    pub fn with_auth(self, token: &str) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), format!("Bearer {}", token));
        self.with_headers(headers)
    }
    
    /// 设置用户代理
    pub fn with_user_agent(self, user_agent: &str) -> Self {
        let mut headers = HashMap::new();
        headers.insert("User-Agent".to_string(), user_agent.to_string());
        self.with_headers(headers)
    }
    
    /// 检查URL是否可访问
    pub async fn check_url_accessible(&self, url: &str) -> bool {
        self.client.head(url).send().await.is_ok()
    }
    
    /// 获取响应状态码
    pub async fn get_status_code(&self, url: &str) -> Result<u16, reqwest::Error> {
        let response = self.client.get(url).send().await?;
        Ok(response.status().as_u16())
    }
    
    /// 获取响应头
    pub async fn get_headers(&self, url: &str) -> Result<HashMap<String, String>, reqwest::Error> {
        let response = self.client.get(url).send().await?;
        let mut headers = HashMap::new();
        
        for (key, value) in response.headers() {
            headers.insert(
                key.to_string(),
                value.to_str().unwrap_or("").to_string()
            );
        }
        
        Ok(headers)
    }
    
    /// 异步批量请求
    pub async fn batch_requests(&self, urls: Vec<String>) -> Vec<Result<String, reqwest::Error>> {
        let futures: Vec<_> = urls.into_iter()
            .map(|url| self.get(&url))
            .collect();
        
        futures::future::join_all(futures).await
    }
    
    /// 重试请求
    pub async fn retry_request<F, T>(&self, operation: F, max_retries: u32) -> Result<T, reqwest::Error>
    where
        F: Fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T, reqwest::Error>> + Send>>,
    {
        let mut retries = 0;
        
        loop {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    retries += 1;
                    if retries >= max_retries {
                        return Err(e);
                    }
                    
                    // 指数退避
                    let delay = std::time::Duration::from_millis(1000 * (2_u64.pow(retries - 1)));
                    tokio::time::sleep(delay).await;
                }
            }
        }
    }
}

impl Default for HttpUtils {
    fn default() -> Self {
        Self::new()
    }
}

/// HTTP响应结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpResponse<T> {
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub body: T,
}

impl<T> HttpResponse<T> {
    pub fn new(status_code: u16, headers: HashMap<String, String>, body: T) -> Self {
        Self {
            status_code,
            headers,
            body,
        }
    }
    
    pub fn is_success(&self) -> bool {
        self.status_code >= 200 && self.status_code < 300
    }
    
    pub fn is_client_error(&self) -> bool {
        self.status_code >= 400 && self.status_code < 500
    }
    
    pub fn is_server_error(&self) -> bool {
        self.status_code >= 500 && self.status_code < 600
    }
}

/// URL工具类
pub struct UrlUtils;

impl UrlUtils {
    /// 验证URL格式
    pub fn is_valid_url(url: &str) -> bool {
        url::Url::parse(url).is_ok()
    }
    
    /// 构建查询参数
    pub fn build_query_string(params: &HashMap<String, String>) -> String {
        params.iter()
            .map(|(key, value)| format!("{}={}", key, value))
            .collect::<Vec<_>>()
            .join("&")
    }
    
    /// 解析查询参数
    pub fn parse_query_string(query: &str) -> HashMap<String, String> {
        let mut params = HashMap::new();
        
        for pair in query.split('&') {
            if let Some((key, value)) = pair.split_once('=') {
                params.insert(key.to_string(), value.to_string());
            }
        }
        
        params
    }
    
    /// 获取域名
    pub fn get_domain(url: &str) -> Option<String> {
        url::Url::parse(url).ok().and_then(|url| url.host_str().map(|s| s.to_string()))
    }
    
    /// 获取路径
    pub fn get_path(url: &str) -> Option<String> {
        url::Url::parse(url).ok().map(|url| url.path().to_string())
    }
    
    /// 获取协议
    pub fn get_scheme(url: &str) -> Option<String> {
        url::Url::parse(url).ok().map(|url| url.scheme().to_string())
    }
    
    /// URL编码
    pub fn encode(url: &str) -> String {
        urlencoding::encode(url).to_string()
    }
    
    /// URL解码
    pub fn decode(url: &str) -> Result<String, urlencoding::UrlDecodeError> {
        urlencoding::decode(url).map(|s| s.to_string())
    }
}

