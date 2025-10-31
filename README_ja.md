# シンプルな埋め込み国際化フレームワーク

コンパイル時埋め込みとランタイム言語切り替えをサポートする、非常にシンプルなRust国際化フレームワークです。

## 特徴

- 🚀 **使いやすい** - たった3つのAPI：翻訳登録、言語設定、翻訳取得
- 📦 **ゼロ依存** - 外部依存なし、純粋なRust実装
- 🔒 **タイプセーフ** - コンパイル時チェック、意図しない上書きを防止
- 🌍 **多言語対応** - 任意の言語コードをサポート（zh-CN、en-US、ja-JPなど）
- 🔄 **自動フォールバック** - 現在の言語 → 英語 → 最初の利用可能な翻訳
- 🛡️ **安全機構** - 重複キー検出、意図しない上書きを防止
- 💬 **国際化エラー** - エラーメッセージ自体が多言語をサポート

## クイックスタート

### 1. 依存関係を追加

```toml
[dependencies]
rat_embed_lang = "0.1.0"
```

### 2. 基本的な使い方

詳細な使用例は `examples/` ディレクトリを参照：

- `basic_usage.rs` - 基本的な多言語使用
- `duplicate_key_test.rs` - 重複キー検出とエラー処理
- `parameterized_translation.rs` - パラメータ化翻訳と動的テキスト置換

サンプル実行：
```bash
cargo run --example basic_usage
cargo run --example parameterized_translation
```

## APIドキュメント

### コアAPI

| 関数 | 説明 |
|------|------|
| `register_translations(translations)` | 翻訳データを登録 |
| `set_language(lang)` | 現在の言語を設定 |
| `t(key)` | 現在の言語で翻訳テキストを取得 |
| `tf(key, args)` | パラメータ化翻訳テキストを取得 |

### ヘルパーAPI

| 関数 | 説明 |
|------|------|
| `current_language()` | 現在の言語を取得 |
| `clear_translations()` | 登録済みのすべての翻訳をクリア |
| `t_with_lang(key, lang)` | 指定された言語で翻訳を取得 |
| `tf_with_lang(key, lang, args)` | 指定言語のパラメータ化翻訳を取得 |
| `has_translation(key)` | 翻訳が存在するかチェック |
| `get_all_keys()` | すべての翻訳キーを取得 |

### 言語ユーティリティ

| 関数 | 説明 |
|------|------|
| `normalize_language_code(code)` | 言語コードを正規化 |
| `get_language_from_env()` | 環境変数から言語設定を取得 |

## 使用ガイド

### 環境変数による言語検出

フレームワークは以下の優先順位で言語を検出します：
1. `RAT_LANG` - アプリケーション固有の言語設定
2. `LANG` - システム言語環境変数
3. `en-US` - デフォルト英語

```rust
// アプリケーション固有の言語設定を使用
std::env::set_var("RAT_LANG", "zh-CN");
let lang = get_language_from_env(); // "zh-CN"を返す
```

### フォールバック機構

要求された翻訳が存在しない場合、以下の順序でフォールバックします：
1. 現在の言語の翻訳
2. 英語翻訳（`en-US`）
3. 最初の利用可能な翻訳
4. `[key]`形式のキー名を返す

```rust
// 現在の言語が"fr-FR"だが中国語と英語の翻訳しか存在しない場合
set_language("fr-FR");
println!("{}", t("hello")); // 英語の"Hello"にフォールバック
```

### 重複キー保護

フレームワークは重複する翻訳キーを検出し、意図しない上書きを防止します：

```rust
// 最初の登録
register_translations(translations1); // ✅ 成功

// 重複キーを含む2回目の登録
register_translations(translations2); // ❌ パニック：重複キー検出

// 正しい再登録方法
clear_translations(); // 最初にクリア
register_translations(translations2); // ✅ 成功
```

### 国際化エラーメッセージ

エラーメッセージは現在の言語環境に基づいて表示されます：

```rust
set_language("zh-CN");
// エラーメッセージ表示：翻译key重复: hello, world. 请检查...

set_language("en-US");
// エラーメッセージ表示：Duplicate translation keys: hello, world. Please check...
```

## 高度な使い方

### モジュラー翻訳

翻訳を複数のモジュールに分割できます：

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

### 動的言語切り替え

```rust
fn switch_language(lang: &str) {
    set_language(lang);
    // すべての翻訳呼び出しが新しい言語を使用
    update_ui();
}
```

## サンプルプロジェクト

`examples/`ディレクトリの完全なサンプルを参照：

- `basic_usage.rs` - 基本的な使用例
- `duplicate_key_test.rs` - 重複キー検出テスト

サンプル実行：
```bash
cargo run --example basic_usage
cargo run --example duplicate_key_test
```

## 設計哲学

### シンプルさ第一
- コア機能のみを提供
- 複雑なマクロや設定なし
- 直接的なHashMap使用、学習コストが低い

### 安全で信頼性
- 重複キー検出が意図しない上書きを防止
- スレッドセーフな状態管理
- 明確なエラーメッセージ

### 柔軟で拡張可能
- 任意の言語コードをサポート
- ユーザーが翻訳データ構造を完全に制御
- 既存プロジェクトへの統合が容易

## ライセンス

LGPL v3

## 貢献

IssueとPull Requestを歓迎します！

## 変更ログ

### v0.1.0
- 初期バージョン
- 基本的な多言語サポート
- 重複キー検出
- 国際化エラーメッセージ