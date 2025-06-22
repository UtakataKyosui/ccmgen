use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProjectType {
    RustNormal,
    RustWasm,
    JavaScript,
    TypeScript,
    NodeJs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectInfo {
    pub project_type: ProjectType,
    pub name: String,
    pub path: PathBuf,
    pub features: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectStructure {
    pub source_files: Vec<PathBuf>,
    pub test_files: Vec<PathBuf>,
    pub config_files: Vec<PathBuf>,
    pub doc_files: Vec<PathBuf>,
    pub dependencies: HashMap<String, String>,
    pub scripts: HashMap<String, String>,
    pub entry_points: Vec<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectContext {
    pub info: ProjectInfo,
    pub structure: ProjectStructure,
    pub suggested_commands: Vec<String>,
}

pub struct ProjectDetector;

impl ProjectDetector {
    pub fn detect_project(path: &Path) -> Option<ProjectInfo> {
        if let Some(cargo_info) = Self::detect_rust_project(path) {
            Some(cargo_info)
        } else if let Some(js_info) = Self::detect_js_project(path) {
            Some(js_info)
        } else {
            None
        }
    }

    pub fn analyze_project_structure(project: &ProjectInfo) -> ProjectStructure {
        let mut structure = ProjectStructure::new();
        
        structure.scan_directory(&project.path);
        structure.extract_metadata(project);
        structure
    }

    pub fn create_project_context(path: &Path) -> Option<ProjectContext> {
        let info = Self::detect_project(path)?;
        let structure = Self::analyze_project_structure(&info);
        let suggested_commands = Self::suggest_commands(&info, &structure);

        Some(ProjectContext {
            info,
            structure,
            suggested_commands,
        })
    }

    fn suggest_commands(info: &ProjectInfo, structure: &ProjectStructure) -> Vec<String> {
        let mut commands = Vec::new();

        // プロジェクト固有のコマンド提案
        match info.project_type {
            ProjectType::RustNormal => {
                if !structure.test_files.is_empty() {
                    commands.push("run-specific-test".to_string());
                }
                if structure.dependencies.contains_key("tokio") || structure.dependencies.contains_key("async-std") {
                    commands.push("async-refactor".to_string());
                }
                if structure.dependencies.contains_key("serde") {
                    commands.push("serialization-helper".to_string());
                }
            },
            ProjectType::RustWasm => {
                commands.push("wasm-size-analysis".to_string());
                commands.push("js-binding-generator".to_string());
                if structure.config_files.iter().any(|p| p.file_name().unwrap_or_default() == "webpack.config.js") {
                    commands.push("webpack-wasm-optimization".to_string());
                }
            },
            ProjectType::JavaScript | ProjectType::TypeScript => {
                if structure.scripts.contains_key("test") {
                    commands.push("test-coverage-analysis".to_string());
                }
                if structure.dependencies.contains_key("react") {
                    commands.push("react-component-generator".to_string());
                }
                if structure.dependencies.contains_key("vue") {
                    commands.push("vue-component-generator".to_string());
                }
            },
            ProjectType::NodeJs => {
                if structure.dependencies.contains_key("express") {
                    commands.push("express-route-generator".to_string());
                }
                if structure.dependencies.contains_key("mongoose") || structure.dependencies.contains_key("prisma") {
                    commands.push("database-model-generator".to_string());
                }
            },
        }

        // ファイル構造に基づく提案
        if structure.doc_files.is_empty() {
            commands.push("documentation-generator".to_string());
        }
        
        if structure.config_files.iter().any(|p| p.file_name().unwrap_or_default() == "Dockerfile") {
            commands.push("docker-optimization".to_string());
        }

        if structure.config_files.iter().any(|p| p.file_name().unwrap_or_default() == ".github") {
            commands.push("ci-cd-enhancement".to_string());
        }

        commands
    }

    fn detect_rust_project(path: &Path) -> Option<ProjectInfo> {
        let cargo_path = path.join("Cargo.toml");
        if !cargo_path.exists() {
            return None;
        }

        let cargo_content = fs::read_to_string(&cargo_path).ok()?;
        let cargo_toml: toml::Value = toml::from_str(&cargo_content).ok()?;
        
        let name = cargo_toml
            .get("package")
            .and_then(|p| p.get("name"))
            .and_then(|n| n.as_str())
            .unwrap_or("unknown")
            .to_string();

        let mut features = Vec::new();
        let project_type = if Self::is_wasm_project(&cargo_toml, path) {
            features.push("wasm".to_string());
            ProjectType::RustWasm
        } else {
            ProjectType::RustNormal
        };

        if cargo_toml.get("dependencies").is_some() {
            features.push("dependencies".to_string());
        }
        if cargo_toml.get("dev-dependencies").is_some() {
            features.push("dev-dependencies".to_string());
        }

        Some(ProjectInfo {
            project_type,
            name,
            path: path.to_path_buf(),
            features,
        })
    }

    fn detect_js_project(path: &Path) -> Option<ProjectInfo> {
        let package_path = path.join("package.json");
        if !package_path.exists() {
            return None;
        }

        let package_content = fs::read_to_string(&package_path).ok()?;
        let package_json: serde_json::Value = serde_json::from_str(&package_content).ok()?;
        
        let name = package_json
            .get("name")
            .and_then(|n| n.as_str())
            .unwrap_or("unknown")
            .to_string();

        let mut features = Vec::new();
        let project_type = if Self::has_typescript_config(path) {
            features.push("typescript".to_string());
            ProjectType::TypeScript
        } else if Self::is_node_project(&package_json) {
            features.push("nodejs".to_string());
            ProjectType::NodeJs
        } else {
            ProjectType::JavaScript
        };

        if package_json.get("dependencies").is_some() {
            features.push("dependencies".to_string());
        }
        if package_json.get("devDependencies").is_some() {
            features.push("devDependencies".to_string());
        }
        if package_json.get("scripts").is_some() {
            features.push("scripts".to_string());
        }

        Some(ProjectInfo {
            project_type,
            name,
            path: path.to_path_buf(),
            features,
        })
    }

    fn is_wasm_project(cargo_toml: &toml::Value, path: &Path) -> bool {
        // Check for wasm-pack configuration
        if cargo_toml.get("package")
            .and_then(|p| p.get("metadata"))
            .and_then(|m| m.get("wasm-pack"))
            .is_some() {
            return true;
        }

        // Check for wasm-bindgen dependency
        if let Some(deps) = cargo_toml.get("dependencies") {
            if deps.get("wasm-bindgen").is_some() {
                return true;
            }
        }

        // Check for lib crate-type
        if let Some(lib) = cargo_toml.get("lib") {
            if let Some(crate_type) = lib.get("crate-type") {
                if let Some(types) = crate_type.as_array() {
                    return types.iter().any(|t| t.as_str() == Some("cdylib"));
                }
            }
        }

        // Check for wasm-pack.json
        path.join("wasm-pack.json").exists()
    }

    fn has_typescript_config(path: &Path) -> bool {
        path.join("tsconfig.json").exists() || 
        path.join("tsconfig.build.json").exists() ||
        path.join("typescript.json").exists()
    }

    fn is_node_project(package_json: &serde_json::Value) -> bool {
        // Check for Node.js specific fields
        if let Some(main) = package_json.get("main") {
            if let Some(main_str) = main.as_str() {
                return main_str.ends_with(".js") || main_str.ends_with(".mjs");
            }
        }

        // Check for Node.js dependencies
        if let Some(deps) = package_json.get("dependencies") {
            let node_deps = ["express", "fastify", "koa", "@types/node"];
            for dep in node_deps {
                if deps.get(dep).is_some() {
                    return true;
                }
            }
        }

        // Check for engines field
        if package_json.get("engines")
            .and_then(|e| e.get("node"))
            .is_some() {
            return true;
        }

        false
    }
}

impl ProjectStructure {
    pub fn new() -> Self {
        Self {
            source_files: Vec::new(),
            test_files: Vec::new(),
            config_files: Vec::new(),
            doc_files: Vec::new(),
            dependencies: HashMap::new(),
            scripts: HashMap::new(),
            entry_points: Vec::new(),
        }
    }

    pub fn scan_directory(&mut self, path: &Path) {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    self.categorize_file(&path);
                } else if path.is_dir() && !self.should_skip_directory(&path) {
                    self.scan_directory(&path);
                }
            }
        }
    }

    fn categorize_file(&mut self, path: &Path) {
        if let Some(extension) = path.extension().and_then(|s| s.to_str()) {
            match extension {
                "rs" => {
                    if path.to_string_lossy().contains("test") || 
                       path.file_name().unwrap_or_default().to_string_lossy().starts_with("test_") {
                        self.test_files.push(path.to_path_buf());
                    } else {
                        self.source_files.push(path.to_path_buf());
                    }
                },
                "js" | "jsx" | "ts" | "tsx" => {
                    if path.to_string_lossy().contains("test") || 
                       path.to_string_lossy().contains("spec") ||
                       path.file_name().unwrap_or_default().to_string_lossy().contains(".test.") {
                        self.test_files.push(path.to_path_buf());
                    } else {
                        self.source_files.push(path.to_path_buf());
                    }
                },
                "toml" | "json" | "yaml" | "yml" | "config" => {
                    self.config_files.push(path.to_path_buf());
                },
                "md" | "rst" | "txt" => {
                    self.doc_files.push(path.to_path_buf());
                },
                _ => {}
            }
        }

        // 特別なファイル名の処理
        if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
            match filename {
                "Dockerfile" | ".dockerignore" | "docker-compose.yml" | "docker-compose.yaml" => {
                    self.config_files.push(path.to_path_buf());
                },
                "main.rs" | "lib.rs" | "index.js" | "index.ts" | "app.js" | "app.ts" => {
                    self.entry_points.push(path.to_path_buf());
                },
                _ => {}
            }
        }
    }

    fn should_skip_directory(&self, path: &Path) -> bool {
        if let Some(dirname) = path.file_name().and_then(|s| s.to_str()) {
            matches!(dirname, "target" | "node_modules" | ".git" | "dist" | "build" | ".next")
        } else {
            false
        }
    }

    pub fn extract_metadata(&mut self, project: &ProjectInfo) {
        match project.project_type {
            ProjectType::RustNormal | ProjectType::RustWasm => {
                self.extract_rust_metadata(&project.path);
            },
            ProjectType::JavaScript | ProjectType::TypeScript | ProjectType::NodeJs => {
                self.extract_js_metadata(&project.path);
            },
        }
    }

    fn extract_rust_metadata(&mut self, path: &Path) {
        let cargo_path = path.join("Cargo.toml");
        if let Ok(content) = fs::read_to_string(&cargo_path) {
            if let Ok(cargo_toml) = toml::from_str::<toml::Value>(&content) {
                // 依存関係の抽出
                if let Some(deps) = cargo_toml.get("dependencies") {
                    if let Some(deps_table) = deps.as_table() {
                        for (name, value) in deps_table {
                            let version = match value {
                                toml::Value::String(v) => v.clone(),
                                toml::Value::Table(t) => {
                                    t.get("version").and_then(|v| v.as_str()).unwrap_or("*").to_string()
                                },
                                _ => "*".to_string(),
                            };
                            self.dependencies.insert(name.clone(), version);
                        }
                    }
                }

                // スクリプト（ビルドスクリプトなど）の抽出
                if let Some(package) = cargo_toml.get("package") {
                    if let Some(build) = package.get("build") {
                        if let Some(build_script) = build.as_str() {
                            self.scripts.insert("build".to_string(), build_script.to_string());
                        }
                    }
                }
            }
        }
    }

    fn extract_js_metadata(&mut self, path: &Path) {
        let package_path = path.join("package.json");
        if let Ok(content) = fs::read_to_string(&package_path) {
            if let Ok(package_json) = serde_json::from_str::<serde_json::Value>(&content) {
                // 依存関係の抽出
                if let Some(deps) = package_json.get("dependencies") {
                    if let Some(deps_obj) = deps.as_object() {
                        for (name, value) in deps_obj {
                            if let Some(version) = value.as_str() {
                                self.dependencies.insert(name.clone(), version.to_string());
                            }
                        }
                    }
                }

                // dev依存関係の抽出
                if let Some(dev_deps) = package_json.get("devDependencies") {
                    if let Some(dev_deps_obj) = dev_deps.as_object() {
                        for (name, value) in dev_deps_obj {
                            if let Some(version) = value.as_str() {
                                self.dependencies.insert(format!("dev:{}", name), version.to_string());
                            }
                        }
                    }
                }

                // スクリプトの抽出
                if let Some(scripts) = package_json.get("scripts") {
                    if let Some(scripts_obj) = scripts.as_object() {
                        for (name, value) in scripts_obj {
                            if let Some(script) = value.as_str() {
                                self.scripts.insert(name.clone(), script.to_string());
                            }
                        }
                    }
                }
            }
        }
    }
}