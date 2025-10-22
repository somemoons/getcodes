use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Excel工具类
pub struct ExcelUtils;

impl ExcelUtils {
    /// 读取Excel文件
    pub fn read_excel(path: &str) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
        let mut workbook = calamine::open_workbook(path)?;
        let mut data = Vec::new();
        
        if let Some(Ok(range)) = workbook.worksheet_range_at(0) {
            for row in range.rows() {
                let row_data: Vec<String> = row.iter()
                    .map(|cell| cell.to_string())
                    .collect();
                data.push(row_data);
            }
        }
        
        Ok(data)
    }
    
    /// 读取Excel文件到结构体
    pub fn read_excel_to_struct<T>(path: &str, headers: &[String]) -> Result<Vec<T>, Box<dyn std::error::Error>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let data = Self::read_excel(path)?;
        if data.is_empty() {
            return Ok(Vec::new());
        }
        
        let mut result = Vec::new();
        let header_row = &data[0];
        
        // 验证标题行
        if header_row.len() != headers.len() {
            return Err("标题行数量不匹配".into());
        }
        
        for (i, header) in headers.iter().enumerate() {
            if header_row.get(i).unwrap_or(&"".to_string()) != header {
                return Err(format!("标题行第{}列不匹配，期望: {}，实际: {}", i + 1, header, header_row.get(i).unwrap_or(&"".to_string())).into());
            }
        }
        
        // 处理数据行
        for row_data in data.iter().skip(1) {
            let mut row_map = HashMap::new();
            for (i, cell) in row_data.iter().enumerate() {
                if let Some(header) = headers.get(i) {
                    row_map.insert(header.clone(), cell.clone());
                }
            }
            
            let json_str = serde_json::to_string(&row_map)?;
            let record: T = serde_json::from_str(&json_str)?;
            result.push(record);
        }
        
        Ok(result)
    }
    
    /// 写入Excel文件
    pub fn write_excel(path: &str, data: &[Vec<String>]) -> Result<(), Box<dyn std::error::Error>> {
        use calamine::{Writer, Xlsx};
        use std::fs::File;
        
        let file = File::create(path)?;
        let mut workbook = Xlsx::new(file)?;
        
        for (row_index, row_data) in data.iter().enumerate() {
            for (col_index, cell_value) in row_data.iter().enumerate() {
                workbook.write_cell(row_index as u32, col_index as u16, cell_value)?;
            }
        }
        
        workbook.close()?;
        Ok(())
    }
    
    /// 写入结构体到Excel文件
    pub fn write_struct_to_excel<T>(path: &str, headers: &[String], data: &[T]) -> Result<(), Box<dyn std::error::Error>>
    where
        T: Serialize,
    {
        let mut excel_data = Vec::new();
        
        // 添加标题行
        excel_data.push(headers.to_vec());
        
        // 添加数据行
        for record in data {
            let json_str = serde_json::to_string(record)?;
            let row_map: HashMap<String, String> = serde_json::from_str(&json_str)?;
            
            let mut row_data = Vec::new();
            for header in headers {
                let value = row_map.get(header).unwrap_or(&"".to_string()).clone();
                row_data.push(value);
            }
            excel_data.push(row_data);
        }
        
        Self::write_excel(path, &excel_data)
    }
    
    /// 读取CSV文件
    pub fn read_csv(path: &str) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
        let mut reader = csv::Reader::from_path(path)?;
        let mut data = Vec::new();
        
        for result in reader.records() {
            let record = result?;
            let row_data: Vec<String> = record.iter().map(|field| field.to_string()).collect();
            data.push(row_data);
        }
        
        Ok(data)
    }
    
    /// 写入CSV文件
    pub fn write_csv(path: &str, headers: &[String], data: &[Vec<String>]) -> Result<(), Box<dyn std::error::Error>> {
        let mut writer = csv::Writer::from_path(path)?;
        
        // 写入标题行
        writer.write_record(headers)?;
        
        // 写入数据行
        for row_data in data {
            writer.write_record(row_data)?;
        }
        
        writer.flush()?;
        Ok(())
    }
    
    /// 写入结构体到CSV文件
    pub fn write_struct_to_csv<T>(path: &str, headers: &[String], data: &[T]) -> Result<(), Box<dyn std::error::Error>>
    where
        T: Serialize,
    {
        let mut csv_data = Vec::new();
        
        for record in data {
            let json_str = serde_json::to_string(record)?;
            let row_map: HashMap<String, String> = serde_json::from_str(&json_str)?;
            
            let mut row_data = Vec::new();
            for header in headers {
                let value = row_map.get(header).unwrap_or(&"".to_string()).clone();
                row_data.push(value);
            }
            csv_data.push(row_data);
        }
        
        Self::write_csv(path, headers, &csv_data)
    }
    
    /// 验证Excel文件格式
    pub fn validate_excel_format(path: &str, expected_headers: &[String]) -> Result<bool, Box<dyn std::error::Error>> {
        let data = Self::read_excel(path)?;
        if data.is_empty() {
            return Ok(false);
        }
        
        let header_row = &data[0];
        if header_row.len() != expected_headers.len() {
            return Ok(false);
        }
        
        for (i, expected_header) in expected_headers.iter().enumerate() {
            if header_row.get(i).unwrap_or(&"".to_string()) != expected_header {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
    
    /// 获取Excel工作表名称
    pub fn get_worksheet_names(path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut workbook = calamine::open_workbook(path)?;
        let mut names = Vec::new();
        
        for sheet_name in workbook.worksheet_names() {
            names.push(sheet_name);
        }
        
        Ok(names)
    }
    
    /// 读取指定工作表
    pub fn read_worksheet(path: &str, sheet_name: &str) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
        let mut workbook = calamine::open_workbook(path)?;
        let mut data = Vec::new();
        
        if let Some(Ok(range)) = workbook.worksheet_range(sheet_name) {
            for row in range.rows() {
                let row_data: Vec<String> = row.iter()
                    .map(|cell| cell.to_string())
                    .collect();
                data.push(row_data);
            }
        }
        
        Ok(data)
    }
    
    /// 格式化单元格值
    pub fn format_cell_value(value: &str) -> String {
        value.trim().to_string()
    }
    
    /// 检查单元格是否为空
    pub fn is_cell_empty(value: &str) -> bool {
        value.trim().is_empty()
    }
    
    /// 验证数据行数量
    pub fn validate_data_rows(data: &[Vec<String>], min_rows: usize, max_rows: usize) -> bool {
        let data_rows = data.len().saturating_sub(1); // 减去标题行
        data_rows >= min_rows && data_rows <= max_rows
    }
    
    /// 清理Excel数据（去除空行和空列）
    pub fn clean_excel_data(data: &[Vec<String>]) -> Vec<Vec<String>> {
        if data.is_empty() {
            return Vec::new();
        }
        
        let mut cleaned_data = Vec::new();
        
        for row in data {
            let cleaned_row: Vec<String> = row.iter()
                .map(|cell| Self::format_cell_value(cell))
                .collect();
            
            // 检查行是否为空
            if !cleaned_row.iter().all(|cell| Self::is_cell_empty(cell)) {
                cleaned_data.push(cleaned_row);
            }
        }
        
        cleaned_data
    }
}

