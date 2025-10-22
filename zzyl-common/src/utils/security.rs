use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 安全工具类
pub struct SecurityUtils;

impl SecurityUtils {
    /// 生成盐值
    pub fn generate_salt() -> String {
        SaltString::generate(&mut OsRng).to_string()
    }
    
    /// 加密密码
    pub fn encrypt_password(password: &str) -> Result<String, argon2::password_hash::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
        Ok(password_hash.to_string())
    }
    
    /// 验证密码
    pub fn verify_password(password: &str, hash: &str) -> Result<bool, argon2::password_hash::Error> {
        let parsed_hash = PasswordHash::new(hash)?;
        let argon2 = Argon2::default();
        Ok(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
    }
    
    /// 生成JWT令牌
    pub fn generate_token(user_id: &Uuid, username: &str, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
        let claims = Claims {
            sub: user_id.to_string(),
            username: username.to_string(),
            exp: chrono::Utc::now().timestamp() + 7 * 24 * 60 * 60, // 7天过期
            iat: chrono::Utc::now().timestamp(),
        };
        
        encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
    }
    
    /// 验证JWT令牌
    pub fn verify_token(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let validation = Validation::new(Algorithm::HS256);
        let token_data = decode::<Claims>(token, &DecodingKey::from_secret(secret.as_ref()), &validation)?;
        Ok(token_data.claims)
    }
    
    /// 生成随机字符串
    pub fn generate_random_string(length: usize) -> String {
        use rand::Rng;
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        let mut rng = rand::thread_rng();
        
        (0..length)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }
    
    /// 生成验证码
    pub fn generate_captcha(length: usize) -> String {
        use rand::Rng;
        const CHARSET: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
        let mut rng = rand::thread_rng();
        
        (0..length)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }
    
    /// 生成数字验证码
    pub fn generate_numeric_captcha(length: usize) -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        (0..length)
            .map(|_| rng.gen_range(0..10).to_string())
            .collect()
    }
    
    /// MD5哈希
    pub fn md5_hash(input: &str) -> String {
        use md5::{Digest, Md5};
        let mut hasher = Md5::new();
        hasher.update(input.as_bytes());
        format!("{:x}", hasher.finalize())
    }
    
    /// SHA256哈希
    pub fn sha256_hash(input: &str) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        format!("{:x}", hasher.finalize())
    }
    
    /// 生成UUID
    pub fn generate_uuid() -> String {
        Uuid::new_v4().to_string()
    }
    
    /// 生成短UUID（去掉连字符）
    pub fn generate_short_uuid() -> String {
        Uuid::new_v4().to_string().replace('-', "")
    }
}

/// JWT Claims
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,    // 主题（用户ID）
    pub username: String, // 用户名
    pub exp: i64,       // 过期时间
    pub iat: i64,       // 签发时间
}

impl Claims {
    /// 获取用户ID
    pub fn get_user_id(&self) -> Result<Uuid, uuid::Error> {
        Uuid::parse_str(&self.sub)
    }
}

