/// 基础使用示例

use std::collections::HashMap;
use rat_embed_lang::{set_language, register_translations, t, get_language_from_env};

fn main() {
    println!("=== 基础多语言使用示例 ===\n");

    // 准备翻译数据
    let mut translations = HashMap::new();

    // 基础问候语
    let mut hello_translations = HashMap::new();
    hello_translations.insert("zh-CN".to_string(), "你好".to_string());
    hello_translations.insert("en-US".to_string(), "Hello".to_string());
    hello_translations.insert("ja-JP".to_string(), "こんにちは".to_string());
    hello_translations.insert("ko-KR".to_string(), "안녕하세요".to_string());
    translations.insert("hello".to_string(), hello_translations);

    // 再见
    let mut goodbye_translations = HashMap::new();
    goodbye_translations.insert("zh-CN".to_string(), "再见".to_string());
    goodbye_translations.insert("en-US".to_string(), "Goodbye".to_string());
    goodbye_translations.insert("ja-JP".to_string(), "さようなら".to_string());
    goodbye_translations.insert("ko-KR".to_string(), "안녕히 가세요".to_string());
    translations.insert("goodbye".to_string(), goodbye_translations);

    // 欢迎消息
    let mut welcome_translations = HashMap::new();
    welcome_translations.insert("zh-CN".to_string(), "欢迎使用多语言框架".to_string());
    welcome_translations.insert("en-US".to_string(), "Welcome to the i18n framework".to_string());
    welcome_translations.insert("ja-JP".to_string(), "多言語フレームワークへようこそ".to_string());
    welcome_translations.insert("ko-KR".to_string(), "다국어 프레임워크에 오신 것을 환영합니다".to_string());
    translations.insert("welcome".to_string(), welcome_translations);

    // 错误消息
    let mut error_translations = HashMap::new();
    error_translations.insert("zh-CN".to_string(), "发生错误".to_string());
    error_translations.insert("en-US".to_string(), "Error occurred".to_string());
    error_translations.insert("ja-JP".to_string(), "エラーが発生しました".to_string());
    error_translations.insert("ko-KR".to_string(), "오류가 발생했습니다".to_string());
    translations.insert("error".to_string(), error_translations);

    // 注册翻译数据
    register_translations(translations);

    // 自动检测系统语言
    let system_lang = get_language_from_env();
    println!("检测到系统语言: {}", system_lang);
    set_language(&system_lang);

    println!("当前语言下显示:");
    println!("问候: {}", t("hello"));
    println!("欢迎: {}", t("welcome"));
    println!("再见: {}", t("goodbye"));
    println!("错误: {}", t("error"));
    println!();

    // 手动切换语言演示
    let languages = vec!["zh-CN", "en-US", "ja-JP", "ko-KR"];

    for lang in languages {
        set_language(lang);
        println!("=== 语言: {} ===", lang);
        println!("问候: {}", t("hello"));
        println!("欢迎: {}", t("welcome"));
        println!("再见: {}", t("goodbye"));
        println!("错误: {}", t("error"));
        println!();
    }

    // 测试不存在的key
    println!("=== 测试不存在的key ===");
    set_language("zh-CN");
    println!("不存在的key: {}", t("nonexistent"));
    println!();
}