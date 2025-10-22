use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// JSON工具类
pub struct JsonUtils;

impl JsonUtils {
    /// 对象转JSON字符串
    pub fn to_json_string<T>(obj: &T) -> Result<String, serde_json::Error>
    where
        T: Serialize,
    {
        serde_json::to_string(obj)
    }
    
    /// 对象转JSON字符串（格式化）
    pub fn to_json_string_pretty<T>(obj: &T) -> Result<String, serde_json::Error>
    where
        T: Serialize,
    {
        serde_json::to_string_pretty(obj)
    }
    
    /// JSON字符串转对象
    pub fn from_json_string<T>(json_str: &str) -> Result<T, serde_json::Error>
    where
        T: for<'de> Deserialize<'de>,
    {
        serde_json::from_str(json_str)
    }
    
    /// JSON字符串转HashMap
    pub fn from_json_to_map(json_str: &str) -> Result<HashMap<String, serde_json::Value>, serde_json::Error> {
        serde_json::from_str(json_str)
    }
    
    /// HashMap转JSON字符串
    pub fn map_to_json_string(map: &HashMap<String, serde_json::Value>) -> Result<String, serde_json::Error> {
        serde_json::to_string(map)
    }
    
    /// 验证JSON字符串格式
    pub fn is_valid_json(json_str: &str) -> bool {
        serde_json::from_str::<serde_json::Value>(json_str).is_ok()
    }
    
    /// 获取JSON字段值
    pub fn get_json_field_value(json_str: &str, field_path: &str) -> Result<Option<serde_json::Value>, serde_json::Error> {
        let value: serde_json::Value = serde_json::from_str(json_str)?;
        Self::get_field_value(&value, field_path)
    }
    
    /// 从JSON对象中获取字段值
    pub fn get_field_value(json_value: &serde_json::Value, field_path: &str) -> Result<Option<serde_json::Value>, serde_json::Error> {
        let parts: Vec<&str> = field_path.split('.').collect();
        let mut current = json_value;
        
        for part in parts {
            match current {
                serde_json::Value::Object(map) => {
                    if let Some(value) = map.get(part) {
                        current = value;
                    } else {
                        return Ok(None);
                    }
                }
                serde_json::Value::Array(arr) => {
                    if let Ok(index) = part.parse::<usize>() {
                        if let Some(value) = arr.get(index) {
                            current = value;
                        } else {
                            return Ok(None);
                        }
                    } else {
                        return Ok(None);
                    }
                }
                _ => return Ok(None),
            }
        }
        
        Ok(Some(current.clone()))
    }
    
    /// 设置JSON字段值
    pub fn set_json_field_value(json_str: &str, field_path: &str, value: serde_json::Value) -> Result<String, serde_json::Error> {
        let mut json_value: serde_json::Value = serde_json::from_str(json_str)?;
        Self::set_field_value(&mut json_value, field_path, value)?;
        serde_json::to_string(&json_value)
    }
    
    /// 设置JSON对象中的字段值
    pub fn set_field_value(json_value: &mut serde_json::Value, field_path: &str, value: serde_json::Value) -> Result<(), serde_json::Error> {
        let parts: Vec<&str> = field_path.split('.').collect();
        let mut current = json_value;
        
        for (i, part) in parts.iter().enumerate() {
            if i == parts.len() - 1 {
                // 最后一个部分，设置值
                match current {
                    serde_json::Value::Object(map) => {
                        map.insert(part.to_string(), value);
                    }
                    _ => return Err(serde_json::Error::custom("Cannot set value on non-object")),
                }
            } else {
                // 中间部分，获取或创建对象
                match current {
                    serde_json::Value::Object(map) => {
                        if !map.contains_key(*part) {
                            map.insert(part.to_string(), serde_json::Value::Object(serde_json::Map::new()));
                        }
                        current = map.get_mut(*part).unwrap();
                    }
                    _ => return Err(serde_json::Error::custom("Cannot navigate through non-object")),
                }
            }
        }
        
        Ok(())
    }
    
    /// 删除JSON字段
    pub fn remove_json_field(json_str: &str, field_path: &str) -> Result<String, serde_json::Error> {
        let mut json_value: serde_json::Value = serde_json::from_str(json_str)?;
        Self::remove_field(&mut json_value, field_path)?;
        serde_json::to_string(&json_value)
    }
    
    /// 从JSON对象中删除字段
    pub fn remove_field(json_value: &mut serde_json::Value, field_path: &str) -> Result<(), serde_json::Error> {
        let parts: Vec<&str> = field_path.split('.').collect();
        let mut current = json_value;
        
        for (i, part) in parts.iter().enumerate() {
            if i == parts.len() - 1 {
                // 最后一个部分，删除字段
                match current {
                    serde_json::Value::Object(map) => {
                        map.remove(*part);
                    }
                    _ => return Err(serde_json::Error::custom("Cannot remove field from non-object")),
                }
            } else {
                // 中间部分，导航到对象
                match current {
                    serde_json::Value::Object(map) => {
                        if let Some(value) = map.get_mut(*part) {
                            current = value;
                        } else {
                            return Ok(()); // 字段不存在，无需删除
                        }
                    }
                    _ => return Err(serde_json::Error::custom("Cannot navigate through non-object")),
                }
            }
        }
        
        Ok(())
    }
    
    /// 合并两个JSON对象
    pub fn merge_json_objects(json1: &str, json2: &str) -> Result<String, serde_json::Error> {
        let mut value1: serde_json::Value = serde_json::from_str(json1)?;
        let value2: serde_json::Value = serde_json::from_str(json2)?;
        
        Self::merge_values(&mut value1, &value2);
        serde_json::to_string(&value1)
    }
    
    /// 合并两个JSON值
    pub fn merge_values(target: &mut serde_json::Value, source: &serde_json::Value) {
        match (target, source) {
            (serde_json::Value::Object(target_map), serde_json::Value::Object(source_map)) => {
                for (key, source_value) in source_map {
                    if let Some(target_value) = target_map.get_mut(key) {
                        Self::merge_values(target_value, source_value);
                    } else {
                        target_map.insert(key.clone(), source_value.clone());
                    }
                }
            }
            (target, source) => {
                *target = source.clone();
            }
        }
    }
    
    /// 扁平化JSON对象
    pub fn flatten_json(json_str: &str) -> Result<HashMap<String, serde_json::Value>, serde_json::Error> {
        let value: serde_json::Value = serde_json::from_str(json_str)?;
        let mut result = HashMap::new();
        Self::flatten_value(&value, String::new(), &mut result);
        Ok(result)
    }
    
    /// 扁平化JSON值
    fn flatten_value(value: &serde_json::Value, prefix: String, result: &mut HashMap<String, serde_json::Value>) {
        match value {
            serde_json::Value::Object(map) => {
                for (key, val) in map {
                    let new_prefix = if prefix.is_empty() {
                        key.clone()
                    } else {
                        format!("{}.{}", prefix, key)
                    };
                    Self::flatten_value(val, new_prefix, result);
                }
            }
            serde_json::Value::Array(arr) => {
                for (i, val) in arr.iter().enumerate() {
                    let new_prefix = format!("{}[{}]", prefix, i);
                    Self::flatten_value(val, new_prefix, result);
                }
            }
            _ => {
                result.insert(prefix, value.clone());
            }
        }
    }
    
    /// 验证JSON模式
    pub fn validate_json_schema(json_str: &str, required_fields: &[String]) -> Result<bool, serde_json::Error> {
        let value: serde_json::Value = serde_json::from_str(json_str)?;
        
        match value {
            serde_json::Value::Object(map) => {
                for field in required_fields {
                    if !map.contains_key(field) {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            _ => Ok(false),
        }
    }
    
    /// 格式化JSON字符串
    pub fn format_json(json_str: &str) -> Result<String, serde_json::Error> {
        let value: serde_json::Value = serde_json::from_str(json_str)?;
        serde_json::to_string_pretty(&value)
    }
    
    /// 压缩JSON字符串（去除空格）
    pub fn compress_json(json_str: &str) -> Result<String, serde_json::Error> {
        let value: serde_json::Value = serde_json::from_str(json_str)?;
        serde_json::to_string(&value)
    }
    
    /// 获取JSON对象的所有键
    pub fn get_json_keys(json_str: &str) -> Result<Vec<String>, serde_json::Error> {
        let value: serde_json::Value = serde_json::from_str(json_str)?;
        
        match value {
            serde_json::Value::Object(map) => Ok(map.keys().cloned().collect()),
            _ => Ok(Vec::new()),
        }
    }
    
    /// 获取JSON数组长度
    pub fn get_json_array_length(json_str: &str) -> Result<usize, serde_json::Error> {
        let value: serde_json::Value = serde_json::from_str(json_str)?;
        
        match value {
            serde_json::Value::Array(arr) => Ok(arr.len()),
            _ => Ok(0),
        }
    }
}

