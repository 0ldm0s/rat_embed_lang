/// 多语言核心API

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::sync::LazyLock;

/// 全局翻译存储
static TRANSLATIONS: LazyLock<RwLock<HashMap<String, HashMap<String, String>>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

/// 全局当前语言设置
static CURRENT_LANGUAGE: LazyLock<RwLock<String>> =
    LazyLock::new(|| RwLock::new("en-US".to_string()));

/// 设置当前语言
pub fn set_language(lang: &str) {
    let mut current_lang = CURRENT_LANGUAGE.write().unwrap();
    *current_lang = lang.to_string();
}

/// 获取当前语言
pub fn current_language() -> String {
    CURRENT_LANGUAGE.read().unwrap().clone()
}

/// 注册翻译数据
/// translations: HashMap<key, HashMap<language_code, translation_text>>
/// 如果有重复的key会panic并提示错误信息
pub fn register_translations(translations: HashMap<String, HashMap<String, String>>) {
    let mut global_translations = TRANSLATIONS.write().unwrap();

    // 检查是否有重复的key
    let mut duplicate_keys = Vec::new();
    for key in translations.keys() {
        if global_translations.contains_key(key) {
            duplicate_keys.push(key.clone());
        }
    }

    // 如果有重复key，报错
    if !duplicate_keys.is_empty() {
        let current_lang = CURRENT_LANGUAGE.read().unwrap();
        let error_msg = if current_lang.starts_with("zh") {
            format!("翻译key重复: {}. 请检查或先调用clear_translations()清理现有翻译数据。", duplicate_keys.join(", "))
        } else {
            format!("Duplicate translation keys: {}. Please check or call clear_translations() to clear existing data.", duplicate_keys.join(", "))
        };
        panic!("{}", error_msg);
    }

    // 添加新翻译
    for (key, lang_map) in translations {
        global_translations.insert(key, lang_map);
    }
}

/// 清理所有已注册的翻译数据
pub fn clear_translations() {
    let mut global_translations = TRANSLATIONS.write().unwrap();
    global_translations.clear();
}

/// 获取翻译文本
/// 优先返回当前语言的翻译，如果没有则fallback到英语，如果都没有则返回[key]
pub fn t(key: &str) -> String {
    let translations = TRANSLATIONS.read().unwrap();
    let current_lang = CURRENT_LANGUAGE.read().unwrap();

    if let Some(lang_translations) = translations.get(key) {
        // 优先返回当前语言
        if let Some(text) = lang_translations.get(&*current_lang) {
            return text.clone();
        }

        // Fallback到英语
        if let Some(text) = lang_translations.get("en-US") {
            return text.clone();
        }

        // Fallback到第一个可用的翻译
        if let Some((_, text)) = lang_translations.iter().next() {
            return text.clone();
        }
    }

    // 都没有则返回key本身
    format!("[{}]", key)
}

/// 获取指定语言的翻译
pub fn t_with_lang(key: &str, lang: &str) -> String {
    let translations = TRANSLATIONS.read().unwrap();

    if let Some(lang_translations) = translations.get(key) {
        if let Some(text) = lang_translations.get(lang) {
            return text.clone();
        }
    }

    format!("[{}]", key)
}

/// 检查是否存在指定key的翻译
pub fn has_translation(key: &str) -> bool {
    let translations = TRANSLATIONS.read().unwrap();
    translations.contains_key(key)
}

/// 检查是否存在指定key和语言的翻译
pub fn has_translation_for_lang(key: &str, lang: &str) -> bool {
    let translations = TRANSLATIONS.read().unwrap();
    if let Some(lang_translations) = translations.get(key) {
        lang_translations.contains_key(lang)
    } else {
        false
    }
}

/// 获取所有支持的key列表
pub fn get_all_keys() -> Vec<String> {
    let translations = TRANSLATIONS.read().unwrap();
    translations.keys().cloned().collect()
}

/// 获取指定key支持的所有语言
pub fn get_supported_languages(key: &str) -> Vec<String> {
    let translations = TRANSLATIONS.read().unwrap();
    if let Some(lang_translations) = translations.get(key) {
        lang_translations.keys().cloned().collect()
    } else {
        Vec::new()
    }
}

/// 获取参数化翻译文本
/// 支持用 {参数名} 格式的参数替换
pub fn tf(key: &str, args: &[(&str, &str)]) -> String {
    let template = t(key);
    let mut result = template;

    for (param_name, param_value) in args {
        let placeholder = format!("{{{}}}", param_name);
        result = result.replace(&placeholder, param_value);
    }

    result
}

/// 获取指定语言的参数化翻译文本
/// 支持用 {参数名} 格式的参数替换
pub fn tf_with_lang(key: &str, lang: &str, args: &[(&str, &str)]) -> String {
    let template = t_with_lang(key, lang);
    let mut result = template;

    for (param_name, param_value) in args {
        let placeholder = format!("{{{}}}", param_name);
        result = result.replace(&placeholder, param_value);
    }

    result
}

/// 便捷宏：获取参数化翻译文本
#[macro_export]
macro_rules! tfm {
    ($key:expr, $($param:ident = $value:expr),* $(,)?) => {
        $crate::tf($key, &[$(($crate::__stringify!($param), $crate::__to_string!($value))),*])
    };
}

/// 辅助宏：将标识符转换为字符串
#[doc(hidden)]
#[macro_export]
macro_rules! __stringify {
    ($x:expr) => {
        stringify!($x)
    };
}

/// 辅助宏：将值转换为字符串
#[doc(hidden)]
#[macro_export]
macro_rules! __to_string {
    ($x:expr) => {
        &$x.to_string()
    };
}