use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use dialoguer::{theme::ColorfulTheme, Select};
use dirs::home_dir;

use crate::project::{ProjectDetector, ProjectContext};
use crate::templates::TemplateManager;
use crate::smart_templates::SmartTemplateManager;
use crate::config::ConfigManager;

/// 言語ごとのテンプレート定義
fn get_language_templates() -> Vec<(&'static str, Vec<(&'static str, &'static str)>)> {
    vec![
        (
            "rust",
            vec![
                ("review-performance", "Analyze the performance characteristics of this code and suggest improvements to make it faster or more efficient:"),
                ("generate-tests", "Generate unit tests for the following function using a common testing framework such as Rust’s built-in test framework:"),
                ("summarize-diff", "Summarize the following Git diff in natural language, describing what changed and why:"),
                ("readme-generator", "Based on this Rust project’s structure and content, generate a complete README.md file:"),
            ],
        ),
        (
            "typescript",
            vec![
                ("explain-code", "Explain the following TypeScript code in detail:"),
                ("add-jsdoc", "Add JSDoc comments to the following TypeScript code:"),
                ("generate-tests", "Generate unit tests for the following TypeScript function using Jest:"),
                ("commit-conventional", "Generate a Conventional Commit-style message for the following code diff:"),
            ],
        ),
    ]
}

/// コマンドを~/.claude/commandsに保存
fn save_command(name: &str, body: &str) -> io::Result<()> {
    let path = get_command_dir().join(format!("{name}.md"));
    let mut file = File::create(path)?;
    writeln!(file, "{}", body)?;
    Ok(())
}

/// ユーザーディレクトリのパス取得
fn get_command_dir() -> PathBuf {
    home_dir()
        .expect("Could not get home directory")
        .join(".claude/commands")
}

/// `ccmgen detect` コマンド本体
pub fn detect(path: Option<String>) {
    let target_path = path
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().expect("カレントディレクトリの取得に失敗しました"));

    match ProjectDetector::detect_project(&target_path) {
        Some(project) => {
            println!("🔍 プロジェクト検出結果:");
            println!("  名前: {}", project.name);
            println!("  種別: {:?}", project.project_type);
            println!("  パス: {}", project.path.display());
            if !project.features.is_empty() {
                println!("  機能: {}", project.features.join(", "));
            }
        }
        None => {
            println!("❓ 対応するプロジェクトタイプが見つかりませんでした");
        }
    }
}

/// `ccmgen init` コマンド本体
pub fn init(lang: Option<String>, repo: Option<String>, path: Option<String>) {
    if let Some(repo_url) = repo {
        println!("🔗 GitHubテンプレートのダウンロードは未実装です: {repo_url}");
        // TODO: GitHub連携処理（git2またはreqwest+zip）
        return;
    }

    let target_path = path
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().expect("カレントディレクトリの取得に失敗しました"));

    // プロジェクト自動検出を試行
    let project_context = if lang.is_none() {
        ProjectDetector::create_project_context(&target_path)
    } else {
        None
    };

    let templates = if let Some(ref context) = project_context {
        println!("🔍 プロジェクトを検出しました: {} ({:?})", context.info.name, context.info.project_type);
        println!("💡 {} 個のプロジェクト固有コマンドを含みます", context.suggested_commands.len());
        SmartTemplateManager::create_enhanced_init_templates(context)
    } else {
        // 手動選択または古いロジック
        let legacy_templates = get_language_templates();
        let selected_lang = match lang {
            Some(lang) => lang,
            None => {
                let langs: Vec<&str> = legacy_templates.iter().map(|(l, _)| *l).collect();
                let selection = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("言語を選択してください")
                    .items(&langs)
                    .default(0)
                    .interact()
                    .unwrap();
                langs[selection].to_string()
            }
        };

        legacy_templates
            .iter()
            .find(|(l, _)| *l == selected_lang)
            .map(|(_, t)| t.iter().map(|(name, content)| (name.to_string(), content.to_string())).collect())
            .expect("テンプレートが見つかりません")
    };

    let cmd_dir = get_command_dir();
    fs::create_dir_all(&cmd_dir).expect("コマンドディレクトリの作成に失敗しました");

    for (name, body) in templates {
        match save_command(&name, &body) {
            Ok(_) => println!("✅ {}.md を作成しました", name),
            Err(e) => eprintln!("❌ {}.md の作成に失敗しました: {}", name, e),
        }
    }

    println!("🎉 完了しました: ~/.claude/commands にコマンドが作成されました");
}

/// `claude-cli list` コマンド
pub fn list() {
    let dir = get_command_dir();
    if !dir.exists() {
        println!("⚠️ ユーザーコマンドはまだ存在しません");
        return;
    }

    println!("📋 現在のユーザーコマンド一覧:");
    for entry in fs::read_dir(&dir).unwrap() {
        if let Ok(file) = entry {
            if let Some(name) = file.path().file_name() {
                println!(" - {}", name.to_string_lossy());
            }
        }
    }
}

/// `ccmgen remove <name>` コマンド
pub fn remove(name: &str) {
    let path = get_command_dir().join(format!("{name}.md"));
    if path.exists() {
        fs::remove_file(&path).expect("ファイル削除に失敗しました");
        println!("🗑️ 削除しました: {name}.md");
    } else {
        println!("❓ 指定されたコマンドが見つかりません: {name}.md");
    }
}

/// `ccmgen config` コマンド
pub fn config() {
    match ConfigManager::create_default_config() {
        Ok(_) => println!("🎉 設定ファイルが作成されました"),
        Err(e) => eprintln!("❌ 設定ファイルの作成に失敗しました: {}", e),
    }
}

/// `ccmgen analyze` コマンド
pub fn analyze(path: Option<String>) {
    let target_path = path
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().expect("カレントディレクトリの取得に失敗しました"));

    match ProjectDetector::create_project_context(&target_path) {
        Some(context) => {
            println!("🔍 プロジェクト詳細分析結果:");
            print_project_context(&context);
        }
        None => {
            println!("❓ 対応するプロジェクトタイプが見つかりませんでした");
        }
    }
}

fn print_project_context(context: &ProjectContext) {
    let info = &context.info;
    let structure = &context.structure;
    
    println!("📋 基本情報:");
    println!("  名前: {}", info.name);
    println!("  種別: {:?}", info.project_type);
    println!("  パス: {}", info.path.display());
    
    if !info.features.is_empty() {
        println!("  機能: {}", info.features.join(", "));
    }

    println!("\n📁 ファイル構成:");
    println!("  ソースファイル: {}個", structure.source_files.len());
    println!("  テストファイル: {}個", structure.test_files.len());
    println!("  設定ファイル: {}個", structure.config_files.len());
    println!("  ドキュメント: {}個", structure.doc_files.len());
    println!("  エントリーポイント: {}個", structure.entry_points.len());

    if !structure.dependencies.is_empty() {
        println!("\n📦 主要な依存関係:");
        let mut deps: Vec<_> = structure.dependencies.iter().collect();
        deps.sort_by_key(|(name, _)| name.as_str());
        for (name, version) in deps.iter().take(10) {
            println!("  {} = {}", name, version);
        }
        if structure.dependencies.len() > 10 {
            println!("  ... 他{}個", structure.dependencies.len() - 10);
        }
    }

    if !structure.scripts.is_empty() {
        println!("\n🔧 利用可能なスクリプト:");
        let mut scripts: Vec<_> = structure.scripts.iter().collect();
        scripts.sort_by_key(|(name, _)| name.as_str());
        for (name, command) in scripts.iter().take(8) {
            println!("  {}: {}", name, command);
        }
        if structure.scripts.len() > 8 {
            println!("  ... 他{}個", structure.scripts.len() - 8);
        }
    }

    if !context.suggested_commands.is_empty() {
        println!("\n💡 推奨プロジェクト固有コマンド:");
        for cmd in &context.suggested_commands {
            println!("  - {}", cmd);
        }
        println!("\n🚀 これらのコマンドを生成するには:");
        println!("   ccmgen init --path {}", info.path.display());
    }
}
