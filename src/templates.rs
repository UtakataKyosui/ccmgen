use crate::project::{ProjectType, ProjectInfo};
use std::collections::HashMap;

pub struct TemplateManager;

impl TemplateManager {
    pub fn get_templates_for_project(project: &ProjectInfo) -> Vec<(&'static str, &'static str)> {
        match project.project_type {
            ProjectType::RustNormal => Self::rust_templates(),
            ProjectType::RustWasm => Self::rust_wasm_templates(),
            ProjectType::JavaScript => Self::javascript_templates(),
            ProjectType::TypeScript => Self::typescript_templates(),
            ProjectType::NodeJs => Self::nodejs_templates(),
        }
    }

    fn rust_templates() -> Vec<(&'static str, &'static str)> {
        vec![
            ("review-performance", 
             "Analyze the performance characteristics of this Rust code and suggest improvements to make it faster or more efficient:"),
            ("generate-tests", 
             "Generate unit tests for the following Rust function using the built-in test framework:"),
            ("add-documentation", 
             "Add comprehensive Rust documentation comments (///) to the following code:"),
            ("optimize-memory", 
             "Review this Rust code for memory usage optimization opportunities:"),
            ("add-error-handling", 
             "Improve error handling in this Rust code using Result<T, E> and proper error types:"),
            ("refactor-traits", 
             "Suggest trait implementations or refactoring opportunities for this Rust code:"),
            ("cargo-optimization", 
             "Analyze and suggest Cargo.toml optimizations for this Rust project:"),
            ("async-conversion", 
             "Convert this synchronous Rust code to use async/await patterns:"),
        ]
    }

    fn rust_wasm_templates() -> Vec<(&'static str, &'static str)> {
        vec![
            ("wasm-bindgen-wrapper", 
             "Create wasm-bindgen JavaScript bindings for this Rust function:"),
            ("wasm-optimize", 
             "Optimize this Rust code for WebAssembly size and performance:"),
            ("js-interop", 
             "Create JavaScript interop code for this Rust WASM module:"),
            ("wasm-memory-management", 
             "Review and optimize memory management for this Rust WASM code:"),
            ("wasm-pack-config", 
             "Generate wasm-pack configuration for this Rust WebAssembly project:"),
            ("browser-integration", 
             "Create browser integration code for this Rust WASM module:"),
            ("wasm-types", 
             "Convert these Rust types to be WASM-compatible with proper serialization:"),
            ("performance-profile", 
             "Create performance profiling setup for this Rust WASM application:"),
        ]
    }

    fn javascript_templates() -> Vec<(&'static str, &'static str)> {
        vec![
            ("add-jsdoc", 
             "Add comprehensive JSDoc comments to the following JavaScript code:"),
            ("modernize-syntax", 
             "Convert this JavaScript code to use modern ES6+ syntax and features:"),
            ("add-error-handling", 
             "Improve error handling in this JavaScript code with try-catch and proper validation:"),
            ("generate-tests", 
             "Generate unit tests for the following JavaScript function using Jest:"),
            ("optimize-performance", 
             "Analyze and optimize the performance of this JavaScript code:"),
            ("add-validation", 
             "Add input validation and type checking to this JavaScript function:"),
            ("convert-promises", 
             "Convert this callback-based JavaScript code to use Promises or async/await:"),
            ("bundle-analysis", 
             "Analyze this JavaScript code for bundle size optimization opportunities:"),
        ]
    }

    fn typescript_templates() -> Vec<(&'static str, &'static str)> {
        vec![
            ("add-types", 
             "Add comprehensive TypeScript type annotations to this JavaScript code:"),
            ("interface-design", 
             "Design TypeScript interfaces and types for this code structure:"),
            ("generic-implementation", 
             "Implement TypeScript generics to make this code more reusable:"),
            ("strict-mode-fix", 
             "Fix TypeScript strict mode errors in this code:"),
            ("type-guards", 
             "Create TypeScript type guards for runtime type checking:"),
            ("utility-types", 
             "Use TypeScript utility types to improve this code structure:"),
            ("declaration-files", 
             "Generate TypeScript declaration files (.d.ts) for this JavaScript library:"),
            ("tsconfig-optimization", 
             "Optimize tsconfig.json settings for this TypeScript project:"),
        ]
    }

    fn nodejs_templates() -> Vec<(&'static str, &'static str)> {
        vec![
            ("express-middleware", 
             "Create Express.js middleware for this functionality:"),
            ("api-endpoint", 
             "Design and implement a RESTful API endpoint for this Node.js application:"),
            ("database-integration", 
             "Add database integration code for this Node.js function:"),
            ("environment-config", 
             "Create environment-based configuration management for this Node.js app:"),
            ("logging-setup", 
             "Implement comprehensive logging for this Node.js application:"),
            ("authentication", 
             "Add authentication and authorization to this Node.js API:"),
            ("docker-setup", 
             "Create Docker configuration for this Node.js application:"),
            ("performance-monitoring", 
             "Add performance monitoring and health checks to this Node.js service:"),
            ("package-optimization", 
             "Optimize package.json and dependencies for this Node.js project:"),
        ]
    }

    pub fn get_custom_templates() -> HashMap<String, String> {
        // Future: Load from configuration file
        HashMap::new()
    }

    pub fn create_project_specific_template(project: &ProjectInfo, _template_name: &str, content: &str) -> String {
        let context = format!("Project: {} ({})", project.name, format!("{:?}", project.project_type));
        let features = if !project.features.is_empty() {
            format!("Features: {}", project.features.join(", "))
        } else {
            String::new()
        };
        
        format!("{}\n{}\n\n{}", context, features, content)
    }
}