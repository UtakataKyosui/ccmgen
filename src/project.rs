use std::path::{Path, PathBuf};
use std::fs;
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