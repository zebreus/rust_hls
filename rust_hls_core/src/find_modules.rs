use thiserror::Error;
mod extract_modules_from_file;
pub use extract_modules_from_file::*;
use glob::glob;
use std::path::PathBuf;

#[derive(Error, Debug)]
pub enum FindModulesError {
    #[error(transparent)]
    GlobPatternError(#[from] glob::PatternError),
    #[error(transparent)]
    GlobError(#[from] glob::GlobError),
    #[error(transparent)]
    StripPrefixError(#[from] std::path::StripPrefixError),
    #[error(transparent)]
    ExtractModuleError(#[from] ExtractModuleError),
}

fn convert_to_relative_paths(
    root: &PathBuf,
    files: Vec<PathBuf>,
) -> Result<Vec<PathBuf>, FindModulesError> {
    let root_string = root.to_string_lossy().to_string();
    let relative_files: Result<Vec<PathBuf>, _> = files
        .iter()
        .map(|p| {
            p.strip_prefix(root_string.as_str())
                .and_then(|p| Ok(p.to_path_buf()))
        })
        .collect();

    Ok(relative_files?)
}

/// Find all rust files in the crate
///
/// # Arguments
///
/// * `root` - The root directory of the crate
///
/// For now this just looks for all files ending in .rs that are in src/
pub fn find_files(root: &PathBuf) -> Result<Vec<PathBuf>, FindModulesError> {
    let root_string = root.to_string_lossy().to_string();
    let valid_files: Result<Vec<PathBuf>, _> =
        glob(format!("{}/src/**/*.rs", root_string).as_str())?.collect();
    convert_to_relative_paths(root, valid_files?)
}

pub fn find_modules(root: &PathBuf) -> Result<Vec<MacroModule>, FindModulesError> {
    let files = find_files(root)?;
    let modules = files
        .iter()
        .map(|f| extract_modules_from_file(root, f))
        .collect::<Result<Vec<Vec<MacroModule>>, ExtractModuleError>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<MacroModule>>();

    return Ok(modules);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_FILES_ROOT: &str = "test_suites/test_files";

    #[test]
    fn finds_a_reasonable_amount_of_modules_in_the_test_files_crate() {
        // The test file should contain a lot of modules. At least three
        let root = PathBuf::from(TEST_FILES_ROOT);

        let modules = find_modules(&root).unwrap();

        assert!(modules.len() >= 3);
    }
}
