use std::{collections::HashSet, io};

use cargo_toml::Manifest;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum GetDependenciesError {
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error("The source crate does not contain a Cargo.toml file")]
    NoCargoToml,
    #[error("Failed to parse cargo toml for the generated crate")]
    FailedToParseCargoToml(#[from] cargo_toml::Error),
    #[error("Failed to generate cargo toml for the generated crate")]
    FailedToGenerateCargoToml(#[from] toml::ser::Error),
}

/// Extract all available dependencies from the cargo.toml file
pub fn get_available_dependencies(
    cargo_toml: &String,
) -> Result<HashSet<String>, GetDependenciesError> {
    let manifest = Manifest::from_str(cargo_toml.as_str())?;

    let dependencies: std::collections::BTreeMap<String, cargo_toml::Dependency> =
        manifest.dependencies;
    let dependency_names_iterator = dependencies.into_iter().map(|dependency| dependency.0);

    let dependency_names = HashSet::from_iter(dependency_names_iterator);
    return Ok(dependency_names);
}

#[cfg(test)]
mod tests {

    use std::{fs, path::Path};

    use super::*;

    #[test]
    fn crate_with_relative_path_dependency_get_fixed() {
        let original_crate_path = Path::new("test_suites/crate_with_path_dependency")
            .to_path_buf()
            .canonicalize()
            .unwrap();

        let cargo_toml = fs::read_to_string(original_crate_path.join("Cargo.toml")).unwrap();

        let available_dependencies = get_available_dependencies(&cargo_toml).unwrap();

        assert_eq!(available_dependencies.len(), 1);
        assert!(available_dependencies.contains("test_crate"));
    }
}
