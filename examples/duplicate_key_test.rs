/// 测试重复key报错

use std::collections::HashMap;
use rat_embed_lang::{set_language, register_translations, clear_translations};

fn main() {
    println!("=== 测试重复key报错 ===\n");

    // 第一组翻译
    let mut translations1 = HashMap::new();
    let mut hello_translations = HashMap::new();
    hello_translations.insert("zh-CN".to_string(), "你好".to_string());
    hello_translations.insert("en-US".to_string(), "Hello".to_string());
    translations1.insert("hello".to_string(), hello_translations);

    // 注册第一组
    register_translations(translations1);
    println!("成功注册第一组翻译");

    // 第二组翻译，包含重复的key "hello"
    let mut translations2 = HashMap::new();
    let mut world_translations = HashMap::new();
    world_translations.insert("zh-CN".to_string(), "世界".to_string());
    world_translations.insert("en-US".to_string(), "World".to_string());
    translations2.insert("world".to_string(), world_translations);

    // 重复的hello key
    let mut hello_translations2 = HashMap::new();
    hello_translations2.insert("zh-CN".to_string(), "您好".to_string());
    hello_translations2.insert("en-US".to_string(), "Hi".to_string());
    translations2.insert("hello".to_string(), hello_translations2);

    // 测试不同语言环境下的错误信息
    println!("测试中文环境下的重复key报错:");
    set_language("zh-CN");

    // 这里应该panic并显示中文错误信息
    // register_translations(translations2);

    println!("如果想测试重复key报错，请取消注释上面这行代码");

    println!("\n演示正确的使用方式 - 先清理再注册:");
    clear_translations();
    println!("已清理现有翻译数据");

    // 现在可以正常注册
    register_translations(translations2);
    println!("成功注册第二组翻译数据");
}