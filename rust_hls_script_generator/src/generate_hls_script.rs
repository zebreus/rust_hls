use std::ops::Add;

use rust_hls_macro_lib::HlsArguments;

pub struct GenerateHlsOptions {
    pub function_name: String,
    pub hls_arguments: HlsArguments,
}

/// These flags will be used when performing HLS from the generated LLVM IR.
pub const DEFAULT_HLS_FLAGS: &str = r#"--compiler=I386_CLANG16"#;

/// Return a shell script that generates verilog based on a LLVM file
///
/// The script will be named `generate_verilog.sh` and placed at the root of the extracted crate.
/// The LLVM should be read from a file named `result.ll` at the crate root
/// It will be executed without arguments.
/// The script should generate a `result.v` verilog file at the crate root.
/// Any stdout logs should be written to a `stdout.log` file at the crate root.
/// Any stderr logs should be written to a `stderr.log` file at the crate root.
pub fn generate_hls_script(options: &GenerateHlsOptions) -> String {
    let GenerateHlsOptions {
        function_name,
        hls_arguments,
    } = options;

    let hls_flags = match hls_arguments.skip_default_bambu_flags.unwrap_or(false) {
        true => String::new(),
        false => DEFAULT_HLS_FLAGS.into(),
    }
    .add(&hls_arguments.bambu_flags());

    let bambu_docker = format!(
        r#"docker run --rm -i -v "$(pwd):$(pwd)" --workdir=$(pwd) --user $(id -u):$(id -g) zebreus/rust_hls_tools:latest"#
    );

    let bambu_command = format!(r#"{bambu_docker} bambu"#);

    return String::from(format!(
        r#"
#!/usr/bin/env bash

function main {{
set -xe

# Perform HLS
{bambu_command} --simulator=VERILATOR result.ll --top-fname={function_name} --clock-name=clk {hls_flags}
mv {function_name}.v result.v

}}

# Execute and log the output into files
main > >(tee -a stdout.log) 2> >(tee -a stderr.log 1>&2)
"#
    ));
}

#[cfg(test)]
mod tests {
    use fs_extra::dir::{copy, CopyOptions};
    use std::fs::remove_file;
    use std::process::Command;
    use std::{
        fs::File,
        io::{self, Write},
        path::Path,
    };
    use tempfile::TempDir;

    use crate::{generate_llvm_for_tests, GenerateLlvmOptions};

    use super::*;

    fn generate_hls_script_file(crate_path: &Path, options: &GenerateHlsOptions) -> io::Result<()> {
        let path = crate_path.join("hls.sh");
        let mut file = File::create(path)?;
        let content = generate_hls_script(options);
        file.write_all(content.as_bytes())?;
        file.sync_all()?;
        Ok(())
    }

    #[test]
    fn generate_hls_script_creates_a_file() {
        let dir = TempDir::new().unwrap();

        generate_hls_script_file(
            &dir.path(),
            &GenerateHlsOptions {
                function_name: "test".into(),
                hls_arguments: HlsArguments::default(),
            },
        )
        .unwrap();
        let expected_file_path = dir.path().join("hls.sh");
        assert!(expected_file_path.exists());

        dir.close().unwrap();
    }

    #[test]
    fn synthesizing_test_crate_creates_verilog_file() {
        let dir = TempDir::new().unwrap();
        copy("test_suites/test_crate", dir.path(), &CopyOptions::new()).unwrap();

        let crate_path = dir.path().join("test_crate");
        let crate_name = "test_crate";
        let function_name = "add";
        let mut arguments = HlsArguments::default();
        arguments.bambu_flag.push("-v4".into());
        generate_llvm_for_tests(&GenerateLlvmOptions {
            function_name: function_name.into(),
            hls_arguments: arguments.clone(),
            prepared_crate_root: crate_path.clone(),
        })
        .unwrap();

        generate_hls_script_file(
            &crate_path,
            &GenerateHlsOptions {
                function_name: function_name.into(),
                hls_arguments: arguments,
            },
        )
        .unwrap();

        let output = Command::new("sh")
            .arg("hls.sh")
            .current_dir(&crate_path)
            .output()
            .expect(
                format!(
                    "Failed to perform HLS in {crate_name} ({})",
                    crate_path.to_str().unwrap()
                )
                .as_str(),
            );

        let exit_code = output.status.code().unwrap();
        if exit_code != 0 {
            println!(
                "Failed HLS. Keeping directory {}",
                crate_path.to_str().unwrap()
            );
            println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
            println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
            let _keep_tempdir = dir.into_path();
        }
        assert_eq!(exit_code, 0);

        assert!(crate_path.join("result.v").exists());
    }

    #[test]
    fn synthesizing_test_crate_creates_stdout_and_stderr_logs() {
        let dir = TempDir::new().unwrap();
        copy("test_suites/test_crate", dir.path(), &CopyOptions::new()).unwrap();

        let crate_path = dir.path().join("test_crate");
        let function_name = "add";

        generate_llvm_for_tests(&GenerateLlvmOptions {
            function_name: function_name.into(),
            hls_arguments: HlsArguments::default(),
            prepared_crate_root: crate_path.clone(),
        })
        .unwrap();

        let stdout_file = crate_path.join("stdout.log");
        let stderr_file = crate_path.join("stderr.log");
        remove_file(&stdout_file).unwrap();
        remove_file(&stderr_file).unwrap();

        generate_hls_script_file(
            &crate_path,
            &GenerateHlsOptions {
                function_name: function_name.into(),
                hls_arguments: HlsArguments::default(),
            },
        )
        .unwrap();

        let output = Command::new("sh")
            .arg("hls.sh")
            .current_dir(&crate_path)
            .output()
            .unwrap();

        let exit_code = output.status.code().unwrap();
        assert_eq!(exit_code, 0);

        assert!(stdout_file.exists());
        assert!(stderr_file.exists());

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        let stdout_from_file = std::fs::read_to_string(stdout_file).unwrap();
        let stderr_from_file = std::fs::read_to_string(stderr_file).unwrap();

        let path = dir.into_path();
        eprintln!("Keeping directory {:?}", path);

        assert_eq!(stdout, stdout_from_file);
        assert_eq!(stderr, stderr_from_file);
    }
}
