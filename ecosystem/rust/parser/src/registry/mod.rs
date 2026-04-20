use crate::library::RustLibraryParser;
use ligen::idl::Registry;
use ligen::prelude::*;
use std::process::Stdio;

#[derive(Default)]
pub struct RustRegistryParser;

impl RustRegistryParser {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Transformer<std::path::PathBuf, Registry> for RustRegistryParser {
    fn transform(&self, input: std::path::PathBuf, config: &Config) -> Result<Registry> {
        <Self as Transformer<&std::path::Path, Registry>>::transform(self, input.as_path(), config)
    }

    fn name(&self) -> &str {
        "Rust"
    }

    fn config(&self) -> Config {
        Default::default()
    }
}

impl Transformer<&std::path::Path, Registry> for RustRegistryParser {
    fn transform(&self, input: &std::path::Path, config: &Config) -> Result<Registry> {
        tokio::runtime::Runtime::new()
            .map_err(|e| Error::Message(format!("Failed to create tokio runtime: {}", e)))?
            .block_on(self.transform_async(input, config))
    }

    fn name(&self) -> &str {
        "Rust"
    }

    fn config(&self) -> Config {
        Default::default()
    }
}

#[async_trait]
impl AsyncTransformer<std::path::PathBuf, Registry> for RustRegistryParser {
    async fn transform(&self, input: std::path::PathBuf, config: &Config) -> Result<Registry> {
        self.transform_async(input.as_path(), config).await
    }

    fn name(&self) -> &str {
        "Rust"
    }

    fn config(&self) -> Config {
        Default::default()
    }
}

impl RustRegistryParser {
    async fn transform_async(&self, input: &std::path::Path, config: &Config) -> Result<Registry> {
        let mut registry = Registry::new();
        let manifest_path = if input.is_dir() {
            input.join("Cargo.toml")
        } else {
            input.to_path_buf()
        };

        // 1. Get metadata offline first to see if we need to fetch.
        let output = tokio::process::Command::new("cargo")
            .arg("metadata")
            .arg("--format-version")
            .arg("1")
            .arg("--manifest-path")
            .arg(&manifest_path)
            .arg("--offline")
            .output()
            .await;

        let output = match output {
            Ok(output) if output.status.success() => output,
            _ => {
                // 2. If offline metadata fails, fetch dependencies.
                let mut child = tokio::process::Command::new("cargo")
                    .arg("fetch")
                    .arg("--manifest-path")
                    .arg(&manifest_path)
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn()
                    .map_err(|e| Error::Message(format!("Failed to run cargo fetch: {}", e)))?;

                let status = child.wait().await.map_err(|e| {
                    Error::Message(format!("Failed to wait for cargo fetch: {}", e))
                })?;

                if !status.success() {
                    return Err(Error::Message(format!(
                        "cargo fetch failed with status: {}",
                        status
                    )));
                }

                // 3. Get metadata online after fetch.
                tokio::process::Command::new("cargo")
                    .arg("metadata")
                    .arg("--format-version")
                    .arg("1")
                    .arg("--manifest-path")
                    .arg(&manifest_path)
                    .output()
                    .await
                    .map_err(|e| Error::Message(format!("Failed to run cargo metadata: {}", e)))?
            }
        };

        if !output.status.success() {
            return Err(Error::Message(format!(
                "cargo metadata failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        let metadata: serde_json::Value = serde_json::from_slice(&output.stdout)
            .map_err(|e| Error::Message(format!("Failed to parse cargo metadata JSON: {}", e)))?;

        let workspace_members: Vec<String> = metadata["workspace_members"]
            .as_array()
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();

        let mut package_ids_to_parse = std::collections::HashSet::new();
        for member_id in &workspace_members {
            package_ids_to_parse.insert(member_id.clone());
        }

        if let Some(nodes) = metadata["resolve"]["nodes"].as_array() {
            for node in nodes {
                if let Some(id) = node["id"].as_str() {
                    if workspace_members.contains(&id.to_string()) {
                        if let Some(deps) = node["dependencies"].as_array() {
                            for dep in deps {
                                if let Some(dep_id) = dep.as_str() {
                                    package_ids_to_parse.insert(dep_id.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }

        let library_parser = RustLibraryParser::new();

        if let Some(packages) = metadata["packages"].as_array() {
            for package in packages {
                if let Some(id) = package["id"].as_str() {
                    if package_ids_to_parse.contains(id) {
                        if let Some(manifest_path_str) = package["manifest_path"].as_str() {
                            let package_manifest_path = std::path::Path::new(manifest_path_str);
                            let package_dir = package_manifest_path.parent().unwrap();
                            if let Ok(library) = library_parser.transform(package_dir, config) {
                                registry
                                    .libraries
                                    .insert(library.identifier.clone(), library);
                            }
                        }
                    }
                }
            }
        }

        Ok(registry)
    }
}
