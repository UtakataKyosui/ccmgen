# ccmgen - Claude Code Command Generator

ccmgenは、プロジェクトタイプを自動検出してClaude Code用のユーザーコマンドを生成するRust製CLIツールです。

## 特徴

- **自動プロジェクト検出**: Cargo.toml、package.jsonなどから言語・環境を自動判別
- **多言語対応**: Rust(Normal/WASM)、JavaScript、TypeScript、Node.js
- **専門テンプレート**: 各言語に特化した8+種類のコマンドテンプレート
- **プロジェクト固有分析**: ファイル構造・依存関係・スクリプトを詳細分析
- **スマートテンプレート**: プロジェクトコンテキストに基づく動的コマンド生成
- **設定管理**: TOML形式でカスタムテンプレートの管理が可能
- **Claude Code統合**: ~/.claude/commandsに直接コマンドファイルを生成

## サポート環境

| 言語/環境 | 検出条件 | 生成テンプレート例 |
|----------|----------|-------------------|
| **Rust (Normal)** | Cargo.toml | パフォーマンス分析、テスト生成、ドキュメント追加 |
| **Rust (WASM)** | wasm-bindgen依存、cdylib設定 | wasm-bindgenラッパー、JS相互運用、メモリ最適化 |
| **JavaScript** | package.json | ES6+現代化、Promise変換、バンドル分析 |
| **TypeScript** | tsconfig.json | 型注釈、インターフェース設計、strict修正 |
| **Node.js** | Node.js特有の依存関係 | Expressミドルウェア、API作成、認証実装 |

## インストール

```bash
# リポジトリをクローン
git clone https://github.com/UtakataKyosui/ccmgen.git
cd ccmgen

# ビルドして実行
cargo build --release
./target/release/ccmgen --help
```

## 使い方

### 基本的な使用方法

```bash
# 現在のディレクトリのプロジェクトを検出してコマンド生成
ccmgen init

# 特定のパスを指定
ccmgen init --path /path/to/project

# 言語を手動指定
ccmgen init --lang rust
```

### プロジェクト検出・分析

```bash
# プロジェクト情報を表示
ccmgen detect

# 特定のパスを検出
ccmgen detect --path /path/to/project

# プロジェクト詳細分析と推奨コマンド表示
ccmgen analyze

# 特定のパスを詳細分析
ccmgen analyze --path /path/to/project
```

### コマンド管理

```bash
# 作成済みコマンド一覧表示
ccmgen list

# 特定のコマンドを削除
ccmgen remove command-name

# 設定ファイル初期化
ccmgen config
```

## 生成されるファイル

- **コマンドファイル**: `~/.claude/commands/*.md`
- **設定ファイル**: `~/.claude/ccmgen.toml`

## 設定例

`~/.claude/ccmgen.toml`:

```toml
[default_settings]
auto_detect = true
prefer_typescript = true
include_tests = true
include_docs = true

[custom_templates.rust]
name = "custom-review"
description = "カスタムレビューテンプレート"
content = "このRustコードをセキュリティ観点からレビューしてください："
```

## 開発

```bash
# 開発用ビルド
cargo build

# 型チェック
cargo check

# テスト実行
cargo test

# CLI実行
./target/debug/ccmgen detect
```

## アーキテクチャ

- **main.rs**: CLI エントリーポイント
- **project.rs**: プロジェクト検出・構造分析エンジン
- **templates.rs**: 言語別テンプレート管理
- **smart_templates.rs**: プロジェクトコンテキスト対応テンプレート
- **commands.rs**: CLI コマンド実装
- **config.rs**: 設定管理システム

## コントリビュート

1. このリポジトリをフォーク
2. フィーチャーブランチを作成 (`git checkout -b feature/amazing-feature`)
3. 変更をコミット (`git commit -m 'Add amazing feature'`)
4. ブランチにプッシュ (`git push origin feature/amazing-feature`)
5. プルリクエストを作成

## ライセンス

このプロジェクトはMITライセンスの下で公開されています。詳細は[LICENSE](LICENSE)ファイルをご覧ください。

## Claude Codeとの連携

ccmgenで生成されたコマンドは、Claude Codeで以下のように使用できます：

```bash
# Claude Codeでプロジェクト固有のコマンドを実行
claude review-performance
claude generate-tests
claude add-documentation
```

各コマンドは検出されたプロジェクト情報（名前、種別、機能）を含んでいるため、より精密で関連性の高いコード支援を受けることができます。

## プロジェクト固有コマンド例

ccmgenは依存関係とファイル構造を分析し、プロジェクトに特化したコマンドを自動提案します：

### Rust プロジェクト
- **tokio/async-std検出時**: `async-refactor` - 非同期コード変換支援
- **serde依存時**: `serialization-helper` - シリアライゼーション実装
- **テストファイル存在時**: `run-specific-test` - 特定テスト実行

### JavaScript/TypeScript プロジェクト
- **React依存時**: `react-component-generator` - コンポーネント生成
- **Vue依存時**: `vue-component-generator` - Vueコンポーネント生成
- **テストスクリプト時**: `test-coverage-analysis` - カバレッジ分析

### Node.js プロジェクト
- **Express依存時**: `express-route-generator` - ルート生成
- **Mongoose/Prisma時**: `database-model-generator` - DBモデル生成

### 共通機能
- **Dockerファイル検出時**: `docker-optimization` - Docker最適化
- **GitHub Actions時**: `ci-cd-enhancement` - CI/CD改善
- **ドキュメント不足時**: `documentation-generator` - ドキュメント生成

これらの推奨コマンドは `ccmgen analyze` で確認でき、`ccmgen init` で一括生成されます。