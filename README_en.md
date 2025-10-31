# Simple Embedded Internationalization Framework

An extremely simple Rust internationalization framework that supports compile-time embedding and runtime language switching.

## Features

- 🚀 **Easy to Use** - Just 3 APIs: register translations, set language, get translation
- 📦 **Zero Dependencies** - No external dependencies, pure Rust implementation
- 🔒 **Type Safe** - Compile-time checks, prevents accidental overwrites
- 🌍 **Multi-language Support** - Supports any language code (zh-CN, en-US, ja-JP, etc.)
- 🔄 **Automatic Fallback** - Current language → English → First available translation
- 🛡️ **Safety Mechanisms** - Duplicate key detection, prevents accidental overwrites
- 💬 **Internationalized Errors** - Error messages themselves support multiple languages

## Quick Start

### 1. Add Dependency

```toml
[dependencies]
rat_embed_lang = "0.1.0"
```

### 2. Basic Usage

```rust
use std::collections::HashMap;
use rat_embed_lang::{set_language, register_translations, t, get_language_from_env};

fn main() {
    // Prepare translation data
    let mut translations = HashMap::new();

    // Add multilingual translations for "hello"
    let mut hello_translations = HashMap::new();
    hello_translations.insert("zh-CN".to_string(), "你好".to_string());
    hello_translations.insert("en-US".to_string(), "Hello".to_string());
    hello_translations.insert("ja-JP".to_string(), "こんにちは".to_string());
    translations.insert("hello".to_string(), hello_translations);

    // Register translation data
    register_translations(translations);

    // Auto-detect system language or set manually
    let system_lang = get_language_from_env();
    set_language(&system_lang);

    // Use translations
    println!("{}", t("hello")); // Displays corresponding translation based on current language
}
```

## API Documentation

### Core APIs

| Function | Description |
|----------|-------------|
| `register_translations(translations)` | Register translation data |
| `set_language(lang)` | Set current language |
| `t(key)` | Get translation text in current language |

### Helper APIs

| Function | Description |
|----------|-------------|
| `current_language()` | Get current language |
| `clear_translations()` | Clear all registered translations |
| `t_with_lang(key, lang)` | Get translation in specified language |
| `has_translation(key)` | Check if translation exists |
| `get_all_keys()` | Get all translation keys |

### Language Utilities

| Function | Description |
|----------|-------------|
| `normalize_language_code(code)` | Normalize language code |
| `get_language_from_env()` | Get language setting from environment variables |

## Usage Guide

### Environment Variable Language Detection

The framework detects languages in the following priority:
1. `RAT_LANG` - Application-specific language setting
2. `LANG` - System language environment variable
3. `en-US` - Default English

```rust
// Use application-specific language setting
std::env::set_var("RAT_LANG", "zh-CN");
let lang = get_language_from_env(); // Returns "zh-CN"
```

### Fallback Mechanism

When requested translation doesn't exist, fallback in the following order:
1. Current language translation
2. English translation (`en-US`)
3. First available translation
4. Return `[key]` formatted key name

```rust
// If current language is "fr-FR" but only Chinese and English translations exist
set_language("fr-FR");
println!("{}", t("hello")); // Will fallback to English "Hello"
```

### Duplicate Key Protection

The framework detects duplicate translation keys to prevent accidental overwrites:

```rust
// First registration
register_translations(translations1); // ✅ Success

// Second registration with duplicate keys
register_translations(translations2); // ❌ Panic: duplicate key detection

// Correct way to re-register
clear_translations(); // Clear first
register_translations(translations2); // ✅ Success
```

### Internationalized Error Messages

Error messages display based on current language environment:

```rust
set_language("zh-CN");
// Error message shows: 翻译key重复: hello, world. 请检查...

set_language("en-US");
// Error message shows: Duplicate translation keys: hello, world. Please check...
```

## Advanced Usage

### Modular Translations

You can split translations into multiple modules:

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

### Dynamic Language Switching

```rust
fn switch_language(lang: &str) {
    set_language(lang);
    // All translation calls will use new language
    update_ui();
}
```

## Example Projects

See complete examples in the `examples/` directory:

- `basic_usage.rs` - Basic usage example
- `duplicate_key_test.rs` - Duplicate key detection test

Run examples:
```bash
cargo run --example basic_usage
cargo run --example duplicate_key_test
```

## Design Philosophy

### Simplicity First
- Only provide core functionality
- No complex macros or configurations
- Direct HashMap usage, low learning curve

### Safe and Reliable
- Duplicate key detection prevents accidental overwrites
- Thread-safe state management
- Clear error messages

### Flexible and Extensible
- Support any language code
- Users completely control translation data structure
- Easy to integrate into existing projects

## License

LGPL v3

## Contributing

Issues and Pull Requests are welcome!

## Changelog

### v0.1.0
- Initial version
- Basic multi-language support
- Duplicate key detection
- Internationalized error messages