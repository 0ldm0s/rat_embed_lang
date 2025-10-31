//! 简单的嵌入式多语言框架
//!
//! 使用示例：
//! ```rust
//! use std::collections::HashMap;
//! use i18n_embed::{set_language, register_translations, t};
//!
//! // 准备翻译数据
//! let mut translations = HashMap::new();
//! let mut hello_translations = HashMap::new();
//! hello_translations.insert("zh-CN".to_string(), "你好".to_string());
//! hello_translations.insert("en-US".to_string(), "Hello".to_string());
//! translations.insert("hello".to_string(), hello_translations);
//!
//! // 注册翻译数据
//! register_translations(translations);
//!
//! // 设置语言
//! set_language("zh-CN");
//!
//! // 使用翻译
//! let hello = t("hello"); // 返回 "你好"
//! ```

pub mod language;
pub mod i18n;

// 重新导出核心API
pub use language::{normalize_language_code, get_language_from_env};
pub use i18n::{
    set_language, current_language, register_translations, t, t_with_lang,
    has_translation, has_translation_for_lang, get_all_keys, get_supported_languages
};
