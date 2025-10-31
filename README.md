# 简单的嵌入式多语言框架

一个极其简单的Rust多语言框架，支持编译时嵌入和运行时语言切换。

## 特性

- 🚀 **简单易用** - 只需3个API：注册翻译、设置语言、获取翻译
- 📦 **零依赖** - 无外部依赖，纯Rust实现
- 🔒 **类型安全** - 编译时检查，防止意外覆盖
- 🌍 **多语言支持** - 支持任意语言代码（zh-CN, en-US, ja-JP等）
- 🔄 **自动fallback** - 当前语言 → 英语 → 第一个可用翻译
- 🛡️ **安全机制** - 重复key检测，防止意外覆盖
- 💬 **国际化错误** - 错误信息本身也支持多语言

## 快速开始

### 1. 添加依赖

```toml
[dependencies]
rat_embed_lang = "0.1.0"
```

### 2. 基础使用

```rust
use std::collections::HashMap;
use rat_embed_lang::{set_language, register_translations, t, get_language_from_env};

fn main() {
    // 准备翻译数据
    let mut translations = HashMap::new();

    // 添加"hello"的多语言翻译
    let mut hello_translations = HashMap::new();
    hello_translations.insert("zh-CN".to_string(), "你好".to_string());
    hello_translations.insert("en-US".to_string(), "Hello".to_string());
    hello_translations.insert("ja-JP".to_string(), "こんにちは".to_string());
    translations.insert("hello".to_string(), hello_translations);

    // 注册翻译数据
    register_translations(translations);

    // 自动检测系统语言或手动设置
    let system_lang = get_language_from_env();
    set_language(&system_lang);

    // 使用翻译
    println!("{}", t("hello")); // 根据当前语言显示对应翻译
}
```

## API文档

### 核心API

| 函数 | 描述 |
|------|------|
| `register_translations(translations)` | 注册翻译数据 |
| `set_language(lang)` | 设置当前语言 |
| `t(key)` | 获取当前语言的翻译文本 |

### 辅助API

| 函数 | 描述 |
|------|------|
| `current_language()` | 获取当前语言 |
| `clear_translations()` | 清理所有已注册的翻译 |
| `t_with_lang(key, lang)` | 获取指定语言的翻译 |
| `has_translation(key)` | 检查是否存在翻译 |
| `get_all_keys()` | 获取所有翻译key |

### 语言工具

| 函数 | 描述 |
|------|------|
| `normalize_language_code(code)` | 标准化语言代码 |
| `get_language_from_env()` | 从环境变量获取语言设置 |

## 使用指南

### 环境变量语言检测

框架按以下优先级检测语言：
1. `RAT_LANG` - 应用特定的语言设置
2. `LANG` - 系统语言环境变量
3. `en-US` - 默认英语

```rust
// 使用应用特定的语言设置
std::env::set_var("RAT_LANG", "zh-CN");
let lang = get_language_from_env(); // 返回 "zh-CN"
```

### Fallback机制

当请求的翻译不存在时，按以下顺序fallback：
1. 当前语言的翻译
2. 英语翻译 (`en-US`)
3. 第一个可用的翻译
4. 返回 `[key]` 格式的key名

```rust
// 如果当前语言是"fr-FR"但只有中文和英文翻译
set_language("fr-FR");
println!("{}", t("hello")); // 会fallback到英语"Hello"
```

### 重复key保护

框架会检测重复的翻译key，防止意外覆盖：

```rust
// 第一次注册
register_translations(translations1); // ✅ 成功

// 第二次注册包含重复key
register_translations(translations2); // ❌ panic: 重复key检测

// 正确的重新注册方式
clear_translations(); // 先清理
register_translations(translations2); // ✅ 成功
```

### 多语言错误信息

错误信息根据当前语言环境显示：

```rust
set_language("zh-CN");
// 错误信息显示：翻译key重复: hello, world. 请检查...

set_language("en-US");
// 错误信息显示：Duplicate translation keys: hello, world. Please check...
```

## 高级用法

### 模块化翻译

你可以将翻译分成多个模块：

```rust
// ui_translations.rs
pub fn get_ui_translations() -> HashMap<String, HashMap<String, String>> {
    let mut translations = HashMap::new();

    let mut button_ok = HashMap::new();
    button_ok.insert("zh-CN", "确定".to_string());
    button_ok.insert("en-US", "OK".to_string());
    translations.insert("button_ok".to_string(), button_ok);

    translations
}

// main.rs
register_translations(get_ui_translations());
register_translations(get_error_translations());
```

### 动态语言切换

```rust
fn switch_language(lang: &str) {
    set_language(lang);
    // 所有翻译调用都会使用新语言
    update_ui();
}
```

## 示例项目

查看 `examples/` 目录中的完整示例：

- `basic_usage.rs` - 基础使用示例
- `duplicate_key_test.rs` - 重复key检测测试

运行示例：
```bash
cargo run --example basic_usage
cargo run --example duplicate_key_test
```

## 设计理念

### 简单优先
- 只提供最核心的功能
- 无复杂的宏或配置
- 直接使用HashMap，学习成本低

### 安全可靠
- 重复key检测防止意外覆盖
- 线程安全的状态管理
- 明确的错误提示

### 灵活扩展
- 支持任意语言代码
- 用户完全控制翻译数据结构
- 易于集成到现有项目

## 许可证

LGPL v3

## 贡献

欢迎提交Issue和Pull Request！

## 更新日志

### v0.1.0
- 初始版本
- 基础多语言支持
- 重复key检测
- 国际化错误信息