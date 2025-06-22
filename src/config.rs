use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use dirs::home_dir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub custom_templates: HashMap<String, Vec<CustomTemplate>>,
    pub default_settings: DefaultSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomTemplate {
    pub name: String,
    pub description: String,
    pub content: String,
    pub language: Option<String>,
    pub project_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultSettings {
    pub auto_detect: bool,
    pub prefer_typescript: bool,
    pub include_tests: bool,
    pub include_docs: bool,
}

impl Default for DefaultSettings {
    fn default() -> Self {
        Self {
            auto_detect: true,
            prefer_typescript: true,
            include_tests: true,
            include_docs: true,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            custom_templates: HashMap::new(),
            default_settings: DefaultSettings::default(),
        }
    }
}

pub struct ConfigManager;

impl ConfigManager {
    pub fn get_config_path() -> PathBuf {
        home_dir()
            .expect("Could not get home directory")
            .join(".claude")
            .join("ccmgen.toml")
    }

    pub fn load_config() -> Config {
        let config_path = Self::get_config_path();
        
        if !config_path.exists() {
            return Config::default();
        }

        match fs::read_to_string(&config_path) {
            Ok(content) => {
                toml::from_str(&content).unwrap_or_else(|e| {
                    eprintln!("⚠️ 設定ファイルの読み込みに失敗しました: {}", e);
                    Config::default()
                })
            }
            Err(e) => {
                eprintln!("⚠️ 設定ファイルの読み込みに失敗しました: {}", e);
                Config::default()
            }
        }
    }

    pub fn save_config(config: &Config) -> Result<(), std::io::Error> {
        let config_path = Self::get_config_path();
        
        // ディレクトリが存在しない場合は作成
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(config)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        
        fs::write(&config_path, content)?;
        Ok(())
    }

    pub fn add_custom_template(
        language: &str,
        name: &str,
        description: &str,
        content: &str,
    ) -> Result<(), std::io::Error> {
        let mut config = Self::load_config();
        
        let template = CustomTemplate {
            name: name.to_string(),
            description: description.to_string(),
            content: content.to_string(),
            language: Some(language.to_string()),
            project_type: None,
        };

        config
            .custom_templates
            .entry(language.to_string())
            .or_insert_with(Vec::new)
            .push(template);

        Self::save_config(&config)
    }

    pub fn get_custom_templates_for_language(language: &str) -> Vec<CustomTemplate> {
        let config = Self::load_config();
        config
            .custom_templates
            .get(language)
            .cloned()
            .unwrap_or_default()
    }

    pub fn create_default_config() -> Result<(), std::io::Error> {
        let config = Config::default();
        Self::save_config(&config)?;
        println!("✅ デフォルト設定ファイルを作成しました: {}", Self::get_config_path().display());
        Ok(())
    }
}