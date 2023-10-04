use std::path::Path;
use ligen::common::*;

pub fn parse_file_recursive(path: &Path) -> Result<syn::File> {
    let mut file = load_file(path)?;
    load_modules(path, &mut file.items)?;
    Ok(file)
}

fn load_file(path: &Path) -> Result<syn::File> {
    let content = std::fs::read_to_string(path)?;
    syn::parse_file(&content)
        .map_err(|e| Error::Message(format!("Failed to parse file {:?}.", e)))
}

fn load_modules(path: &Path, items: &mut [syn::Item]) -> Result<()> {
    let path = path_handling::find_base_path(path)?;
    for item in items {
        if let syn::Item::Mod(module) = item {
            load_module(&path, module)?;
        }
    }
    Ok(())
}

fn load_module(path: &Path, module: &mut syn::ItemMod) -> Result<()> {
    if module.content.is_none() {
        let module_name = module.ident.to_string();
        let module_path = path_handling::find_module_path(path, &module_name)?;
        let mut file = load_file(&module_path)?;
        load_modules(&module_path, &mut file.items)?;
        module.attrs.extend(file.attrs);
        module.content = Some((syn::token::Brace::default(), file.items));
    }
    Ok(())
}

mod path_handling {
    use super::*;
    pub fn find_module_path(path: &Path, module_name: &str) -> Result<std::path::PathBuf> {
        let file_rs = path.join(&module_name).with_extension("rs");
        let mod_rs = path.join(&module_name).join("mod.rs");
        match (file_rs.exists(), mod_rs.exists()) {
            (true, true) => Err(Error::Message(format!("Ambiguous module {:?}.", module_name))),
            (true, false) => Ok(file_rs),
            (false, true) => Ok(mod_rs),
            (false, false) => Err(Error::Message(format!("Failed to find module {:?}.", module_name))),
        }
    }

    pub fn find_base_path(path: &Path) -> Result<std::path::PathBuf> {
        if path.ends_with("mod.rs") || path.ends_with("lib.rs") {
            Ok(path
                .parent()
                .ok_or("Failed to get parent directory.")?
                .to_path_buf())
        } else {
            Ok(path.with_extension(""))
        }
    }
}
