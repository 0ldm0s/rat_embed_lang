/// 参数化翻译示例

use std::collections::HashMap;
use rat_embed_lang::{set_language, register_translations, tf, tf_with_lang, tfm};

fn main() {
    println!("=== 参数化翻译示例 ===\n");

    // 准备包含参数的翻译数据
    let mut translations = HashMap::new();

    // 欢迎消息 - 包含name参数
    let mut welcome_translations = HashMap::new();
    welcome_translations.insert("zh-CN".to_string(), "欢迎，{name}！".to_string());
    welcome_translations.insert("en-US".to_string(), "Welcome, {name}!".to_string());
    welcome_translations.insert("ja-JP".to_string(), "ようこそ、{name}さん！".to_string());
    welcome_translations.insert("ko-KR".to_string(), "{name}님, 환영합니다!".to_string());
    translations.insert("welcome".to_string(), welcome_translations);

    // 登录消息 - 包含username和time参数
    let mut login_translations = HashMap::new();
    login_translations.insert("zh-CN".to_string(), "{username}在{time}登录了系统".to_string());
    login_translations.insert("en-US".to_string(), "{username} logged in at {time}".to_string());
    login_translations.insert("ja-JP".to_string(), "{username}が{time}にログインしました".to_string());
    login_translations.insert("ko-KR".to_string(), "{username}이(가) {time}에 로그인했습니다".to_string());
    translations.insert("login_message".to_string(), login_translations);

    // 错误消息 - 包含error和code参数
    let mut error_translations = HashMap::new();
    error_translations.insert("zh-CN".to_string(), "错误[{code}]: {error}".to_string());
    error_translations.insert("en-US".to_string(), "Error [{code}]: {error}".to_string());
    error_translations.insert("ja-JP".to_string(), "エラー[{code}]: {error}".to_string());
    error_translations.insert("ko-KR".to_string(), "오류[{code}]: {error}".to_string());
    translations.insert("error_message".to_string(), error_translations);

    // 注册翻译数据
    register_translations(translations);

    // 测试不同语言下的参数化翻译
    let languages = vec!["zh-CN", "en-US", "ja-JP", "ko-KR"];
    let test_name = "张三";
    let test_username = "John";
    let test_time = "14:30";
    let test_error = "文件未找到";
    let test_code = "404";

    for lang in languages {
        set_language(lang);
        println!("=== 语言: {} ===", lang);

        // 方法1：使用tf函数
        let welcome = tf("welcome", &[("name", test_name)]);
        println!("欢迎: {}", welcome);

        let login = tf("login_message", &[("username", test_username), ("time", test_time)]);
        println!("登录: {}", login);

        let error = tf("error_message", &[("error", test_error), ("code", test_code)]);
        println!("错误: {}", error);

        // 方法2：使用tf_with_lang指定特定语言
        let welcome_en = tf_with_lang("welcome", "en-US", &[("name", "Alice")]);
        println!("英语欢迎: {}", welcome_en);

        // 方法3：使用便捷宏tfm
        let welcome_macro = tfm!("welcome", name = "Bob");
        println!("宏欢迎: {}", welcome_macro);

        let login_macro = tfm!("login_message", username = "admin", time = "09:00");
        println!("宏登录: {}", login_macro);

        println!();
    }

    // 测试不存在的参数（应该保持原样）
    println!("=== 测试不存在的参数 ===");
    set_language("zh-CN");
    let incomplete = tf("welcome", &[("nonexistent", "value")]);
    println!("不完整参数: {}", incomplete); // 应该显示 "欢迎，{name}！"

    // 测试空参数列表
    let no_params = tf("welcome", &[]);
    println!("无参数: {}", no_params); // 应该显示 "欢迎，{name}！"

    println!();
}