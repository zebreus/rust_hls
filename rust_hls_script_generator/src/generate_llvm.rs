use rust_hls_macro_lib::HlsArguments;
use std::{
    io::{self},
    ops::Add,
    path::PathBuf,
};
use thiserror::Error;

pub struct GenerateLlvmOptions {
    pub function_name: String,
    pub hls_arguments: HlsArguments,
    pub prepared_crate_root: PathBuf,
}

/// These flags will be used when compiling the extracted crate to LLVM IR.
pub const DEFAULT_RUST_FLAGS: &str = r#"-C overflow-checks=off -C no-vectorize-loops -C target-cpu=generic -C panic=abort -C llvm-args=--opaque-pointers=false"#;
// LTO seems broken without opaque pointers  -C linker-plugin-lto=on -C embed-bitcode=on -C lto=fat. Provides no benefit anyway as we are linking manually.

#[derive(Error, Debug)]
pub enum GenerateLlvmError {
    #[error("You need to run `cargo hls` to update the synthesized module for \"{0}\" because its source has changed.")]
    ModuleChanged(String),
    #[error("Module {0} has not been processed yet. You need to run `cargo hls` first.")]
    ModuleNotProcessed(String),
    #[error(transparent)]
    IoError(#[from] io::Error),
}

/// Return a shell script that generates LLVM
///
/// The script will be named `generate_llvm.sh` and placed at the root of the extracted crate.
/// It will be executed without arguments.
/// The script should generate a `result.ll` LLVM IR text file at the crate root.
/// Any stdout logs should be written to a `stdout.log` file at the crate root.
/// Any stderr logs should be written to a `stderr.log` file at the crate root.
pub fn generate_llvm(options: &GenerateLlvmOptions) -> Result<String, GenerateLlvmError> {
    let GenerateLlvmOptions {
        prepared_crate_root,
        function_name,
        hls_arguments,
    } = options;

    let prepared_crate_root = prepared_crate_root.to_string_lossy();

    let rust_flags = match hls_arguments.skip_default_rust_flags.unwrap_or(false) {
        true => String::new(),
        false => DEFAULT_RUST_FLAGS.into(),
    }
    .add(&hls_arguments.rust_flags());

    let bambu_docker = format!(
        r#"docker run --rm -i -v "{prepared_crate_root}:{prepared_crate_root}" --workdir={prepared_crate_root} --user $(id -u):$(id -g) zebreus/rust_hls_tools:latest"#
    );

    let llvm_link_command = format!(r#"{bambu_docker} llvm-link"#);
    let llvm_extract_command = format!(r#"{bambu_docker} llvm-extract"#);
    let llvm_opt_command = format!(r#"{bambu_docker} opt"#);
    let llvm_dis_command = format!(r#"{bambu_docker} llvm-dis"#);
    let jq_command = format!(r#"{bambu_docker} jq"#);

    let llvm_opt_flags = hls_arguments.llvm_opt_flags();
    let llvm_extract_command = format!(
        "{llvm_extract_command} --opaque-pointers=false --recursive --keep-const-init --func={function_name}"
    );

    let llvm_opt_command_with_pipe = match llvm_opt_flags {
        Some(flags) => {
            format!(
                " {llvm_opt_command} --opaque-pointers=false {flags} | {llvm_extract_command} | "
            )
        }
        None => String::new(),
    };

    let create_llvm_command = format!(
        r#"
CRATE_NAME=$(grep -oP '(?<=name = ")[^"]*' Cargo.toml)
CRATE_NAME_UNDERSCORED=$(echo $CRATE_NAME | tr '-' '_')
echo "Executing synthesis in $(pwd)" 1>&2
export RUSTFLAGS='--emit=llvm-bc {rust_flags}'
LLVM_BITCODE_FILES=($(cargo build --release -Z unstable-options --build-plan | {jq_command} '.invocations[].outputs[]' -r | grep -Po "^.*\.rlib$" | sed -E 's/lib([^\/]*)\.rlib/\1\.bc /' | tr -d '\n'))
cargo build --release -Z unstable-options
{llvm_link_command} --opaque-pointers=false "${{LLVM_BITCODE_FILES[@]}}"  | {llvm_extract_command} | {llvm_opt_command_with_pipe} {llvm_dis_command} --opaque-pointers=false -o result.ll
cp result.ll {function_name}.ll
# cp $WORKSPACE_LOCATION/target/release/deps/${{CRATE_NAME_UNDERSCORED}}-*.ll {function_name}.ll
"#
    );

    return Ok(String::from(format!(
        r#"
#!/usr/bin/env bash

function main {{
set -xe
    
# Compile to LLVM IR
{create_llvm_command}

}}

# Execute and log the output into files
main > >(tee -a stdout.log) 2> >(tee -a stderr.log 1>&2)
"#
    )));
}

#[cfg(test)]
#[derive(Error, Debug)]
pub enum ExecuteLlvmScriptError {
    #[error("Failed to generate LLVM in \"{0}\".")]
    FailedToExecute(String),
    #[error("Failed to generate LLVM in \"{0}\" with exitcode {1}.")]
    NonzeroExitCode(String, i32),
    #[error("Failed to load result file after generating LLVM IR in {0}.")]
    FailedToLoadResultFile(String),
}

#[cfg(test)]
/// Execute the generate_llvm script.
///
/// Returns false, if it failed
fn execute_llvm_script_file(
    crate_root: &std::path::Path,
) -> Result<String, ExecuteLlvmScriptError> {
    use std::{fs::File, io::Read, process::Command};
    let output = Command::new("sh")
        .arg("generate_llvm.sh")
        .current_dir(&crate_root)
        .output();

    let Ok(output) = output else {
        return Err(ExecuteLlvmScriptError::FailedToExecute(
            crate_root.to_string_lossy().into(),
        ));
    };

    let exit_code = output.status.code().unwrap();
    if exit_code != 0 {
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        return Err(ExecuteLlvmScriptError::NonzeroExitCode(
            crate_root.to_string_lossy().into(),
            exit_code,
        ));
    }
    assert_eq!(exit_code, 0);

    let output_file_path = crate_root.join("result.ll");
    let file = File::open(output_file_path);
    let Ok(mut file) = file else {
        return Err(ExecuteLlvmScriptError::FailedToLoadResultFile(
            crate_root.to_string_lossy().into(),
        ));
    };
    let mut content = String::new();
    let read_result = file.read_to_string(&mut content);
    if let Err(_) = read_result {
        return Err(ExecuteLlvmScriptError::FailedToLoadResultFile(
            crate_root.to_string_lossy().into(),
        ));
    }
    return Ok(content);
}

#[cfg(test)]
fn generate_llvm_script_file(
    crate_path: &std::path::Path,
    options: &GenerateLlvmOptions,
) -> io::Result<()> {
    use std::{fs::File, io::Write};
    let path = crate_path.join("generate_llvm.sh");
    let mut file = File::create(path)?;
    let content = generate_llvm(options).unwrap();
    file.write_all(content.as_bytes())?;
    file.sync_all()?;
    Ok(())
}

#[cfg(test)]
pub fn generate_llvm_for_tests(options: &GenerateLlvmOptions) -> Result<(), GenerateLlvmError> {
    generate_llvm_script_file(&options.prepared_crate_root, options)?;
    execute_llvm_script_file(&options.prepared_crate_root).unwrap();
    Ok(())
}

#[cfg(test)]
mod tests {
    use fs_extra::dir::{copy, CopyOptions};
    use tempfile::TempDir;

    use super::*;

    #[test]
    fn script_generates_llvm_file() {
        let dir = TempDir::new().unwrap();
        copy("test_suites/test_crate", dir.path(), &CopyOptions::new()).unwrap();

        let crate_path = dir.path().join("test_crate");
        let function_name = "add";

        let mut arguments = HlsArguments::default();
        arguments.bambu_flag.push("-v4".into());

        generate_llvm_script_file(
            &crate_path,
            &GenerateLlvmOptions {
                prepared_crate_root: crate_path.clone(),
                function_name: function_name.into(),
                hls_arguments: arguments,
            },
        )
        .unwrap();

        let _ = match execute_llvm_script_file(&crate_path) {
            Ok(generated_llvm) => generated_llvm,
            Err(error) => {
                let _keep_tempdir = dir.into_path();
                Result::<String, _>::Err(error).unwrap();
                panic!();
            }
        };

        // eprintln!("Generated LLMV: {}", generated_llvm);

        // let output = Command::new("sh")
        //     .arg("hls.sh")
        //     .current_dir(&crate_path)
        //     .output()
        //     .expect(
        //         format!(
        //             "Failed to perform HLS in {crate_name} ({})",
        //             crate_path.to_str().unwrap()
        //         )
        //         .as_str(),
        //     );

        // let exit_code = output.status.code().unwrap();
        // if exit_code != 0 {
        //     println!(
        //         "Failed HLS. Keeping directory {}",
        //         crate_path.to_str().unwrap()
        //     );
        //     println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        //     println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        //     let _keep_tempdir = dir.into_path();
        // }
        // assert_eq!(exit_code, 0);

        // assert!(crate_path.join("result.v").exists());
    }

    // #[test]
    // fn synthesizing_test_crate_creates_stdout_and_stderr_logs() {
    //     let dir = TempDir::new().unwrap();
    //     copy("test_suites/test_crate", dir.path(), &CopyOptions::new()).unwrap();

    //     let crate_path = dir.path().join("test_crate");
    //     let function_name = "add";

    //     generate_hls_script_file(
    //         &crate_path,
    //         &GenerateHlsOptions {
    //             function_name: function_name.into(),
    //             hls_arguments: HlsArguments::default(),
    //         },
    //     )
    //     .unwrap();

    //     let output = Command::new("sh")
    //         .arg("hls.sh")
    //         .current_dir(&crate_path)
    //         .output()
    //         .unwrap();

    //     let exit_code = output.status.code().unwrap();
    //     assert_eq!(exit_code, 0);

    //     let stdout_file = crate_path.join("stdout.log");
    //     let stderr_file = crate_path.join("stderr.log");

    //     assert!(stdout_file.exists());
    //     assert!(stderr_file.exists());

    //     let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    //     let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    //     let stdout_from_file = std::fs::read_to_string(stdout_file).unwrap();
    //     let stderr_from_file = std::fs::read_to_string(stderr_file).unwrap();

    //     assert_eq!(stdout, stdout_from_file);
    //     assert_eq!(stderr, stderr_from_file);
    // }
}
