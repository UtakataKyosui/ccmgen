use crate::project::{ProjectContext, ProjectType};
use std::collections::HashMap;

pub struct SmartTemplateManager;

impl SmartTemplateManager {
    pub fn generate_context_specific_templates(context: &ProjectContext) -> Vec<(String, String)> {
        let mut templates = Vec::new();
        
        for suggested_cmd in &context.suggested_commands {
            if let Some((name, content)) = Self::create_template_for_command(suggested_cmd, context) {
                templates.push((name, content));
            }
        }
        
        templates
    }

    fn create_template_for_command(command: &str, context: &ProjectContext) -> Option<(String, String)> {
        let base_context = Self::build_context_string(context);
        
        match command {
            "run-specific-test" => Some((
                command.to_string(),
                format!("{}\n\nRun a specific test file or test function in this Rust project. Please specify the test to run:", base_context)
            )),
            "async-refactor" => Some((
                command.to_string(),
                format!("{}\n\nRefactor this synchronous Rust code to use async/await patterns, considering the tokio/async-std dependencies:", base_context)
            )),
            "serialization-helper" => Some((
                command.to_string(),
                format!("{}\n\nAdd Serde serialization/deserialization support to this Rust struct or enum:", base_context)
            )),
            "wasm-size-analysis" => Some((
                command.to_string(),
                format!("{}\n\nAnalyze and optimize this Rust WASM code for binary size reduction:", base_context)
            )),
            "js-binding-generator" => Some((
                command.to_string(),
                format!("{}\n\nGenerate JavaScript bindings for this Rust WASM function using wasm-bindgen:", base_context)
            )),
            "webpack-wasm-optimization" => Some((
                command.to_string(),
                format!("{}\n\nOptimize webpack configuration for this Rust WASM project:", base_context)
            )),
            "test-coverage-analysis" => Some((
                command.to_string(),
                format!("{}\n\nAnalyze test coverage for this JavaScript/TypeScript project and suggest improvements:", base_context)
            )),
            "react-component-generator" => Some((
                command.to_string(),
                format!("{}\n\nGenerate a React component with TypeScript support for this functionality:", base_context)
            )),
            "vue-component-generator" => Some((
                command.to_string(),
                format!("{}\n\nGenerate a Vue.js component with TypeScript support for this functionality:", base_context)
            )),
            "express-route-generator" => Some((
                command.to_string(),
                format!("{}\n\nCreate Express.js route handlers with proper error handling and validation:", base_context)
            )),
            "database-model-generator" => Some((
                command.to_string(),
                format!("{}\n\nGenerate database models and schemas for this Node.js application:", base_context)
            )),
            "documentation-generator" => Some((
                command.to_string(),
                format!("{}\n\nGenerate comprehensive documentation for this project including README, API docs, and code comments:", base_context)
            )),
            "docker-optimization" => Some((
                command.to_string(),
                format!("{}\n\nOptimize the Dockerfile and Docker configuration for this project:", base_context)
            )),
            "ci-cd-enhancement" => Some((
                command.to_string(),
                format!("{}\n\nImprove CI/CD pipeline configuration for this project:", base_context)
            )),
            _ => None,
        }
    }

    fn build_context_string(context: &ProjectContext) -> String {
        let info = &context.info;
        let structure = &context.structure;
        
        let mut ctx = format!("Project: {} ({:?})", info.name, info.project_type);
        
        if !info.features.is_empty() {
            ctx.push_str(&format!("\nFeatures: {}", info.features.join(", ")));
        }
        
        ctx.push_str(&format!("\nFiles: {} source, {} tests, {} configs", 
            structure.source_files.len(), 
            structure.test_files.len(), 
            structure.config_files.len()));
        
        if !structure.dependencies.is_empty() {
            let key_deps: Vec<_> = structure.dependencies.keys()
                .filter(|k| Self::is_important_dependency(k, &info.project_type))
                .take(5)
                .collect();
            if !key_deps.is_empty() {
                let deps_str: Vec<String> = key_deps.iter().map(|s| s.to_string()).collect();
                ctx.push_str(&format!("\nKey dependencies: {}", deps_str.join(", ")));
            }
        }
        
        if !structure.scripts.is_empty() {
            let scripts: Vec<_> = structure.scripts.keys().take(3).collect();
            let scripts_str: Vec<String> = scripts.iter().map(|s| s.to_string()).collect();
            ctx.push_str(&format!("\nAvailable scripts: {}", scripts_str.join(", ")));
        }
        
        ctx
    }

    fn is_important_dependency(dep_name: &str, project_type: &ProjectType) -> bool {
        match project_type {
            ProjectType::RustNormal | ProjectType::RustWasm => {
                matches!(dep_name, "tokio" | "async-std" | "serde" | "clap" | "wasm-bindgen" | "web-sys" | "js-sys")
            },
            ProjectType::JavaScript | ProjectType::TypeScript | ProjectType::NodeJs => {
                matches!(dep_name, "react" | "vue" | "express" | "fastify" | "mongoose" | "prisma" | "jest" | "typescript")
            },
        }
    }

    pub fn create_enhanced_init_templates(context: &ProjectContext) -> Vec<(String, String)> {
        let mut templates = Vec::new();
        
        // 既存の基本テンプレートを取得
        let base_templates = crate::templates::TemplateManager::get_templates_for_project(&context.info);
        
        // 基本テンプレートをプロジェクトコンテキストで拡張
        for (name, content) in base_templates {
            let enhanced_content = format!("{}\n\n{}", 
                Self::build_context_string(context), 
                content);
            templates.push((name.to_string(), enhanced_content));
        }
        
        // プロジェクト固有のテンプレートを追加
        templates.extend(Self::generate_context_specific_templates(context));
        
        templates
    }
}