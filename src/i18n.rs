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
pub fn register_translations(translations: HashMap<String, HashMap<String, String>>) {
    let mut global_translations = TRANSLATIONS.write().unwrap();
    *global_translations = translations;
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