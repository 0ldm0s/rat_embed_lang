/// 语言工具函数模块

/// 标准化语言代码
/// 将各种格式的语言代码统一为标准格式
pub fn normalize_language_code(code: &str) -> String {
    let code = code.trim().to_lowercase();

    match code.as_str() {
        // 简体中文
        "zh" | "zh_cn" | "zh-cn" => "zh-CN".to_string(),

        // 英语
        "en" | "en_us" | "en-us" => "en-US".to_string(),

        // 日语
        "ja" | "ja_jp" | "ja-jp" => "ja-JP".to_string(),

        // 韩语
        "ko" | "ko_kr" | "ko-kr" => "ko-KR".to_string(),

        // 其他常见语言
        "fr" | "fr_fr" | "fr-fr" => "fr-FR".to_string(),
        "de" | "de_de" | "de-de" => "de-DE".to_string(),
        "es" | "es_es" | "es-es" => "es-ES".to_string(),
        "it" | "it_it" | "it-it" => "it-IT".to_string(),
        "ru" | "ru_ru" | "ru-ru" => "ru-RU".to_string(),
        "pt" | "pt_br" | "pt-br" => "pt-BR".to_string(),
        "ar" | "ar_sa" | "ar-sa" => "ar-SA".to_string(),
        "hi" | "hi_in" | "hi-in" => "hi-IN".to_string(),
        "th" | "th_th" | "th-th" => "th-TH".to_string(),
        "vi" | "vi_vn" | "vi-vn" => "vi-VN".to_string(),

        // 如果已经是标准格式，直接返回
        _ => {
            if code.len() == 5 && code.chars().nth(2) == Some('-') {
                code.to_uppercase()
            } else {
                // 无法识别的格式，默认返回英语
                "en-US".to_string()
            }
        }
    }
}

/// 从环境变量获取语言设置
/// 优先级：RAT_LANG > LANG > 默认英语
pub fn get_language_from_env() -> String {
    // 优先使用RAT_LANG环境变量
    if let Ok(rat_lang) = std::env::var("RAT_LANG") {
        return normalize_language_code(&rat_lang);
    }

    // 其次使用系统LANG环境变量
    if let Ok(lang) = std::env::var("LANG") {
        return normalize_language_code(&lang);
    }

    // 最后默认返回英语
    "en-US".to_string()
}

/// 检查是否为有效的语言代码
pub fn is_valid_language_code(code: &str) -> bool {
    let normalized = normalize_language_code(code);
    // 检查是否为标准的xx-XX格式
    normalized.len() == 5 && normalized.chars().nth(2) == Some('-')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_language_code() {
        assert_eq!(normalize_language_code("zh"), "zh-CN");
        assert_eq!(normalize_language_code("zh_CN"), "zh-CN");
        assert_eq!(normalize_language_code("zh-cn"), "zh-CN");
        assert_eq!(normalize_language_code("en"), "en-US");
        assert_eq!(normalize_language_code("EN-US"), "en-US");
        assert_eq!(normalize_language_code("invalid"), "en-US");
    }

    #[test]
    fn test_get_language_from_env() {
        // 这个测试需要在实际环境中运行
        // 或者设置临时环境变量
        let lang = get_language_from_env();
        assert!(!lang.is_empty());
        assert!(is_valid_language_code(&lang));
    }
}