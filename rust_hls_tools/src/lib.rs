//! This crate provides functionality to run commands in an environment with all the rust_hls dependencies.

use docker_command::{Launcher, RunOpt, UserAndGroup, Volume};
use std::{
    ffi::OsString,
    path::{Path, PathBuf},
    process::Output,
};
use thiserror::Error;

const DOCKER_IMAGE: &str = "zebreus/rust_hls_tools:latest";

/// Error type for [RustHlsCommand]
#[derive(Error, Debug)]
pub enum RustHlsCommandError {
    #[error("No container command (docker or podman) found")]
    NoContainerCommandFound(),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    DockerCommandError(#[from] docker_command::command_run::Error),
}

/// A command that will be executed in an environment with all the rust_hls dependencies.
///
/// The command is executed in the [zebreus/rust_hls_tools:latest](https://hub.docker.com/r/zebreus/rust_hls_tools) docker container. The container contains the external tools that are required by rust_hls. This includes PandA bambu and an older version of Verilator. The docker image is built with nix from the definition in [flake.nix](https://github.com/zebreus/rust_hls/blob/master/flake.nix).
///
/// Uses the `docker_command` crate to run the container with docker or podman.
///
/// For now this is only docker, but I should definitly add support for locally installed tools.
pub struct RustHlsCommand {
    program: String,
    args: Vec<String>,
    dirs: Vec<String>,
    working_directory: Option<String>,
}

impl RustHlsCommand {
    /// Create a new command with the given program.
    pub fn new(program: impl Into<String>) -> RustHlsCommand {
        return RustHlsCommand {
            program: program.into(),
            working_directory: None,
            args: Vec::new(),
            dirs: Vec::new(),
        };
    }

    /// Add an argument to the command.
    pub fn arg(&mut self, arg: impl Into<String>) -> &mut Self {
        self.args.push(arg.into());
        self
    }

    /// Set the working directory.
    ///
    /// Will also add the working directory to the acessible directories
    pub fn set_working_directory(&mut self, working_directory: &Path) -> &mut Self {
        let directory_string = working_directory.to_string_lossy().to_string();
        self.working_directory = directory_string.into();
        self.add_directory(working_directory)
    }

    /// Add a directory that should be accessible during command execution
    pub fn add_directory(&mut self, directory: &Path) -> &mut Self {
        let directory_string = directory.to_string_lossy().to_string().into();
        self.dirs.push(directory_string);
        self
    }

    /// Run the command and return the output.
    pub fn output(&mut self) -> Result<Output, RustHlsCommandError> {
        let user = UserAndGroup::current();

        let volumes: Vec<Volume> = self
            .dirs
            .iter()
            .map(|dir| Volume {
                src: PathBuf::from(dir),
                dst: PathBuf::from(dir),
                read_write: true,
                options: vec![],
            })
            .collect();

        let args: Vec<OsString> = self.args.iter().map(|arg| arg.into()).collect();

        let mut command = Launcher::auto()
            .ok_or(RustHlsCommandError::NoContainerCommandFound())?
            .run(RunOpt {
                image: DOCKER_IMAGE.into(),
                command: Some(PathBuf::from(&self.program)),
                args: args,
                user: Some(user),
                volumes,
                init: true,
                remove: true,
                ..Default::default()
            });
        println!("running: {:?}", command);

        let output = command.enable_capture().run()?;

        return Ok(std::process::Output {
            status: output.status,
            stdout: output.stdout,
            stderr: output.stderr,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_run_verilator() {
        let mut command = RustHlsCommand::new("verilator");
        command.arg("--version");
        let output = command.output().unwrap();
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains("Verilator 4.110"));
    }

    #[test]
    fn can_run_bambu() {
        let mut command = RustHlsCommand::new("bambu");
        command.arg("--version");
        let output = command.output().unwrap();
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains("Politecnico di Milano - DEIB"));
    }

    #[test]
    fn can_run_jq() {
        let mut command = RustHlsCommand::new("jq");
        command.arg("--help");
        let output = command.output().unwrap();
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains("jq - commandline JSON processor"));
    }

    #[test]
    fn can_run_llvm_commands() {
        let mut command = RustHlsCommand::new("llvm-dis");
        command.arg("--help");
        let output = command.output().unwrap();
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains("disassembler"));
    }
}
