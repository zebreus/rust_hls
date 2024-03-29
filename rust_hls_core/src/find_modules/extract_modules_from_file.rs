use crate::{
    darling_error_outside_macro::DarlingErrorOutsideMacro, extract_file_hash,
    filename_to_module_path, synthesized_module_filepath, ExtractHashError, ExtractModulePathError,
};
use itertools::Itertools;
use rust_hls_macro_lib::{extract_hls_macro, HlsArguments};
use serde::Serialize;
use std::{
    fs::{self},
    path::PathBuf,
};
use syn::Item;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExtractModuleError {
    #[error(transparent)]
    HlsMacroError(#[from] DarlingErrorOutsideMacro),
    #[error(transparent)]
    ExtractHashError(#[from] ExtractHashError),
    #[error(transparent)]
    ExtractModulePathError(#[from] ExtractModulePathError),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("{error}")]
    FailedToParseFile { error: DarlingErrorOutsideMacro },
    #[error("You can not use the rust_hls macro on a module import ({file} {module})")]
    RustHlsMacroOnModuleImport { file: String, module: String },
    #[error(transparent)]
    CargoTomlError(#[from] cargo_toml::Error),
    #[error(transparent)]
    FailedToSerializeCargoToml(#[from] toml::ser::Error),
}

/// Parse a file at a path into a syn AST
fn parse_file(source_file: &PathBuf) -> Result<syn::File, ExtractModuleError> {
    let content = fs::read_to_string(&source_file)?;
    let parsed_file = syn::parse_file(content.as_str()).or_else(|err| {
        Err(ExtractModuleError::FailedToParseFile {
            error: DarlingErrorOutsideMacro::from_syn_error(err, &source_file),
        })
    })?;
    Ok(parsed_file)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ModuleType {
    /// Submodule in the same file
    Inline,
}

/// A module that contains a macro function.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MacroModule {
    /// Content of the macro call module
    pub item_mod: syn::ItemMod,
    /// Content of the macro call module as a stringified syn::File
    pub module_content_string: String,
    /// The arguments that should be used when converting the macro
    pub hls_arguments: HlsArguments,
    /// Path to the original file that contains the macro call
    pub source_file: PathBuf,
    /// Path to the original crate root
    pub crate_root: PathBuf,
    /// Path to the module in the original file. Like "something::something::module_name"
    pub module_path: Vec<String>,
    /// Absolute path to the module in the original crate.
    ///
    /// Can be used to address the module with crate::
    pub absolute_module_path: Vec<String>,
    /// Original type of the module
    pub module_type: ModuleType,
    /// Cargo.toml of the crate.
    pub cargo_toml: String,
    /// The hash that is currently present in the output file
    ///
    /// None if the output file does not exist
    pub previous_hash: Option<String>,
    /// The path to the output file
    pub output_file: PathBuf,
}

impl MacroModule {
    /// Helper for creating MacroModules in tests
    pub fn new_for_tests(
        content: proc_macro2::TokenStream,
        filename: &str,
    ) -> (Self, tempfile::TempDir) {
        let file: syn::File = syn::parse2(content).unwrap();
        let content = prettyplease::unparse(&file);
        Self::new_for_tests_string(&content, filename)
    }
    /// Helper for creating MacroModules in tests
    pub fn new_for_tests_string(content: &str, filename: &str) -> (Self, tempfile::TempDir) {
        let dir = tempfile::TempDir::new().unwrap();
        std::fs::create_dir_all(dir.path().join("src")).unwrap();
        fs_extra::file::write_all(dir.path().join(filename), content).unwrap();
        fs_extra::file::write_all(
            dir.path().join("Cargo.toml"),
            r#"[package]
        name = "test-crate"
        version = "1.0.0"
        edition = "2021""#,
        )
        .unwrap();

        let modules = super::find_modules(&dir.path().into()).unwrap();

        let first_module = modules.into_iter().nth(0).unwrap();

        (first_module, dir)
    }
}

// Rust-hls rules
// No external mods
// No references to the same crate
// No using mod to add external modules
/// Extracts a module from an item, if it is a Item::Mod. Otherwise returns an empty vector.
///
fn extract_module_from_item(
    item: syn::Item,
    crate_root: &PathBuf,
    file: &PathBuf,
    current_module_path: &Vec<String>,
) -> Result<Vec<MacroModule>, ExtractModuleError> {
    match item {
        Item::Macro(_)
        | Item::Const(_)
        | Item::Fn(_)
        | Item::Static(_)
        | Item::Struct(_)
        | Item::Trait(_)
        | Item::TraitAlias(_)
        | Item::Type(_)
        | Item::Union(_)
        | Item::Use(_)
        | Item::Verbatim(_)
        | Item::ExternCrate(_)
        | Item::ForeignMod(_)
        | Item::Impl(_)
        | Item::Enum(_) => Ok(Vec::new()),
        Item::Mod(module) => {
            let module_name = module.ident.to_string();

            let current_module_path = current_module_path
                .iter()
                .cloned()
                .chain(std::iter::once(module_name.clone()))
                .collect();

            let rust_hls_options = extract_hls_macro(&module.attrs).or_else(|err| {
                let error = DarlingErrorOutsideMacro::new(&err, &crate_root.join(file));
                return Err(error);
            })?;

            let Some(rust_hls_arguments) = rust_hls_options else {
                // Could not find a rust-hls macro
                let thing: Result<Vec<MacroModule>, ExtractModuleError> = match module.content {
                    Some((_, items)) => {
                        extract_hls_modules(items, crate_root, file, &current_module_path)
                    }
                    None => Ok(Vec::new()),
                };
                return thing;
            };

            let Some(module_content) = &module.content else {
                return Err(ExtractModuleError::RustHlsMacroOnModuleImport {
                    module: current_module_path.iter().join("::"),
                    file: file.to_string_lossy().to_string(),
                });
            };

            let module_file = syn::File {
                shebang: None,
                attrs: Vec::new(),
                items: module_content.1.clone(),
            };

            let input_module_path = filename_to_module_path(&file, &current_module_path)?;
            let output_file = synthesized_module_filepath(&input_module_path);
            let output_file_content = std::fs::read_to_string(crate_root.join(&output_file));

            let previous_hash = match output_file_content {
                Ok(content) => Some(extract_file_hash(&content)?),
                Err(_) => None,
            };

            let cargo_toml_str = std::fs::read_to_string(crate_root.join("Cargo.toml")).unwrap_or(
                r#"[package]
name = "temporary"
version = "0.0.1"
edition = "2021""#
                    .into(),
            );

            let mut manifest = cargo_toml::Manifest::from_str(&cargo_toml_str)?;
            manifest.complete_from_path(&crate_root.join("Cargo.toml"))?;
            let mut buffer = String::new();
            let serializer = toml::Serializer::new(&mut buffer);
            manifest.serialize(serializer)?;
            let cargo_toml_str = buffer;

            let result: Result<Vec<MacroModule>, ExtractModuleError> = Ok(vec![MacroModule {
                module_content_string: prettyplease::unparse(&module_file),
                item_mod: module,
                source_file: file.clone(),
                crate_root: crate_root.clone(),
                module_path: current_module_path.clone(),
                absolute_module_path: input_module_path,
                hls_arguments: rust_hls_arguments,
                module_type: ModuleType::Inline,
                cargo_toml: cargo_toml_str,
                previous_hash,
                output_file,
            }]);

            result
        }
        _ => Ok(Vec::new()) as Result<Vec<MacroModule>, ExtractModuleError>,
    }
}

/// Finds all modules that look like they are rust-hls modules in this file.
pub fn extract_hls_modules(
    items: Vec<syn::Item>,
    crate_root: &PathBuf,
    file: &PathBuf,
    current_module_path: &Vec<String>,
) -> Result<Vec<MacroModule>, ExtractModuleError> {
    items
        .into_iter()
        .map(|item| {
            let modules = extract_module_from_item(item, crate_root, file, current_module_path);
            modules
        })
        .try_fold(Vec::new(), |mut acc, f| {
            let Ok(macro_modules) = f else { return f };
            acc.extend(macro_modules.into_iter());
            Ok(acc)
        })
}

pub fn extract_modules_from_file(
    crate_root: &PathBuf,
    file: &PathBuf,
) -> Result<Vec<MacroModule>, ExtractModuleError> {
    let source_file = crate_root.join(file);
    let parsed_file = parse_file(&source_file)?;
    let items = parsed_file.items;

    extract_hls_modules(items, crate_root, file, &vec![])
}

#[cfg(test)]
mod tests {
    use super::*;
    use glob::glob;
    use quote::quote;

    const TEST_FILES_ROOT: &str = "test_suites/test_files";

    #[test]
    fn all_test_files_are_valid() {
        let valid_files: Result<Vec<PathBuf>, _> =
            glob(format!("{TEST_FILES_ROOT}/src/valid/*.rs").as_str())
                .expect("Failed to read glob pattern")
                .collect();

        for file in valid_files.expect("Failed when checking globbed files") {
            extract_modules_from_file(
                &PathBuf::from(TEST_FILES_ROOT),
                &PathBuf::from(
                    file.strip_prefix(TEST_FILES_ROOT)
                        .expect("Failed to root prefix"),
                ),
            )
            .expect(format!("Failed to extract modules from file {:?}", file).as_str());
        }
    }

    #[test]
    fn finds_one_module_if_there_is_only_one() {
        let root = PathBuf::from(TEST_FILES_ROOT);
        let file_with_one_hls_module = PathBuf::from("src/valid/file_with_one_hls_module.rs");

        let result = extract_modules_from_file(&root, &file_with_one_hls_module).unwrap();

        assert_eq!(result.len(), 1);
        let module = result.first().unwrap();
        assert_eq!(module.absolute_module_path.last().unwrap(), "hls_module");
        assert_eq!(module.module_path, vec!("hls_module"));
        assert_eq!(module.source_file, file_with_one_hls_module);
    }

    #[test]
    fn detects_inline_modules() {
        let root = PathBuf::from(TEST_FILES_ROOT);
        let file_with_one_hls_module = PathBuf::from("src/valid/file_with_one_hls_module.rs");

        let result = extract_modules_from_file(&root, &file_with_one_hls_module).unwrap();

        assert_eq!(result.len(), 1);
        let module = result.first().unwrap();
        assert_eq!(module.module_type, ModuleType::Inline);
    }

    #[test]
    fn finds_two_modules_in_a_file_with_two_modules_also_they_are_ordered_correctly() {
        let root = PathBuf::from(TEST_FILES_ROOT);
        let file_with_two_hls_modules = PathBuf::from("src/valid/file_with_two_hls_modules.rs");

        let result = extract_modules_from_file(&root, &file_with_two_hls_modules).unwrap();

        assert_eq!(result.len(), 2);
        let module = result.get(0).unwrap();
        assert_eq!(module.absolute_module_path.last().unwrap(), "hls_module_a");
        assert_eq!(module.module_path, vec!("hls_module_a"));
        assert_eq!(&module.source_file, &file_with_two_hls_modules);

        let module = result.get(1).unwrap();
        assert_eq!(module.absolute_module_path.last().unwrap(), "hls_module_b");
        assert_eq!(module.module_path, vec!("hls_module_b"));
        assert_eq!(&module.source_file, &file_with_two_hls_modules);
    }

    #[test]
    fn finds_no_modules_if_there_are_none() {
        let root = PathBuf::from(TEST_FILES_ROOT);
        let module_without_macro = PathBuf::from("src/valid/module_without_macro.rs");

        let result = extract_modules_from_file(&root, &module_without_macro).unwrap();

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn finds_modules_even_if_they_dont_have_a_hls_function() {
        let root = PathBuf::from(TEST_FILES_ROOT);

        let file_with_hls_module_without_hls_function =
            PathBuf::from("src/valid/file_with_hls_module_without_hls_function.rs");
        let result =
            extract_modules_from_file(&root, &file_with_hls_module_without_hls_function).unwrap();
        assert_eq!(result.len(), 1);

        let file_with_hls_module_without_function =
            PathBuf::from("src/valid/file_with_hls_module_without_function.rs");
        let result =
            extract_modules_from_file(&root, &file_with_hls_module_without_function).unwrap();
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn new_for_tests_seems_to_work() {
        let module = MacroModule::new_for_tests(
            quote!(
                #[hls]
                mod toast {
                    #[hls]
                    #[no_mangle]
                    pub extern "C" fn add(a: u32, b: u32) {
                        a + b
                    }
                }
            ),
            "src/lib.rs",
        );

        assert_eq!(module.0.absolute_module_path.last().unwrap(), "toast");
    }
}
