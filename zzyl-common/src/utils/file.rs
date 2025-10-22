use std::fs;
use std::path::{Path, PathBuf};
use std::io::{self, Read, Write};
use mime_guess::MimeGuess;
use tempfile::NamedTempFile;

/// 文件工具类
pub struct FileUtils;

impl FileUtils {
    /// 检查文件是否存在
    pub fn exists(path: &str) -> bool {
        Path::new(path).exists()
    }
    
    /// 检查是否为文件
    pub fn is_file(path: &str) -> bool {
        Path::new(path).is_file()
    }
    
    /// 检查是否为目录
    pub fn is_dir(path: &str) -> bool {
        Path::new(path).is_dir()
    }
    
    /// 获取文件大小
    pub fn get_file_size(path: &str) -> Result<u64, io::Error> {
        let metadata = fs::metadata(path)?;
        Ok(metadata.len())
    }
    
    /// 获取文件扩展名
    pub fn get_extension(path: &str) -> Option<String> {
        Path::new(path)
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_lowercase())
    }
    
    /// 获取文件名（不含扩展名）
    pub fn get_filename_without_extension(path: &str) -> Option<String> {
        Path::new(path)
            .file_stem()
            .and_then(|name| name.to_str())
            .map(|name| name.to_string())
    }
    
    /// 获取文件名（含扩展名）
    pub fn get_filename(path: &str) -> Option<String> {
        Path::new(path)
            .file_name()
            .and_then(|name| name.to_str())
            .map(|name| name.to_string())
    }
    
    /// 获取文件MIME类型
    pub fn get_mime_type(path: &str) -> Option<String> {
        MimeGuess::from_path(path).first().map(|mime| mime.to_string())
    }
    
    /// 创建目录
    pub fn create_dir(path: &str) -> Result<(), io::Error> {
        fs::create_dir_all(path)
    }
    
    /// 删除文件
    pub fn delete_file(path: &str) -> Result<(), io::Error> {
        fs::remove_file(path)
    }
    
    /// 删除目录
    pub fn delete_dir(path: &str) -> Result<(), io::Error> {
        fs::remove_dir_all(path)
    }
    
    /// 复制文件
    pub fn copy_file(src: &str, dest: &str) -> Result<u64, io::Error> {
        fs::copy(src, dest)
    }
    
    /// 移动文件
    pub fn move_file(src: &str, dest: &str) -> Result<(), io::Error> {
        fs::rename(src, dest)
    }
    
    /// 读取文件内容为字符串
    pub fn read_to_string(path: &str) -> Result<String, io::Error> {
        fs::read_to_string(path)
    }
    
    /// 读取文件内容为字节数组
    pub fn read_bytes(path: &str) -> Result<Vec<u8>, io::Error> {
        fs::read(path)
    }
    
    /// 写入字符串到文件
    pub fn write_string(path: &str, content: &str) -> Result<(), io::Error> {
        fs::write(path, content)
    }
    
    /// 写入字节数组到文件
    pub fn write_bytes(path: &str, content: &[u8]) -> Result<(), io::Error> {
        fs::write(path, content)
    }
    
    /// 追加字符串到文件
    pub fn append_string(path: &str, content: &str) -> Result<(), io::Error> {
        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }
    
    /// 获取目录下的所有文件
    pub fn list_files(dir: &str) -> Result<Vec<PathBuf>, io::Error> {
        let mut files = Vec::new();
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                files.push(path);
            }
        }
        Ok(files)
    }
    
    /// 获取目录下的所有子目录
    pub fn list_dirs(dir: &str) -> Result<Vec<PathBuf>, io::Error> {
        let mut dirs = Vec::new();
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                dirs.push(path);
            }
        }
        Ok(dirs)
    }
    
    /// 递归获取目录下的所有文件
    pub fn list_files_recursive(dir: &str) -> Result<Vec<PathBuf>, io::Error> {
        let mut files = Vec::new();
        Self::list_files_recursive_impl(dir, &mut files)?;
        Ok(files)
    }
    
    fn list_files_recursive_impl(dir: &str, files: &mut Vec<PathBuf>) -> Result<(), io::Error> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                files.push(path);
            } else if path.is_dir() {
                Self::list_files_recursive_impl(path.to_str().unwrap(), files)?;
            }
        }
        Ok(())
    }
    
    /// 格式化文件大小
    pub fn format_file_size(size: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        let mut size = size as f64;
        let mut unit_index = 0;
        
        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }
        
        if unit_index == 0 {
            format!("{} {}", size as u64, UNITS[unit_index])
        } else {
            format!("{:.2} {}", size, UNITS[unit_index])
        }
    }
    
    /// 生成临时文件
    pub fn create_temp_file() -> Result<NamedTempFile, io::Error> {
        NamedTempFile::new()
    }
    
    /// 生成临时文件路径
    pub fn create_temp_file_path(prefix: &str, suffix: &str) -> Result<PathBuf, io::Error> {
        let temp_dir = std::env::temp_dir();
        let mut counter = 0;
        loop {
            let filename = format!("{}{}{}", prefix, counter, suffix);
            let path = temp_dir.join(filename);
            if !path.exists() {
                return Ok(path);
            }
            counter += 1;
        }
    }
    
    /// 检查文件是否可读
    pub fn is_readable(path: &str) -> bool {
        fs::metadata(path)
            .map(|metadata| metadata.permissions().readonly())
            .unwrap_or(false)
    }
    
    /// 检查文件是否可写
    pub fn is_writable(path: &str) -> bool {
        // 尝试以写模式打开文件
        fs::OpenOptions::new().write(true).open(path).is_ok()
    }
    
    /// 获取文件的最后修改时间
    pub fn get_last_modified(path: &str) -> Result<std::time::SystemTime, io::Error> {
        let metadata = fs::metadata(path)?;
        metadata.modified()
    }
    
    /// 获取文件的创建时间
    pub fn get_created(path: &str) -> Result<std::time::SystemTime, io::Error> {
        let metadata = fs::metadata(path)?;
        metadata.created()
    }
    
    /// 检查文件路径是否安全（防止目录遍历攻击）
    pub fn is_safe_path(path: &str) -> bool {
        let path = Path::new(path);
        path.components().all(|component| {
            match component {
                std::path::Component::ParentDir => false,
                std::path::Component::CurDir => false,
                _ => true,
            }
        })
    }
    
    /// 规范化文件路径
    pub fn normalize_path(path: &str) -> Result<PathBuf, io::Error> {
        let path = Path::new(path);
        path.canonicalize()
    }
    
    /// 检查文件扩展名是否允许
    pub fn is_allowed_extension(path: &str, allowed_extensions: &[&str]) -> bool {
        if let Some(extension) = Self::get_extension(path) {
            allowed_extensions.contains(&extension.as_str())
        } else {
            false
        }
    }
    
    /// 检查文件大小是否在限制范围内
    pub fn is_size_allowed(path: &str, max_size: u64) -> Result<bool, io::Error> {
        let file_size = Self::get_file_size(path)?;
        Ok(file_size <= max_size)
    }
}
