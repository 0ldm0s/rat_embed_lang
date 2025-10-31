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

详细的用法示例请查看 `examples/` 目录：

- `basic_usage.rs` - 基础多语言使用
- `duplicate_key_test.rs` - 重复key检测和错误处理
- `parameterized_translation.rs` - 参数化翻译和动态文本替换

运行示例：
```bash
cargo run --example basic_usage
cargo run --example parameterized_translation
```

## API文档

### 核心API

| 函数 | 描述 |
|------|------|
| `register_translations(translations)` | 注册翻译数据 |
| `set_language(lang)` | 设置当前语言 |
| `t(key)` | 获取当前语言的翻译文本 |
| `tf(key, args)` | 获取参数化翻译文本 |

### 辅助API

| 函数 | 描述 |
|------|------|
| `current_language()` | 获取当前语言 |
| `clear_translations()` | 清理所有已注册的翻译 |
| `t_with_lang(key, lang)` | 获取指定语言的翻译 |
| `tf_with_lang(key, lang, args)` | 获取指定语言的参数化翻译 |
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

详细的环境变量检测示例请查看 `basic_usage.rs`

### Fallback机制

当请求的翻译不存在时，按以下顺序fallback：
1. 当前语言的翻译
2. 英语翻译 (`en-US`)
3. 第一个可用的翻译
4. 返回 `[key]` 格式的key名

详细的fallback机制示例请查看 `basic_usage.rs`

### 重复key保护

框架会检测重复的翻译key，防止意外覆盖：

重复key检测和错误处理的详细示例请查看 `duplicate_key_test.rs`

### 多语言错误信息

错误信息根据当前语言环境显示：

多语言错误信息的详细示例请查看 `duplicate_key_test.rs`

## 高级用法

### 参数化翻译

支持动态文本替换，使用 `{参数名}` 格式：

- `tf(key, args)` - 获取参数化翻译文本
- `tf_with_lang(key, lang, args)` - 获取指定语言的参数化翻译
- `tfm!` 宏 - 便捷的参数化翻译语法

详细用法请查看 `parameterized_translation.rs` 示例。

### 模块化翻译和动态语言切换

详细的高级用法示例请查看各个示例文件。

## 示例项目

查看 `examples/` 目录中的完整示例：

- `basic_usage.rs` - 基础多语言使用和fallback机制
- `duplicate_key_test.rs` - 重复key检测和错误处理
- `parameterized_translation.rs` - 参数化翻译和动态文本替换

运行示例：
```bash
cargo run --example basic_usage
cargo run --example duplicate_key_test
cargo run --example parameterized_translation
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