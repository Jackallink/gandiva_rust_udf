use gandiva_rust_udf_macro::udf;
use serde_json::Value;

#[udf]
pub fn escjson(json_str: &str) -> Result<String, String> {
    // 尝试将输入字符串解析为 JSON 值
    let value: Result<Value, _> = serde_json::from_str(json_str);
    match value {
        Ok(v) => {
            // 如果是有效的 JSON，将其转换为字符串，这会自动处理转义
            let escaped_json_str = serde_json::to_string(&v).map_err(|e| e.to_string())?;
            Ok(escaped_json_str)
        }
        Err(e) => {
            // 如果不是有效的 JSON，返回错误信息
            Err(e.to_string())
        }
    }
}

#[cfg(test)]
mod escjson_tests {
    use super::*;

    #[test]
    fn test_valid_json() {
        let input_json = r#"{"message": "Hello, world"}"#;
        let expected_output = r#"{"message":"Hello, world"}"#;
        assert_eq!(escjson(input_json).unwrap(), expected_output);
    }

    #[test]
    fn test_json_with_special_chars() {
        let input_json = r#"{"message": "Hello, \"world\"!"}"#;
        let expected_output = r#"{"message":"Hello, \"world\"!"}"#;
        assert_eq!(escjson(input_json).unwrap(), expected_output);
    }

    #[test]
    fn test_invalid_json() {
            let input_json = r#"{"key" "value"}"#;
            let result = escjson(input_json);
            println!("{:?}", result);
            assert!(result.is_err());
            let error_message = result.err().unwrap();
            println!("{}", error_message);
            // 检查错误信息是否包含 "expected `:`" 或其他相关错误信息
            assert!(
                error_message.contains("key must be a string") || error_message.contains("expected `:`"),
                "Error message: {}",
                error_message
            );
        }
    }
