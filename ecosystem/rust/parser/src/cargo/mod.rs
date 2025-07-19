use anyhow::Context;
use ligen::prelude::*;
use ligen::idl::{Author, Dependency, Identifier, VersionRequirement, Version};
use cargo_toml::{Inheritable, Manifest, Workspace};

pub struct Cargo {
    pub folder: std::path::PathBuf,
    pub workspace: Option<Workspace>,
    pub manifest: Manifest
}

impl Cargo {
    pub fn new(path: impl AsRef<std::path::Path>) -> Result<Self> {
        let path = path.as_ref();
        let (folder, manifest) = if path.is_dir() {
            (path.to_path_buf(), path.join("Cargo.toml"))
        } else {
            (path.parent().unwrap().to_path_buf(), path.to_path_buf())
        };
        let workspace = Self::get_project_root_from_path(&folder)?;
        let workspace = cargo_toml::Manifest::from_path(workspace.join("Cargo.toml"))
            .map_err(|e| anyhow::anyhow!("Failed to read Cargo.toml: {}", e))?;
        let workspace = workspace.workspace;
        let manifest = cargo_toml::Manifest::from_path(manifest)
            .map_err(|e| anyhow::anyhow!("Failed to read Cargo.toml: {}", e))?;
        Ok(Self { folder, workspace, manifest })
    }

    pub fn get_name(&self) -> Result<&String> {
        let package = self.manifest.package.as_ref().context("Package not found in Cargo.toml.")?;
        Ok(&package.name)
    }

    fn get_from_package_or_workspace<T>(package: Inheritable<T>, workspace: Option<T>) -> Result<T> {
        match package {
            Inheritable::Inherited { .. } => Ok(workspace.context("Workspace not found in Cargo.toml.")?),
            Inheritable::Set(authors) => Ok(authors),
        }
    }

    pub fn get_authors(&self) -> Result<Vec<Author>> {
        let package = self.manifest.package.as_ref().context("Package not found in Cargo.toml.")?.authors.as_ref();
        let workspace_package = self.workspace.as_ref().and_then(|workspace| workspace.package.as_ref()).and_then(|package| package.authors.as_ref());
        let authors = Self::get_from_package_or_workspace(package, workspace_package)?;
        Ok(authors.iter().map(Author::from).collect())
    }

    pub fn get_keywords(&self) -> Result<Vec<String>> {
        let package = self.manifest.package.as_ref().context("Package not found in Cargo.toml.")?.keywords.as_ref();
        let workspace_package = self.workspace.as_ref().and_then(|workspace| workspace.package.as_ref()).and_then(|package| package.keywords.as_ref());
        let keywords = Self::get_from_package_or_workspace(package, workspace_package)?;
        Ok(keywords.to_vec())
    }

    pub fn get_license(&self) -> Result<Option<String>> {
        let manifest = self.manifest.package.as_ref().context("Package not found in Cargo.toml.")?;
        if let Some(license) = &manifest.license {
            let package = license.as_ref();
            let workspace_package = self.workspace.as_ref().and_then(|workspace| workspace.package.as_ref()).and_then(|package| package.license.as_ref());
            let license = Self::get_from_package_or_workspace(package, workspace_package)?;
            Ok(Some(license.clone()))
        }
        else {
            Ok(None)
        }
    }

    pub fn get_version(&self) -> Result<Version> {
        let package = self.manifest.package.as_ref().context("Package not found in Cargo.toml.")?.version.as_ref();
        let workspace_package = self.workspace.as_ref().and_then(|workspace| workspace.package.as_ref()).and_then(|package| package.version.as_ref());
        let version = Self::get_from_package_or_workspace(package, workspace_package)?;
        Version::try_from(version.clone())
    }

    pub fn get_description(&self) -> Result<Option<String>> {
        let manifest = self.manifest.package.as_ref().context("Package not found in Cargo.toml.")?;
        if let Some(description) = manifest.description.as_ref() {
            let package = description.as_ref();
            let workspace_package = self.workspace.as_ref().and_then(|workspace| workspace.package.as_ref()).and_then(|package| package.description.as_ref());
            let description = Self::get_from_package_or_workspace(package, workspace_package)?;
            Ok(Some(description.clone()))
        }
        else {
            Ok(None)
        }
    }
    
    pub fn get_homepage(&self) -> Result<Option<String>> {
        let manifest = self.manifest.package.as_ref().context("Package not found in Cargo.toml.")?;
        if let Some(homepage) = manifest.homepage.as_ref() {
            let package = homepage.as_ref();
            let workspace_package = self.workspace.as_ref().and_then(|workspace| workspace.package.as_ref()).and_then(|package| package.homepage.as_ref());
            let homepage = Self::get_from_package_or_workspace(package, workspace_package)?;
            Ok(Some(homepage.clone()))
        }
        else {
            Ok(None)
        }
    }

    pub fn get_rust_version(&self) -> Result<Option<VersionRequirement>> {
        let manifest = self.manifest.package.as_ref().context("Package not found in Cargo.toml.")?;
        if let Some(rust_version) = manifest.rust_version.as_ref() {
            let package = rust_version.as_ref();
            let workspace_package = self.workspace.as_ref().and_then(|workspace| workspace.package.as_ref()).and_then(|package| package.rust_version.as_ref());
            let version = Self::get_from_package_or_workspace(package, workspace_package)?;
            Ok(Some(VersionRequirement::from(version.clone())))
        }
        else {
            Ok(None)
        }
    }

    pub fn get_dependencies(&self) -> Result<Vec<Dependency>> {
        let mut dependencies = vec![];
        for (name, requirements) in self.manifest.dependencies.iter() {
            let identifier = Identifier::from(name.clone());
            let (requirement, features) = match requirements {
                cargo_toml::Dependency::Simple(requirement) => {
                    let requirement = VersionRequirement::from(requirement.clone());
                    (requirement, vec![])
                },
                cargo_toml::Dependency::Detailed(dependency) => {
                    let requirement = dependency.version.as_ref().context("Version not found in Cargo.toml.")?;
                    let requirement = VersionRequirement::from(requirement.clone());
                    (requirement, dependency.features.clone())
                },
                cargo_toml::Dependency::Inherited(_dependency) => {
                    todo!("Inherited dependencies are not supported yet.")
                }
            };
            let features = features.into_iter().map(Identifier::from).collect();
            let dependency = Dependency { identifier, requirement, features };
            dependencies.push(dependency);
        }
        Ok(dependencies)
    }

    pub fn get_project_root_from_path(path: impl AsRef<std::path::Path>) -> Result<std::path::PathBuf> {
        let path = path.as_ref();

        let mut cargo_toml = None;

        for p in path.ancestors() {
            let has_cargo =
                std::fs::read_dir(p)?
                    .any(|p|
                        p.map(|p|
                            p.file_name() == *"Cargo.toml"
                        ).unwrap_or(false)
                    );
            if has_cargo {
                cargo_toml = Some(p);
            }
        }
        if let Some(path) = cargo_toml {
            return Ok(path.to_path_buf());
        }
        Err(anyhow::anyhow!("Failed to find workspace root").into())
    }
}